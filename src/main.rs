extern crate serial;
extern crate sqlite;

use std::env;
use std::time::Duration;

use serial::prelude::*;

mod greenhouse;

fn main() {
  // let connection = sqlite::open(":memory:").unwrap();
  let mut port = serial::open("/dev/ttyACM0").unwrap();
  port.set_timeout(Duration::from_secs(5));

  /* connection.execute("CREATE TABLE IF NOT EXISTS ts_data (
      device_id string NOT NULL,
      epoch_time integer NOT NULL,
      value float NOT NULL,
      PRIMARY KEY(device_id, epoch_time);
  )").unwrap();*/

  let mut buffer: Vec<u8> = (0..255).collect();
  let mut line_buffer: Vec<u8> = [].to_vec();

  loop {
    let line = greenhouse::serial::read_line(&mut port, &mut buffer, &mut line_buffer);
    match line {
      Some(data) => println!("{}", convert_array(data)),
      None => {}
    }
  }
}

fn convert_array(buffer: Vec<u8>) -> String {
  let mut output_string = String::new();

  for n in buffer {
    output_string = format!("{}{}", output_string, n as char);
  }

  return output_string;
}
