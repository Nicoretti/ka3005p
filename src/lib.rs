//! This library helps connect to common power lab bench supplies from Tenma, Farnell, Stamos, Korad, Velleman, RS and various other clones.

//! A quick example of using the library:
//! ```no_run
//! use ka3005p::{Ka3005p, Command, Switch};
//!
//! // There is a helper function to automatically find the power supply
//! let mut dev = ka3005p::find_serial_port().unwrap();
//! // or if you wish to target a particular serial power
//! let mut dev = Ka3005p::new("dev/ttyS0").unwrap();
//!
//! println!("{}", dev.status().unwrap());
//! // "Voltage: 12.00, Current: 0.305, Channel1: CV, Channel2: CV, Lock: Off, Beep: On, Output: On"
//!
//! // Switching the power supply off
//! dev.execute(Command::Power(Switch::Off)).unwrap();
//! // Setting the voltage to 12.1v
//! dev.execute(Command::Voltage(12.1)).unwrap();
//! ```

//! You can also use the accompanying command line utility:
//! ```text
//! > ka3005p status
//! Voltage: 12.00, Current: 0.305, Channel1: CV, Channel2: CV, Lock: Off, Beep: On, Output: On
//! > ka3005p power off
//! Voltage: 12.00, Current: 0.305, Channel1: CV, Channel2: CV, Lock: Off, Beep: On, Output: Off
//! > ka3005p voltage 12.1
//! ```

#![deny(warnings)]
#![warn(missing_docs)]
use anyhow::Context;
use log::debug;
use std::fmt;
use std::io;
use std::str;
use std::time;

#[doc(hidden)] // Users of the library shouldn't use this
pub mod cli;
pub use serialport;
#[cfg(feature = "python_module")]
pub mod py_module;

/// On / Off
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Switch {
    /// Enable the feature/output
    On,
    /// Disable the feature/output
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

/// Commands supported by the power supply.
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
    /// Store current settings to memory. Supports 1 to 5
    Save(u32),
    /// Load stored setting. Note will disable power supply output on load.
    Load(u32),
    /// Sets the voltage. Units in Volts
    Voltage(f32),
    /// Sets the current. Units in Amps
    Current(f32),
}

/// Structure containing all the information fields from the power supply
#[derive(Debug, PartialEq, Eq)]
pub struct Flags {
    /// The raw byte
    flags: u8,
    /// Channel 1. CV or CC mode
    pub channel1: Mode,
    /// Channel 2. CV or CC mode
    pub channel2: Mode,
    /// Interface beep enabled or disabled.
    pub beep: Switch,
    /// Interface locked. Will ignore button presses but not serial commands.
    pub lock: Lock,
    /// Output enabled / disabled
    pub output: Switch,
    // TODO - Some people seem to have read the OCP OVP flags in here.
    // I can't find it in any manuals but it might be trying to figure this out
}

/// Channel One / Two
#[derive(Debug, PartialEq, Eq)]
pub enum Channel {
    /// Channel One of the power supply
    One,
    /// Channel Two of the power supply (if your device has one)
    Two,
}

/// Locked / Unlocked
#[derive(Debug, PartialEq, Eq)]
pub enum Lock {
    /// Device is currently locked. Ignores physical buttons but will still respond to serial commands
    Locked,
    /// Device is currently unlocked.
    Unlocked,
}

/// CC or CV mode
#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    /// Power supply is in Constant Current mode
    Cc,
    /// Power supply is in Constant Voltage mode
    Cv,
}

/// Contains the current Voltage, Current and Flags of the power supply
pub struct Status {
    /// Flags as reported by the power supply
    pub flags: Flags,
    /// Voltage in volts
    pub voltage: f32,
    /// Current in amps
    pub current: f32,
    /// Target Voltage in volts
    pub set_voltage: f32,
    /// Current Limit in amps
    pub set_current: f32,
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
            "Voltage: {:5.2} ({:5.2}), Current: {:5.3} ({:5.3}), CH1: {:?}, CH2: {:?} Lock: {:?}, Beep: {:?}, Output: {:?}",
            self.voltage,
            self.set_voltage,
            self.current,
            self.set_current,
            self.flags.channel1,
            self.flags.channel2,
            self.flags.lock,
            self.flags.beep,
            self.flags.output,
        )
    }
}

impl From<u8> for Flags {
    fn from(flags: u8) -> Self {
        Flags::new(flags)
    }
}
impl Flags {
    fn new(flags: u8) -> Self {
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

/// A helper function to list all of the detected power supplies.
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

/// Helper function that automatically finds and connects to a power supply.
pub fn find_serial_port() -> anyhow::Result<Ka3005p> {
    let serial_devices = list_serial_ports();

    match serial_devices.len() {
        0 => Err(anyhow::anyhow!("No Power Supply Found!")),
        _ => Ka3005p::new(&serial_devices[0].port_name),
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

/// The power supply. The main object of the library.
pub struct Ka3005p {
    serial: Box<dyn serialport::SerialPort>,
}

impl Ka3005p {
    /// Create a power supply object from a serial port address.
    pub fn new(port_name: &str) -> anyhow::Result<Self> {
        let serial = serialport::new(port_name, 9600)
            .timeout(time::Duration::from_millis(60))
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::One)
            .open()?;

        Ok(Ka3005p { serial })
    }

    /// A convenience function to use if your power supply happens to be picky with the settings.
    /// Note the library defaults have fairly large margins so this should be unnecessary.
    pub fn new_from_serial(serial: Box<dyn serialport::SerialPort>) -> anyhow::Result<Self> {
        Ok(Ka3005p { serial })
    }

    /// Execute a command on the power supply.
    /// Note that these supplies do not return anything on a command so the result only indicates if the serial transfer was successful.
    /// You will need to check that status to make sure the power supply is now in the state you expect.
    pub fn execute(&mut self, command: Command) -> anyhow::Result<()> {
        self.run_command(&String::from(command))?;
        Ok(())
    }

    /// Retrieve status information from the power supply
    /// Returns a struct containing all the information about the power supply
    pub fn status(&mut self) -> anyhow::Result<Status> {
        let printable_ascii = |bytes: Vec<u8>| -> String {
            bytes
                .into_iter()
                .filter(|&b| b >= 32 && b <= 126)
                .collect::<Vec<u8>>()
                .into_iter()
                .map(|b| b as char)
                .collect()
        };

        let flags: Flags = self.run_command_response("STATUS?")?[0].into();
        let voltage = printable_ascii(self.run_command_response("VOUT1?")?).parse()?;
        let current = printable_ascii(self.run_command_response("IOUT1?")?).parse()?;
        let set_voltage = printable_ascii(self.run_command_response("VSET1?")?).parse()?;
        let set_current = printable_ascii(self.run_command_response("ISET1?")?).parse()?;
        Ok(Status {
            flags,
            voltage,
            current,
            set_voltage,
            set_current,
        })
    }

    fn run_command_response(&mut self, command: &str) -> anyhow::Result<Vec<u8>> {
        let res = self.run_command(command)?;
        anyhow::ensure!(!res.is_empty(), "PSU did not respond with data");
        Ok(res)
    }

    fn run_command(&mut self, command: &str) -> anyhow::Result<Vec<u8>> {
        let bytes = command.as_bytes();
        debug!("Sending command: {}", command);
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
        debug!(
            "Received response, raw: {:?}, as string: {})",
            result,
            String::from_utf8_lossy(&result)
        );
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
