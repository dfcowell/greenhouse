extern crate serial;
extern crate sqlite;

use std::env;
use std::io;
use std::time::Duration;

use std::io::prelude::*;
use serial::prelude::*;

fn main() {
    // let connection = sqlite::open(":memory:").unwrap();
    let mut port = serial::open("/dev/ttyS3").unwrap();
    port.set_timeout(Duration::from_secs(5));

    /* connection.execute("CREATE TABLE IF NOT EXISTS ts_data (
        device_id string NOT NULL,
        epoch_time integer NOT NULL,
        value float NOT NULL,
        PRIMARY KEY(device_id, epoch_time);
    )").unwrap();*/

    let mut buf: Vec<u8> = (0..255).collect();

    let data = port.read(&mut buf[..]);
    println!("{:?}", data);
}
