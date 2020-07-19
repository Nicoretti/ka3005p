use std::fmt;
use std::io;
use std::str;
use std::time;

#[derive(Debug, PartialEq)]
pub struct Current {
    a: u32,
    ma: u32,
}

#[derive(Debug, PartialEq)]
pub struct Voltage {
    v: u32,
    mv: u32,
}

#[derive(Debug, PartialEq)]
pub enum Switch {
    On,
    Off,
}

#[derive(Debug)]
pub enum Command {
    Status,
    Beep,
    Power,
    Output,
    OverVoltageProtection,
    OverCurrentProtection,
    Save,
    Load,
    Identification,
}

#[derive(Debug, PartialEq)]
pub struct Status {
    _raw: u8,
}

#[derive(Debug, PartialEq)]
pub enum Channel {
    _1,
    _2,
}

#[derive(Debug, PartialEq)]
pub enum Lock {
    Locked,
    Unlocked,
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    CC,
    CV,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Channel1: {:?}, Channel2: {:?} Lock: {:?}, Beep: {:?}, Output: {:?}",
            self.mode(Channel::_1),
            self.mode(Channel::_2),
            self.lock(),
            self.beep(),
            self.output(),
        )
    }
}

impl std::str::FromStr for Voltage {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.trim().to_lowercase();
        let parts: Vec<&str> = normalized.split(".").collect();
        let v = parts[0].parse::<u32>()?;
        let mv = parts[1].parse::<u32>()?;
        Ok(Voltage { v, mv })
    }
}

impl std::str::FromStr for Current {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.trim().to_lowercase();
        let parts: Vec<&str> = normalized.split(".").collect();
        let a = parts[0].parse::<u32>()?;
        let ma = parts[1].parse::<u32>()?;
        Ok(Current { a, ma })
    }
}

impl Status {
    pub fn new(status: u8) -> Self {
        Status { _raw: status }
    }

    pub fn mode(&self, channel: Channel) -> Mode {
        let bitmask = match channel {
            Channel::_1 => 1,
            Channel::_2 => 2,
        };
        match (self._raw & bitmask) == 0 {
            true => Mode::CC,
            false => Mode::CV,
        }
    }

    pub fn beep(&self) -> Switch {
        let bitmask = 16;
        match (self._raw & bitmask) != 0 {
            true => Switch::On,
            false => Switch::Off,
        }
    }

    pub fn lock(&self) -> Lock {
        let bitmask = 32;
        match (self._raw & bitmask) != 0 {
            true => Lock::Locked,
            false => Lock::Unlocked,
        }
    }

    pub fn output(&self) -> Switch {
        let bitmask = 64;
        match (self._raw & bitmask) != 0 {
            true => Switch::On,
            false => Switch::Off,
        }
    }
}

pub fn find_serial_port() -> Result<Box<dyn serialport::SerialPort>, String> {
    let serial_devices: Vec<serialport::SerialPortInfo> = serialport::available_ports()
        .unwrap()
        .into_iter()
        .filter(|info| match &info.port_type {
            serialport::SerialPortType::UsbPort(usb) => usb.vid == 1046,
            _ => false,
        })
        .collect();

    match serial_devices.len() {
        0 => Err(String::from("No Power Supply Found!")),
        1 => {
            let mut serial = serialport::open(&serial_devices[0].port_name).unwrap();
            serial.set_timeout(time::Duration::from_millis(50)).unwrap();
            serial.set_baud_rate(9600).unwrap();
            serial.set_parity(serialport::Parity::None).unwrap();
            serial.set_stop_bits(serialport::StopBits::One).unwrap();
            Ok(serial)
        }
        _ => Err(String::from("Multiple Power Supplies Found!")),
    }
}

pub fn run_command(serial: &mut Box<dyn serialport::SerialPort>, command: &str) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel1_status() {
        assert_eq!(Mode::CC, Status::new(0).mode(Channel::_1));
        assert_eq!(Mode::CV, Status::new(1).mode(Channel::_1));
    }

    #[test]
    fn test_channel2_status() {
        assert_eq!(Mode::CC, Status::new(0).mode(Channel::_2));
        assert_eq!(Mode::CV, Status::new(2).mode(Channel::_2));
    }

    #[test]
    fn test_beep_status() {
        assert_eq!(Switch::Off, Status::new(0).beep());
        assert_eq!(Switch::On, Status::new(16).beep());
    }

    #[test]
    fn test_lock_status() {
        assert_eq!(Lock::Unlocked, Status::new(0).lock());
        assert_eq!(Lock::Locked, Status::new(32).lock());
    }

    #[test]
    fn test_output_status() {
        assert_eq!(Switch::Off, Status::new(0).output());
        assert_eq!(Switch::On, Status::new(64).output());
    }
    #[test]
    fn test_voltage_from_str() {
        assert_eq!(Voltage { v: 10, mv: 0 }, "10.0".parse::<Voltage>().unwrap());
        assert_eq!(Voltage { v: 1, mv: 9 }, "1.9".parse::<Voltage>().unwrap());
        assert_eq!(Voltage { v: 0, mv: 9 }, "0.9".parse::<Voltage>().unwrap());
    }

    #[test]
    fn test_current_from_str() {
        assert_eq!(Current { a: 10, ma: 0 }, "10.0".parse::<Current>().unwrap());
        assert_eq!(Current { a: 1, ma: 9 }, "1.9".parse::<Current>().unwrap());
        assert_eq!(Current { a: 0, ma: 9 }, "0.9".parse::<Current>().unwrap());
    }
}
