extern crate rusqlite;
extern crate serial;

use chrono::{DateTime, TimeZone, Utc};
use rusqlite::{params, Connection, Result};
use serial::prelude::*;
use std::time::Duration;

mod greenhouse;

fn main() {
  let db_conn = Connection::open_in_memory().unwrap();
  db_conn
    .execute(
      "CREATE TABLE IF NOT EXISTS ts_data (
    series string NOT NULL,
    epoch_time integer NOT NULL,
    resolution string NOT NULL,
    value integer NOT NULL,
    PRIMARY KEY(series, epoch_time, resolution)
  );",
      params![],
    )
    .unwrap();

  let mut port = serial::open("/dev/ttyACM0").unwrap();
  port.set_timeout(Duration::from_secs(5));

  let mut buffer: Vec<u8> = (0..255).collect();
  let mut line_buffer: Vec<u8> = [].to_vec();

  loop {
    let line = greenhouse::serial::read_line(&mut port, &mut buffer, &mut line_buffer);

    if line.is_none() {
      continue;
    }

    let data = line.unwrap();
    let result = greenhouse::serial::parse_metric(convert_array(&data));

    if result.is_err() {
      println!("{}", result.err().unwrap());
      continue;
    }

    let metric = result.unwrap();

    println!("{:?}", metric);

    db_conn.execute(
      "INSERT INTO ts_data (series, epoch_time, resolution, value) VALUES (?1, ?2, '1s', ?3)",
      params![metric.series, metric.timestamp.timestamp(), metric.value],
    );

    /*let mut statement = db_conn
      .prepare("SELECT series, epoch_time, value FROM ts_data")
      .unwrap();
    let ts_iter = statement
      .query_map(params![], |row| {
        Ok(greenhouse::serial::PlantMetric {
          series: row.get(0)?,
          timestamp: Utc.timestamp(row.get(1).unwrap(), 0),
          value: row.get(2)?,
          pump_status: greenhouse::serial::PumpStatus::Off,
        })
      })
      .unwrap();

    for ts in ts_iter {
      println!("{:?}", ts.unwrap());
    }*/
  }
}

fn convert_array(buffer: &[u8]) -> String {
  let mut output_string = String::new();

  for n in buffer {
    output_string = format!("{}{}", output_string, *n as char);
  }

  return output_string;
}
