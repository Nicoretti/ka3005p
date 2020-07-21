use ka3005p;
use human_panic;
use std::clone::Clone;
use std::convert::TryInto;
use std::io;
use structopt::StructOpt;

mod cli {

    #[derive(Debug, Copy, Clone)]
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
                _ => Err(String::from("Value must be either 'on' or 'off'")),
            }
        }
    }

    #[derive(structopt::StructOpt, Debug, Copy, Clone)]
    pub enum Command {
        /// Turns on or off the ouput of the power supply
        Power {
            #[structopt(help = "on/off")]
            switch: Switch,
        },
        /// Return status inforation about the power spply
        Status,
        /// Set the voltage of the ouput or config
        Voltage {
            #[structopt(help = "volts")]
            v: u32,
            #[structopt(help = "milli volts")]
            mv: u32,
        },
        /// Set the current of the ouput or config
        Current {
            #[structopt(help = "ampere")]
            a: u32,
            #[structopt(help = "milli ampere")]
            ma: u32,
        },
        /// Saves current pannel settingts to specified config
        Save {
            // TODO: add more robust type which only allways avialable ids
            #[structopt(help = "1,2,3,4")]
            id: u32,
        },
        /// Loads config settings of specified no.
        Load {
            // TODO: add more robust type which only allways avialable ids
            #[structopt(help = "1,2,3,4")]
            id: u32,
        },
        /// Enbale/Disable over current protection
        Ocp {
            #[structopt(help = "on/off")]
            switch: Switch,
        },
        /// Enbale/Disable over voltage protection
        Ovp {
            #[structopt(help = "on/off")]
            switch: Switch,
        },
        /// Enbale/Disable Beep
        Beep {
            #[structopt(help = "on/off")]
            switch: Switch,
        },
    }

    impl std::convert::TryInto<ka3005p::Command> for Command {
        type Error = String;
        fn try_into(self) -> Result<ka3005p::Command, Self::Error> {
            match self {
                Command::Power { switch } => match switch {
                    Switch::On => Ok(ka3005p::Command::Power(ka3005p::Switch::On)),
                    Switch::Off => Ok(ka3005p::Command::Power(ka3005p::Switch::Off)),
                },
                Command::Ovp { switch } => match switch {
                    Switch::On => Ok(ka3005p::Command::Ovp(ka3005p::Switch::On)),
                    Switch::Off => Ok(ka3005p::Command::Ovp(ka3005p::Switch::Off)),
                },
                Command::Ocp { switch } => match switch {
                    Switch::On => Ok(ka3005p::Command::Ocp(ka3005p::Switch::On)),
                    Switch::Off => Ok(ka3005p::Command::Ocp(ka3005p::Switch::Off)),
                },
                Command::Beep { switch } => match switch {
                    Switch::On => Ok(ka3005p::Command::Beep(ka3005p::Switch::On)),
                    Switch::Off => Ok(ka3005p::Command::Beep(ka3005p::Switch::Off)),
                },
                Command::Load { id } => Ok(ka3005p::Command::Load(id)),
                Command::Save { id } => Ok(ka3005p::Command::Save(id)),
                Command::Voltage { v, mv } => Ok(ka3005p::Command::Voltage(ka3005p::V::new(v, mv))),
                Command::Current { a, ma } => Ok(ka3005p::Command::Current(ka3005p::I::new(a, ma))),
                Command::Status => Err(String::from("Conversion of status is not supported")),
            }
        }
    }

    #[derive(structopt::StructOpt, Debug)]
    #[structopt(about = "Controls a KA3005P bench power supply through its serial interface")]
    #[structopt(global_settings(& [structopt::clap::AppSettings::ColoredHelp]))]
    pub struct Ka3005p {
        #[structopt(subcommand)]
        pub command: Command,
    }
}

fn main() -> io::Result<()> {
    human_panic::setup_panic!();
    let args = cli::Ka3005p::from_args();
    let mut serial = ka3005p::find_serial_port().unwrap();
    match args.command {
        cli::Command::Status => {
            println!("{}", ka3005p::status(serial.as_mut()));
        }
        _ => {
            ka3005p::execute(
                serial.as_mut(),
                args.command
                    .clone()
                    .try_into()
                    .expect("unsupported command converison"),
            );
        }
    };
    std::process::exit(1);
}
