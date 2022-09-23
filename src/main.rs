#![allow(dead_code)]

use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

use geoutils::Location;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Entry {
    #[serde(rename = "YEAR")]
    time_period: String,

    #[serde(rename = "STATION")]
    station: String,

    #[serde(rename = "Entries 0600-1000")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_morning: Option<i32>,

    #[serde(rename = "Exits 0600-1000")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_morning: Option<i32>,

    #[serde(rename = "Entries 1000-1500")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_midday: Option<i32>,

    #[serde(rename = "Exits 1000-1500")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_midday: Option<i32>,

    #[serde(rename = "Entries 1500-1900")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_evening: Option<i32>,

    #[serde(rename = "Exits 1500-1900")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_evening: Option<i32>,

    #[serde(rename = "Entries 1900 -0600")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_midnight: Option<i32>,

    #[serde(rename = "Exits 1900 -0600")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_midnight: Option<i32>,

    #[serde(rename = "Entries 0000-2359")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_total: Option<i32>,

    #[serde(rename = "Exits 0000-2359")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_total: Option<i32>,

    #[serde(rename = "LAT")]
    latitude: f64,

    #[serde(rename = "LONG")]
    longitude: f64,
}

fn search_station(station: String, entries: &Vec<Entry>) {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut times = [0, 0, 0, 0];
    let mut busy_time: Option<i32> = None;
    let mut timely_max = 0;
    let mut busy_year = 0;
    let mut yearly_max = 0;
    for entry in entries {
        if entry.station == station {
            let len = entry.time_period.len();
            let current_year = entry.time_period[len - 4..]
                .parse::<i32>()
                .expect("Failed to parse year");
            let total = entry.entries_total.unwrap_or(0)
                + entry.exits_total.unwrap_or(0)
                + map.get(&current_year).unwrap_or(&0);
            if total > yearly_max {
                yearly_max = total;
                busy_year = current_year;
            }
            map.insert(current_year, total);
            times[0] += entry.entries_morning.unwrap_or(0) + entry.exits_morning.unwrap_or(0);
            times[1] += entry.entries_midday.unwrap_or(0) + entry.exits_midday.unwrap_or(0);
            times[2] += entry.entries_evening.unwrap_or(0) + entry.exits_evening.unwrap_or(0);
            times[3] += entry.entries_midnight.unwrap_or(0) + entry.exits_midnight.unwrap_or(0);
            for i in 0..4 {
                if times[i] > timely_max {
                    timely_max = times[i];
                    busy_time = Some(i as i32);
                }
            }
        }
    }
    if map.is_empty() {
        println!("No data for station {}", station);
    } else {
        println!(
            "{} is the busiest time of day for {} station",
            match busy_time {
                Some(0) => "morning",
                Some(1) => "midday",
                Some(2) => "evening",
                Some(3) => "midnight",
                _ => "unknown",
            },
            station
        );
        println!("{} is the busiest year for station {}", busy_year, station);
    }
}

/// To create a location, run:
///
/// ```rust
/// let berlin = Location::new(52.518611, 13.408056);
/// ```
///
/// then pass two locations into this function for a
/// distance in meters.
fn distance_in_meters(point1: Location, point2: Location) -> f64 {
    point1.distance_to(&point2).unwrap().meters()
}

fn main() -> Result<(), Box<dyn Error>> {
    // read station name from arg
    let station = std::env::args().nth(1);
    let path = Path::new("trains.csv");

    let entries: Vec<Entry> = csv::Reader::from_path(&path)?
        .deserialize()
        .collect::<Result<_, _>>()?;

    println!("Entries: {entries:?}");
    match station {
        Some(station) => search_station(station, &entries),
        None => {}
    }
    Ok(())
}
