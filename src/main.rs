extern crate serial;
extern crate sqlite;

use std::env;
use std::io;
use std::time::Duration;

use std::io::prelude::*;
use serial::prelude::*;

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

    let newline: u8 = 10;

    let mut buf: Vec<u8> = (0..255).collect();
    let mut line_buffer: Vec<u8> = [].to_vec();

    loop {
        let read_result = port.read(&mut buf[..]);

        match read_result {
            Ok(data_length) => {
                let working = buf[0..data_length].to_vec();

                if working.contains(&newline) {
                    let mut iter = working.iter();
                    let index = iter.position(|&x| x == newline).unwrap();

                    let output = [line_buffer, working[0..index].to_vec()].concat();
                    let length = output.len();
                    println!("{}", convert_array(&output, length));
                    line_buffer = working[index+1..data_length].to_vec();
                } else {
                    line_buffer = [line_buffer, working].concat();
                }
            },
            Err(error) => {
                println!("Error reading from port");
            }
        }
    }
}

fn convert_array(buffer: &[u8], length: usize) -> String {
    let mut output_string = String::new();

    for n in 0..length {
        output_string = format!("{}{}", output_string, buffer[n] as char);
    }

    return output_string;
}