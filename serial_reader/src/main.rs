/*!
This program only prints each line that it encounters in the serial data stream from the Arduino.
*/

extern crate serialport;

use std::io::Read;
use std::path::Path;
use std::time::Duration;

use serialport::{DataBits, FlowControl, Parity, SerialPortSettings, StopBits};

const ASCII_NEW_LINE: u8 = 10;

fn main() {
    // The Arduino serial sends to the /dev/ttyACM0 port.
    let mut port = serialport::posix::TTYPort::open(&Path::new("/dev/ttyACM0"), &SerialPortSettings {
        baud_rate: 115_200,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(10),
    }).expect("Failed to open port");

    // We read each byte individually and clear the buffer once we encounter a newline
    let mut line = Vec::with_capacity(28);
    let mut serial_buf: Vec<u8> = vec![0; 1];
    loop {
        if port.read(&mut serial_buf).is_ok() {
            line.push(serial_buf[0]);
            if serial_buf[0] == ASCII_NEW_LINE {
                if let Ok(line) = std::str::from_utf8(&line) {
                    print!("{}", line);
                }
                line.clear();
            }
        }
    }
}