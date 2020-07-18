use serialport;
use std::io;
use std::io::{Read, Write};
use std::str;
use std::str::{FromStr, ParseBoolError};
use std::{thread, time};
use structopt::StructOpt;

mod cli {

    use std::path::PathBuf;

    #[derive(Debug)]
    pub enum Switch {
        On,
        Off,
    }

    impl std::str::FromStr for Switch {
        type Err = String;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().as_ref() {
                "on" => Ok(Switch::On),
                "off" => Ok(Switch::Off),
                _ => Err(String::from("Failed")),
            }
        }
    }

    #[derive(structopt::StructOpt, Debug)]
    #[structopt(about = "Remote controls a KA3000 power supply")]
    #[structopt(global_settings(& [structopt::clap::AppSettings::ColoredHelp]))]
    pub struct PowerSupply {
        #[structopt(name = "switch")]
        #[structopt(help = "enable or disable the power supply")]
        pub switch: Switch,
    }
}

fn run_command(serial: &mut Box<serialport::SerialPort>, command: &str) -> String {
    serial.write(command.as_bytes()).unwrap();
    serial.flush().unwrap();
    let mut result: String = String::from("");
    loop {
        let mut serial_buf: Vec<u8> = vec![0; 1000];
        let r = serial.read(serial_buf.as_mut_slice());
        match r {
            Ok(t) => {
                result.push_str(&str::from_utf8(&serial_buf.as_slice()[..t]).unwrap());
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                break;
            }
            Err(e) => eprintln!("Error {:?}", e),
        }
    }

    return result;
}

fn main() -> io::Result<()> {
    let args = cli::PowerSupply::from_args();

    let serial_devices: Vec<serialport::SerialPortInfo> = serialport::available_ports()?
        .into_iter()
        .filter(|info| match &info.port_type {
            serialport::SerialPortType::UsbPort(usb) => usb.vid == 1046,
            _ => false,
        })
        .collect();

    match serial_devices.len() {
        0 => {
            eprintln!("No Power Supply Found!");
            std::process::exit(1);
        }
        1 => {
            let mut serial = serialport::open(&serial_devices[0].port_name).unwrap();

            serial.set_timeout(time::Duration::from_millis(50)).unwrap();
            serial.set_baud_rate(9600).unwrap();
            serial.set_parity(serialport::Parity::None).unwrap();
            serial.set_stop_bits(serialport::StopBits::One).unwrap();

            match args.switch {
                cli::Switch::On => run_command(&mut serial, "OUT1"),
                cli::Switch::Off => run_command(&mut serial, "OUT0"),
            };

            std::process::exit(1);
        }
        _ => {
            eprintln!("Multiple Power Supplies Found!");
            std::process::exit(1);
        }
    }
}
