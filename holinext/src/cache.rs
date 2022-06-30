use date_nager_at::models::PublicHolidayV3Dto;
use std::{
    collections::HashSet,
    fs::File,
    io::{self, Read, Seek, Write},
    path::{Path, PathBuf},
    time::Duration,
};

pub type CachePath = PathBuf;

pub trait CacheIO {
    fn read_to_string(&self, contents: &mut String, create: bool) -> io::Result<()>;
    fn write_to_path(&self, content: &String) -> io::Result<()>;
}

impl CacheIO for CachePath {
    fn read_to_string(&self, contents: &mut String, create: bool) -> io::Result<()> {
        if !create && !self.exists() {
            return Ok(());
        }
        let mut file = File::options().read(true).create(create).open(self)?;
        trace!("Cache file opened for reading");

        file.read_to_string(contents)?;
        trace!("file contents read");
        Ok(())
    }

    fn write_to_path(&self, content: &String) -> io::Result<()> {
        let mut file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(self)?;
        file.seek(io::SeekFrom::Start(0))?;
        file.write_all(content.as_bytes())?;
        trace!("wrote content");
        Ok(())
    }
}

pub trait JsonCache {
    fn save(&self, path: impl CacheIO, create: bool) -> io::Result<()>;
    fn reload(&mut self, path: impl CacheIO) -> io::Result<()>;

    fn path<'a>(cache: &'a Path, key: &'a str) -> PathBuf {
        cache.join(key)
    }

    fn expired(&self, cache: &Path, key: &str) -> bool {
        // This is OS dependant
        // TODO: FS level support needs to be abstracted out
        // either
        // - cache data has a date stated with it
        // - or extra key file is stored in cache
        <Self as JsonCache>::path(cache, key)
            .metadata()
            .map_or(true, |meta| {
                meta.accessed().map_or(true, |time| {
                    time.elapsed()
                        .map_or(true, |dur| dur > Duration::from_secs(24 * 60 * 60))
                })
            })
    }
}

impl JsonCache for Vec<PublicHolidayV3Dto> {
    fn save(&self, path: impl CacheIO, create: bool) -> io::Result<()> {
        trace!("Cache file will be opened");
        let mut contents = String::new();
        // path might not exist and thus opening might fail, and thats okay
        let _ = path.read_to_string(&mut contents, create);
        trace!("file contents read");

        let mut existing: Self = if contents.len() == 0 {
            vec![]
        } else {
            serde_json::from_str(&contents)?
        };

        trace!("deserialized json structre");

        existing.extend_from_slice(&self[..]);
        let mut hs: HashSet<String> = HashSet::new();
        // TODO: assume day.date is always there
        // TODO: using the newtype pattern implement Hash trait for PublicHolidayV3Dto
        // this allows for better comparison
        // drop holidays without date as these could never be dropped from cache
        existing.retain_mut(|day| {
            day.date.is_some() && hs.insert(day.date.as_ref().unwrap().to_string())
        });

        trace!("extened existing content");

        //  TODO : https://github.com/serde-rs/json/issues/160
        //  serde_json::to_writer(file);
        let text = serde_json::to_string(&existing)?;
        path.write_to_path(&text)
    }

    fn reload(&mut self, path: impl CacheIO) -> io::Result<()> {
        let mut contents = String::new();
        trace!("Cache file opened for reloading");
        path.read_to_string(&mut contents, false)?;

        let existing: Self = if contents.len() == 0 {
            vec![]
        } else {
            serde_json::from_str(&contents)?
        };
        trace!("will be extending from slice");
        self.extend_from_slice(&existing[..]);
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use std::{cell::RefCell, default::Default};

    use super::*;

    #[derive(Debug, Clone)]
    struct DummyFile(String);

    impl CacheIO for std::rc::Rc<std::cell::RefCell<DummyFile>> {
        fn read_to_string(&self, contents: &mut String, _create: bool) -> io::Result<()> {
            let dummy = self.borrow();
            contents.clone_from(&dummy.0);
            Ok(())
        }

        fn write_to_path(&self, content: &String) -> io::Result<()> {
            let mut dummy = self.borrow_mut();
            dummy.0.clone_from(content);
            dbg!(dummy);
            Ok(())
        }
    }

    #[test]
    fn test_save_reload() {
        let mut days: Vec<PublicHolidayV3Dto> = vec![];
        let mut day: PublicHolidayV3Dto = Default::default();
        day.local_name = Some("Dummy".to_string());
        day.date = Some("2022-01-01".to_string());
        days.push(day.clone());
        let dummy = std::rc::Rc::new(RefCell::new(DummyFile(String::new())));
        let _ = days.save(dummy.clone(), true);
        let mut days2: Vec<PublicHolidayV3Dto> = vec![];
        let _ = days2.reload(dummy.clone());
        let day2 = days2.pop().unwrap();
        assert_eq!(day, day2);
    }
}
