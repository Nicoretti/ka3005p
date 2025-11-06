use clap::{Parser, Subcommand};
use std::clone::Clone;

#[derive(Copy, Clone, PartialEq, Subcommand)]
pub enum Command {
    /// Turns on or off the ouput of the power supply
    Power {
        /// on/off
        #[clap(help = "on/off")]
        switch: crate::Switch,
    },
    /// Return status inforation about the power spply
    Status,
    /// Set the voltage of the ouput or config
    Voltage {
        #[clap(help = "volts")]
        v: f32,
    },
    /// Set the current of the ouput or config
    Current {
        #[clap(help = "ampere")]
        a: f32,
    },
    /// Saves current pannel settings to specified config
    Save {
        #[clap(help = "1,2,3,4")]
        id: u32,
    },
    /// Loads config settings of specified no.
    Load {
        #[clap(help = "1,2,3,4")]
        id: u32,
    },
    /// Enable/Disable over current protection
    Ocp {
        #[clap(help = "on/off")]
        switch: crate::Switch,
    },
    /// Enable/Disable over voltage protection
    Ovp {
        #[clap(help = "on/off")]
        switch: crate::Switch,
    },
    /// Enable/Disable Beep
    Beep {
        #[clap(help = "on/off")]
        switch: crate::Switch,
    },
    /// list possible power supply devices
    List {
        /// List all serial ports, not just ones that match the USB ids
        #[clap(short, long)]
        verbose: bool,
    },
    /// Read commands from stdin and execute them
    Interactive,
}

impl std::convert::TryInto<crate::Command> for Command {
    type Error = anyhow::Error;
    fn try_into(self) -> anyhow::Result<crate::Command, Self::Error> {
        match self {
            Command::Power { switch } => Ok(crate::Command::Power(switch)),
            Command::Ovp { switch } => Ok(crate::Command::Ovp(switch)),
            Command::Ocp { switch } => Ok(crate::Command::Ocp(switch)),
            Command::Beep { switch } => Ok(crate::Command::Beep(switch)),
            Command::Load { id } => Ok(crate::Command::Load(id)),
            Command::Save { id } => Ok(crate::Command::Save(id)),
            Command::Voltage { v } => Ok(crate::Command::Voltage(v)),
            Command::Current { a } => Ok(crate::Command::Current(a)),
            _ => Err(anyhow::anyhow!("Conversion is not supported")),
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
#[command(color = clap::ColorChoice::Auto)]
#[command(styles = clap::builder::styling::Styles::styled()
    .header(clap::builder::styling::AnsiColor::Yellow.on_default())
    .usage(clap::builder::styling::AnsiColor::Blue.on_default())
    .literal(clap::builder::styling::AnsiColor::BrightBlue.on_default())
    .placeholder(clap::builder::styling::AnsiColor::Cyan.on_default())
    )]
pub struct Ka3005p {
    #[clap(subcommand)]
    pub command: Command,
    /// Manually select power supply serial device
    #[clap(short, long)]
    pub device: Option<String>,
}
