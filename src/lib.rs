#![deny(warnings)]
use anyhow::Context;
use std::fmt;
use std::io;
use std::str;
use std::str::FromStr;
use std::time;

pub mod cli;

#[derive(Debug, PartialEq, Clone, Copy)]
/// Current
pub struct I {
    ampere: u32,
    milli_ampere: u32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
/// Voltage
pub struct V {
    volts: u32,
    milli_volts: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Switch {
    On,
    Off,
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
    Voltage(V),
    /// Sets the current
    Current(I),
}

#[derive(Debug, PartialEq)]
pub struct Flags {
    flags: u8,
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

pub struct Status {
    flags: Flags,
    voltage: V,
    current: I,
}

impl I {
    pub fn new(ampere: u32, milli_ampere: u32) -> Self {
        I {
            ampere,
            milli_ampere,
        }
    }
}

impl V {
    pub fn new(volts: u32, milli_volts: u32) -> Self {
        V { volts, milli_volts }
    }
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

impl fmt::Display for V {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{} V", self.volts, self.milli_volts)
    }
}

impl fmt::Display for I {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{} A", self.ampere, self.milli_ampere)
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Voltage: {}, Current: {}, Channel1: {:?}, Channel2: {:?} Lock: {:?}, Beep: {:?}, Output: {:?}",
            self.voltage,
            self.current,
            self.flags.mode(Channel::_1),
            self.flags.mode(Channel::_2),
            self.flags.lock(),
            self.flags.beep(),
            self.flags.output(),
        )
    }
}

impl std::str::FromStr for V {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.trim().to_lowercase();
        let parts: Vec<&str> = normalized.split('.').collect();
        let v = parts[0].parse::<u32>()?;
        let mv = parts[1].parse::<u32>()?;
        Ok(V::new(v, mv))
    }
}

impl std::str::FromStr for I {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.trim().to_lowercase();
        let parts: Vec<&str> = normalized.split('.').collect();
        let a = parts[0].parse::<u32>()?;
        let ma = parts[1].parse::<u32>()?;
        Ok(I::new(a, ma))
    }
}

impl Status {
    pub fn new(flags: Flags, voltage: V, current: I) -> Self {
        Status {
            flags,
            voltage,
            current,
        }
    }
}

impl Flags {
    pub fn new(flags: u8) -> Self {
        Flags { flags }
    }

    pub fn mode(&self, channel: Channel) -> Mode {
        let bitmask = match channel {
            Channel::_1 => 1,
            Channel::_2 => 2,
        };
        if (self.flags & bitmask) == 0 {
            Mode::CC
        } else {
            Mode::CV
        }
    }

    pub fn beep(&self) -> Switch {
        let bitmask = 16;
        if (self.flags & bitmask) != 0 {
            Switch::On
        } else {
            Switch::Off
        }
    }

    pub fn lock(&self) -> Lock {
        let bitmask = 32;
        if (self.flags & bitmask) != 0 {
            Lock::Locked
        } else {
            Lock::Unlocked
        }
    }

    pub fn output(&self) -> Switch {
        let bitmask = 64;
        if (self.flags & bitmask) != 0 {
            Switch::On
        } else {
            Switch::Off
        }
    }
}

pub fn find_serial_port() -> anyhow::Result<Box<dyn serialport::SerialPort>> {
    let serial_devices: Vec<serialport::SerialPortInfo> = serialport::available_ports()
        .unwrap()
        .into_iter()
        .filter(|info| match &info.port_type {
            serialport::SerialPortType::UsbPort(usb) => usb.vid == 1046,
            _ => false,
        })
        .collect();

    match serial_devices.len() {
        0 => Err(anyhow::anyhow!("No Power Supply Found!")),
        1 => {
            let mut serial = serialport::open(&serial_devices[0].port_name).unwrap();
            serial.set_timeout(time::Duration::from_millis(50)).unwrap();
            serial.set_baud_rate(9600).unwrap();
            serial.set_parity(serialport::Parity::None).unwrap();
            serial.set_stop_bits(serialport::StopBits::One).unwrap();
            Ok(serial)
        }
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
            Command::Voltage(v) => format!("VSET1:{}.{}", v.volts, v.milli_volts),
            Command::Current(i) => format!("ISET1:{}.{}", i.ampere, i.milli_ampere),
        }
    }
}

pub fn execute(serial: &mut dyn serialport::SerialPort, command: Command) -> anyhow::Result<()> {
    run_command(serial, &String::from(command))?;
    Ok(())
}

/// Retrieve status information from the power supply
pub fn status(serial: &mut dyn serialport::SerialPort) -> anyhow::Result<Status> {
    let flags = Flags::new(run_command(serial, "STATUS?")?[0]);
    let voltage = V::from_str(
        String::from_utf8_lossy(&run_command(serial, "VOUT1?")?)
            .into_owned()
            .as_str(),
    )?;
    let current = I::from_str(
        String::from_utf8_lossy(&run_command(serial, "IOUT1?")?)
            .into_owned()
            .as_str(),
    )?;
    Ok(Status::new(flags, voltage, current))
}

fn run_command(serial: &mut dyn serialport::SerialPort, command: &str) -> anyhow::Result<Vec<u8>> {
    let bytes = command.as_bytes();
    if !serial.write(bytes)? == bytes.len() {
        return Err(anyhow::anyhow!("Could not write command"));
    }
    serial.flush()?;
    let mut result: Vec<u8> = Vec::new();
    let mut is_done = false;
    while !is_done {
        let mut serial_buf: Vec<u8> = vec![0; 512];
        match serial.read(serial_buf.as_mut_slice()) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel1_status() {
        assert_eq!(Mode::CC, Flags::new(0).mode(Channel::_1));
        assert_eq!(Mode::CV, Flags::new(1).mode(Channel::_1));
    }

    #[test]
    fn test_channel2_status() {
        assert_eq!(Mode::CC, Flags::new(0).mode(Channel::_2));
        assert_eq!(Mode::CV, Flags::new(2).mode(Channel::_2));
    }

    #[test]
    fn test_beep_status() {
        assert_eq!(Switch::Off, Flags::new(0).beep());
        assert_eq!(Switch::On, Flags::new(16).beep());
    }

    #[test]
    fn test_lock_status() {
        assert_eq!(Lock::Unlocked, Flags::new(0).lock());
        assert_eq!(Lock::Locked, Flags::new(32).lock());
    }

    #[test]
    fn test_output_status() {
        assert_eq!(Switch::Off, Flags::new(0).output());
        assert_eq!(Switch::On, Flags::new(64).output());
    }
    #[test]
    fn test_voltage_from_str() {
        assert_eq!(
            V {
                volts: 10,
                milli_volts: 0
            },
            "10.0".parse::<V>().unwrap()
        );
        assert_eq!(
            V {
                volts: 1,
                milli_volts: 9
            },
            "1.9".parse::<V>().unwrap()
        );
        assert_eq!(
            V {
                volts: 0,
                milli_volts: 9
            },
            "0.9".parse::<V>().unwrap()
        );
    }

    #[test]
    fn test_current_from_str() {
        assert_eq!(
            I {
                ampere: 10,
                milli_ampere: 0
            },
            "10.0".parse::<I>().unwrap()
        );
        assert_eq!(
            I {
                ampere: 1,
                milli_ampere: 9
            },
            "1.9".parse::<I>().unwrap()
        );
        assert_eq!(
            I {
                ampere: 0,
                milli_ampere: 9
            },
            "0.9".parse::<I>().unwrap()
        );
    }
}
