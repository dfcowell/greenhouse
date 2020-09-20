extern crate serial;

use std::io::Read;

const NEW_LINE: u8 = 10;

pub fn read_line(
  port: &mut serial::SystemPort,
  buffer: &mut Vec<u8>,
  line_buffer: &mut Vec<u8>,
) -> Option<Vec<u8>> {
  let read_result = port.read(&mut buffer[..]);

  match read_result {
    Ok(data_length) => {
      let new_data = buffer[0..data_length].to_vec();

      if new_data.contains(&NEW_LINE) {
        let mut iter = new_data.iter();
        let index = iter.position(|&x| x == NEW_LINE).unwrap();
        let current_line = new_data[0..index].to_vec();
        let next_line = new_data[index + 1..data_length].to_vec();

        let mut output: Vec<u8> = [].to_vec();
        output.clone_from(line_buffer);
        output.extend_from_slice(&current_line);

        line_buffer.truncate(0);
        line_buffer.extend_from_slice(&next_line);

        return Some(output);
      } else {
        line_buffer.extend_from_slice(&new_data);
        return None;
      }
    }
    Err(error) => {
      println!("Error reading from port");
      return None;
    }
  }
}
