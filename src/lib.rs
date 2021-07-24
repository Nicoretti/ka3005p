#![deny(warnings)]
use anyhow::Context;
use std::fmt;
use std::io;
use std::str;
use std::str::FromStr;
use std::time;

pub mod cli;
pub use serialport;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Switch {
    On,
    Off,
}

impl From<Switch> for bool {
    fn from(w: Switch) -> bool {
        match w {
            Switch::On => true,
            Switch::Off => false,
        }
    }
}

impl std::convert::From<bool> for Switch {
    fn from(x: bool) -> Self {
        if x {
            Switch::On
        } else {
            Switch::Off
        }
    }
}

#[derive(Debug)]
pub enum Command {
    /// Enable/Disable Power
    Power(Switch),
    /// Enable/Disable Beep
    Beep(Switch),
    /// Enable/Disable over voltage protection
    Ovp(Switch),
    /// Enable/Disable over current protection
    Ocp(Switch),
    /// Store current pannel settings to memory
    Save(u32),
    /// Load stored setting into pannel
    Load(u32),
    /// Sets the voltage
    Voltage(f32),
    /// Sets the current
    Current(f32),
}

#[derive(Debug, PartialEq)]
pub struct Flags {
    flags: u8,
    pub channel1: Mode,
    pub channel2: Mode,
    pub beep: Switch,
    pub lock: Lock,
    pub output: Switch,
}

#[derive(Debug, PartialEq)]
pub enum Channel {
    One,
    Two,
}

#[derive(Debug, PartialEq)]
pub enum Lock {
    Locked,
    Unlocked,
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Cc,
    Cv,
}

pub struct Status {
    pub flags: Flags,
    pub voltage: f32,
    pub current: f32,
}

impl std::str::FromStr for Switch {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "on" => Ok(Switch::On),
            "off" => Ok(Switch::Off),
            _ => Err(anyhow::anyhow!("Value must be either 'on' or 'off'")),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Voltage: {:.2}, Current: {:.3}, Channel1: {:?}, Channel2: {:?} Lock: {:?}, Beep: {:?}, Output: {:?}",
            self.voltage,
            self.current,
            self.flags.channel1,
            self.flags.channel2,
            self.flags.lock,
            self.flags.beep,
            self.flags.output,
        )
    }
}

impl Status {
    pub fn new(flags: Flags, voltage: f32, current: f32) -> Self {
        Status {
            flags,
            voltage,
            current,
        }
    }
}

impl Flags {
    pub fn new(flags: u8) -> Self {
        let channel1 = if flags & 0x01 != 0 {
            Mode::Cv
        } else {
            Mode::Cc
        };
        let channel2 = if flags & 0x02 != 0 {
            Mode::Cv
        } else {
            Mode::Cc
        };
        let beep = if flags & 0x10 != 0 {
            Switch::On
        } else {
            Switch::Off
        };
        let lock = if flags & 0x20 != 0 {
            Lock::Locked
        } else {
            Lock::Unlocked
        };
        let output = if flags & 0x40 != 0 {
            Switch::On
        } else {
            Switch::Off
        };
        Flags {
            flags,
            channel1,
            channel2,
            beep,
            lock,
            output,
        }
    }
}

pub fn list_serial_ports() -> Vec<serialport::SerialPortInfo> {
    let serial_devices: Vec<serialport::SerialPortInfo> = serialport::available_ports()
        .unwrap()
        .into_iter()
        .filter(|info| match &info.port_type {
            serialport::SerialPortType::UsbPort(usb) => usb.vid == 1046,
            _ => false,
        })
        .collect();
    serial_devices
}

pub fn find_serial_port() -> anyhow::Result<Ka3005p> {
    let serial_devices = list_serial_ports();

    match serial_devices.len() {
        0 => Err(anyhow::anyhow!("No Power Supply Found!")),
        1 => Ka3005p::new(&serial_devices[0].port_name),
        _ => Err(anyhow::anyhow!("Multiple Power Supplies Found!")),
    }
}

impl std::convert::From<Command> for String {
    fn from(c: Command) -> Self {
        match c {
            Command::Power(s) => match s {
                Switch::On => String::from("OUT1"),
                Switch::Off => String::from("OUT0"),
            },
            Command::Ovp(s) => match s {
                Switch::On => String::from("OVP1"),
                Switch::Off => String::from("OVP0"),
            },
            Command::Ocp(s) => match s {
                Switch::On => String::from("OCP1"),
                Switch::Off => String::from("OCP0"),
            },
            Command::Beep(s) => match s {
                Switch::On => String::from("BEEP1"),
                Switch::Off => String::from("BEEP0"),
            },
            Command::Save(id) => format!("SAV{}", id),
            Command::Load(id) => format!("RCL{}", id),
            Command::Voltage(v) => format!("VSET1:{:.2}", v),
            Command::Current(i) => format!("ISET1:{:.3}", i),
        }
    }
}

pub struct Ka3005p {
    serial: Box<dyn serialport::SerialPort>,
}

impl Ka3005p {
    pub fn new(port_name: &str) -> anyhow::Result<Self> {
        let serial = serialport::new(port_name, 9600)
            .timeout(time::Duration::from_millis(60))
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::One)
            .open()?;

        Ok(Ka3005p { serial })
    }

    /// If the user wishes to setup the serial port themselves
    pub fn new_from_serial(serial: Box<dyn serialport::SerialPort>) -> anyhow::Result<Self> {
        Ok(Ka3005p { serial })
    }

    pub fn execute(&mut self, command: Command) -> anyhow::Result<()> {
        self.run_command(&String::from(command))?;
        Ok(())
    }

    /// Retrieve status information from the power supply
    pub fn status(&mut self) -> anyhow::Result<Status> {
        let flags = self.run_command_response("STATUS?")?;
        let flags = Flags::new(flags[0]);
        let voltage = f32::from_str(
            String::from_utf8_lossy(self.run_command_response("VOUT1?")?.as_ref())
                .into_owned()
                .as_str(),
        )?;
        let current = f32::from_str(
            String::from_utf8_lossy(self.run_command_response("IOUT1?")?.as_ref())
                .into_owned()
                .as_str(),
        )?;
        Ok(Status::new(flags, voltage, current))
    }

    fn run_command_response(&mut self, command: &str) -> anyhow::Result<Vec<u8>> {
        let res = self.run_command(command)?;
        anyhow::ensure!(!res.is_empty(), "PSU did not respond with data");
        Ok(res)
    }

    fn run_command(&mut self, command: &str) -> anyhow::Result<Vec<u8>> {
        let bytes = command.as_bytes();
        if !self.serial.write(bytes)? == bytes.len() {
            return Err(anyhow::anyhow!("Could not write command"));
        }
        self.serial.flush()?;
        let mut result: Vec<u8> = Vec::new();
        let mut is_done = false;
        while !is_done {
            let mut serial_buf: Vec<u8> = vec![0; 512];
            match self.serial.read(serial_buf.as_mut_slice()) {
                Ok(count) => {
                    result.extend(serial_buf.drain(..count));
                }
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                    is_done = true;
                }
                Err(e) => {
                    return Err(e).with_context(|| "could not retrieve response from power supply")
                }
            };
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel1_status() {
        assert_eq!(Mode::Cc, Flags::new(0).channel1);
        assert_eq!(Mode::Cv, Flags::new(1).channel1);
    }

    #[test]
    fn test_channel2_status() {
        assert_eq!(Mode::Cc, Flags::new(0).channel2);
        assert_eq!(Mode::Cv, Flags::new(2).channel2);
    }

    #[test]
    fn test_beep_status() {
        assert_eq!(Switch::Off, Flags::new(0).beep);
        assert_eq!(Switch::On, Flags::new(16).beep);
    }

    #[test]
    fn test_lock_status() {
        assert_eq!(Lock::Unlocked, Flags::new(0).lock);
        assert_eq!(Lock::Locked, Flags::new(32).lock);
    }

    #[test]
    fn test_output_status() {
        assert_eq!(Switch::Off, Flags::new(0).output);
        assert_eq!(Switch::On, Flags::new(64).output);
    }

    #[test]
    fn test_output_vset() {
        // PSU is picky on the number of decimal places.
        assert_eq!(
            String::from(Command::Voltage(3.123)),
            "VSET1:3.12".to_string()
        );
        assert_eq!(
            String::from(Command::Voltage(1.500)),
            "VSET1:1.50".to_string()
        );
        assert_eq!(
            String::from(Command::Voltage(4.999)),
            "VSET1:5.00".to_string()
        );
        assert_eq!(
            String::from(Command::Voltage(4.0)),
            "VSET1:4.00".to_string()
        );
    }

    #[test]
    fn test_output_iset() {
        // PSU is picky on the number of decimal places.
        assert_eq!(
            String::from(Command::Current(3.123)),
            "ISET1:3.123".to_string()
        );
        assert_eq!(
            String::from(Command::Current(1.500)),
            "ISET1:1.500".to_string()
        );
        assert_eq!(
            String::from(Command::Current(4.99999)),
            "ISET1:5.000".to_string()
        );
        assert_eq!(
            String::from(Command::Current(4.0)),
            "ISET1:4.000".to_string()
        );
    }
}
