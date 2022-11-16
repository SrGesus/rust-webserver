use warp::multipart::{FormData, Part};
use futures::TryStreamExt;
use warp::Buf;

use std::collections::HashMap;
use std::convert::Infallible;
use std::io::Read;
use serde_json;

use influxdb::{Client, Query, Timestamp, ReadQuery};
use influxdb::InfluxDbWriteable;
use chrono::{NaiveTime, NaiveDateTime, Utc, DateTime};

use crate::db::Db;
use crate::cfg::BUFFER_SIZE;



pub async fn get_data(
    db: Db
) -> Result<impl warp::Reply, Infallible> {
    let db = db.lock().await;
    
    let json = serde_json::to_string_pretty(&db.clone()).unwrap();

    println!("/get_data GET request received sucessfully");
    Ok(json)
}

// Receives an HashMap and pushes it into the Db
pub async fn put_data_hash (
    mut values: HashMap<String, String>,
    db: Db
) -> Result<impl warp::Reply, warp::Rejection> {
    
    if let Some(time) = values.get_mut("Time") {

        // If time is not rfc3339 formatted try HH:MM:SS and add today's date and UTC TimeZone
        if let Err(_) = DateTime::parse_from_rfc3339(time) {
            if let Ok(time) = NaiveTime::parse_from_str(time, "%H:%M:%S%.f") {
                let date = Utc::now().date_naive();
                let datetime = NaiveDateTime::new(date, time);
                values.insert(String::from("Time"), DateTime::<Utc>::from_utc(datetime, Utc).to_string());      
            } else {
                return Ok(format!("Unsuported format for Time: {}\nPlease use rfc3339 or HH:MM:SS", time));
            }
        } 

    // If there's no time parameter add it
    } else {
        values.insert(String::from("Time"), Utc::now().to_rfc3339());
    }

    // Await for database lock
    let mut db = db.lock().await;

    for (key, value) in values.iter() {
        if let Some(vec) = db.get_mut(key) {
            vec.push(value.to_owned());
        } else {
            db.insert(key.to_owned(), vec![value.to_owned()]);
        }
    };

    println!("/put_data POST request received sucessfully");

    // Shouldn't be able to panic because it receives a Hashmap<String, String>
    Ok(serde_json::to_string_pretty(&values).unwrap())
}

// Extracts a HashMap from multipart/form-data and pushes it into the db
pub async fn put_data_formdata(
    values: FormData,
    db: Db
) -> Result<impl warp::Reply, warp::Rejection> {

    // Register time before awaiting
    let utc_time = Utc::now();

    println!("Processing multipart/form-data request");

    // Collect all the parts of the multiform data and wait for the futures
    let mut parts: Vec<Part> = values.try_collect().await.map_err(|err| {
        eprintln!("form error: {}", err);
        warp::reject::reject()
    })?;

    // Extract the data from the FormData and turn it into a HashMap
    let mut hash_map: HashMap<String, String> = HashMap::new();

    // If data fails to collect for a measurement it does not get added into the
    for part in parts.iter_mut() {
        let name = String::from(part.name());
        if let Ok(buf) = part.data().await.unwrap() {
            let mut reader = buf.reader();
            let mut dst = [0; BUFFER_SIZE];

            let len = reader.read(&mut dst).unwrap();
            let data = String::from_utf8_lossy(&dst[0..len]).into_owned();

            //println!("{} = {}", name, data);
            hash_map.insert(name, data);
        } else {
            println!("form-data collection failed for {}", part.name());
        }
        
    }

    // If there's no Time parameter add it to the HashMap
    if let None = hash_map.get("Time") {
        hash_map.insert(String::from("Time"), utc_time.to_rfc3339());
    }

    // Push into the Db and get the added data
    put_data_hash(hash_map, db).await
}