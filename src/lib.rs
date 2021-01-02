#![deny(warnings)]
use anyhow::Context;
use std::fmt;
use std::io;
use std::str;
use std::str::FromStr;
use std::time;

pub mod cli;

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
    CC,
    CV,
}

pub struct Status {
    pub flags: Flags,
    pub voltage: f32,
    pub current: f32,
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
        if x {Switch::On} else {Switch::Off}
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

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Voltage: {}, Current: {}, Channel1: {:?}, Channel2: {:?} Lock: {:?}, Beep: {:?}, Output: {:?}",
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
        let channel1 = if flags & 0x01 != 0 {Mode::CV} else {Mode::CC};
        let channel2 = if flags & 0x02 != 0 {Mode::CV} else {Mode::CC};
        let beep = if flags & 0x10 != 0 {Switch::On} else {Switch::Off};
        let lock = if flags & 0x20 != 0 {Lock::Locked} else {Lock::Unlocked};
        let output = if flags & 0x40 != 0 {Switch::On} else {Switch::Off};
        Flags {flags, channel1, channel2, beep, lock, output}
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
            let mut serial = serialport::new(&serial_devices[0].port_name, 9600).open().unwrap();
            serial.set_timeout(time::Duration::from_millis(50)).unwrap();
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
            Command::Voltage(v) => format!("VSET1:{:.3}", v),
            Command::Current(i) => format!("ISET1:{:.3}", i),
        }
    }
}

pub fn execute(serial: &mut dyn serialport::SerialPort, command: Command) -> anyhow::Result<()> {
    run_command(serial, &String::from(command))?;
    Ok(())
}

/// Retrieve status information from the power supply
pub fn status(serial: &mut dyn serialport::SerialPort) -> anyhow::Result<Status> {
    let flags = Flags::new(run_command_response(serial, "STATUS?")?[0]);
    let voltage = f32::from_str(
        String::from_utf8_lossy(&run_command_response(serial, "VOUT1?")?)
            .into_owned()
            .as_str(),
    )?;
    let current = f32::from_str(
        String::from_utf8_lossy(&run_command_response(serial, "IOUT1?")?)
            .into_owned()
            .as_str(),
    )?;
    Ok(Status::new(flags, voltage, current))
}

fn run_command_response(serial: &mut dyn serialport::SerialPort, command: &str)  -> anyhow::Result<Vec<u8>> {
    let res = run_command(serial, command)?;
    anyhow::ensure!(!res.is_empty(), "PSU did not respond with data");
    Ok(res)
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
        assert_eq!(Mode::CC, Flags::new(0).channel1);
        assert_eq!(Mode::CV, Flags::new(1).channel1);
    }

    #[test]
    fn test_channel2_status() {
        assert_eq!(Mode::CC, Flags::new(0).channel2);
        assert_eq!(Mode::CV, Flags::new(2).channel2);
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
}
