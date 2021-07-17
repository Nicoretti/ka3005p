use std::clone::Clone;

#[derive(structopt::StructOpt, Debug, Copy, Clone)]
pub enum Command {
    /// Turns on or off the ouput of the power supply
    Power {
        #[structopt(help = "on/off")]
        switch: crate::Switch,
    },
    /// Return status inforation about the power spply
    Status,
    /// Set the voltage of the ouput or config
    Voltage {
        #[structopt(help = "volts")]
        v: f32,
    },
    /// Set the current of the ouput or config
    Current {
        #[structopt(help = "ampere")]
        a: f32,
    },
    /// Saves current pannel settings to specified config
    Save {
        #[structopt(help = "1,2,3,4")]
        id: u32,
    },
    /// Loads config settings of specified no.
    Load {
        #[structopt(help = "1,2,3,4")]
        id: u32,
    },
    /// Enable/Disable over current protection
    Ocp {
        #[structopt(help = "on/off")]
        switch: crate::Switch,
    },
    /// Enable/Disable over voltage protection
    Ovp {
        #[structopt(help = "on/off")]
        switch: crate::Switch,
    },
    /// Enable/Disable Beep
    Beep {
        #[structopt(help = "on/off")]
        switch: crate::Switch,
    },
    /// list possible power supply devices
    List {
        /// List all serial ports, not just ones that match the USB ids
        #[structopt(short, long)]
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

#[derive(structopt::StructOpt, Debug)]
#[structopt(about = "Controls a KA3005P bench power supply through its serial interface")]
#[structopt(global_settings(& [structopt::clap::AppSettings::ColoredHelp]))]
pub struct Ka3005p {
    #[structopt(subcommand)]
    pub command: Command,
    /// Manually select power supply serial device
    #[structopt(short, long)]
    pub device: Option<String>,
}
