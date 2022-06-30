use std::io;
use std::path::Path;

use clap::Parser;
use comfy_table::*;
use date_nager_at::apis::configuration::Configuration;
use date_nager_at::apis::public_holiday_api::public_holiday_next_public_holidays;
use date_nager_at::apis::public_holiday_api::PublicHolidayNextPublicHolidaysError;

#[macro_use]
extern crate log;

mod cache;
use cache::CachePath;
use cache::JsonCache;
use date_nager_at::models::PublicHolidayV3Dto;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, value_parser)]
    country: String,
    #[clap(
        short = 's',
        long,
        value_parser,
        required(true),
        long_help = "Full path to cache api results"
    )]
    cache: String,
}

#[derive(Debug)]
pub enum Error<T> {
    ApiError(date_nager_at::apis::Error<T>),
    DiskError(io::Error),
    UnknownError(String),
}

async fn populate(
    args: Args,
    cfg: Configuration,
    holidays: &mut Vec<PublicHolidayV3Dto>,
) -> Result<(), Error<PublicHolidayNextPublicHolidaysError>> {
    let cache: &Path = args.cache.as_ref();
    let key = args.country.as_ref();
    let keypath: CachePath = <Vec<PublicHolidayV3Dto> as JsonCache>::path(cache, key);
    if holidays.expired(cache, key) {
        trace!("cache has expired");
        let result = public_holiday_next_public_holidays(&cfg, &args.country).await;
        match result {
            Err(e) => {
                error!("Unable to get result from API.");
                return Err(Error::ApiError(e));
            }
            Ok(result) => {
                match result.save(keypath.clone(), true) {
                    Err(e) => {
                        //TODO: may be still show the result ?
                        error!("Unable to save cache.");
                        return Err(Error::DiskError(e));
                    }
                    Ok(_) => {}
                };
            }
        };
    } else {
        trace!("reusing cache");
    }
    if matches!(holidays.reload(keypath.clone()), Err(_)) {
        error!(
            "Unable to reload cache. Try deleting {} file",
            keypath.clone().display()
        );
        return Err(Error::UnknownError(
            "Something went wrong while reloading cache".to_string(),
        ));
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("starting up");

    let args = Args::parse();
    let path: &Path = args.cache.as_ref();

    if !path.exists() {
        error!("Cache path does not exists");
        return;
    }

    let mut cfg = Configuration::new();
    cfg.base_path = "https://date.nager.at".to_string();

    let mut holidays: Vec<PublicHolidayV3Dto> = vec![];
    //     let path = cache_path(args.cache.as_ref(), args.country.as_ref());
    trace!("populating holidays");
    match populate(args, cfg, &mut holidays).await {
        Err(e) => {
            error!("{:?}", e);
        }
        Ok(_) => {
            let mut table = Table::new();
            table.set_header(vec!["Date", "Name", "Counties", "Types"]);

            trace!("Iterating holidays for table");
            holidays.iter().for_each(|day| {
                let mut row: Vec<String> = vec![];
                row.push(day.date.clone().unwrap_or("".to_string()));
                row.push(day.name.clone().unwrap_or("".to_string()));
                row.push(
                    day.counties
                        .as_ref()
                        .unwrap_or(&vec![])
                        .as_slice()
                        .join(","),
                );
                row.push(
                    day.types
                        .as_ref()
                        .unwrap_or(&vec![])
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .as_slice()
                        .join(","),
                );
                table.add_row(row);
            });

            println!("{table}");
        }
    }
    trace!("DONE");
}
