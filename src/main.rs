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

    let mut buf: Vec<u8> = (0..255).collect();

    while let dataLength = port.read(&mut buf[..]) {
        let mut myVec = Vec::new();
        myVec.clone_from(&buf);
        print!("{}", convertArray(myVec, dataLength.unwrap()));
    }
}

fn convertArray(buffer: Vec<u8>, length: usize) -> String {
    let mut outputString = String::new();

    for n in 0..length {
        outputString = format!("{}{}", outputString, buffer[n] as char);
    }

    return outputString;
}