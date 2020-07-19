use std::io;
use structopt::StructOpt;

mod cli {

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
                _ => Err(String::from("Value must be either 'on' or 'off'")),
            }
        }
    }

    #[derive(structopt::StructOpt, Debug)]
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
            // TODO: add more robust type which only allways avialable ids
            #[structopt(help = "1,2,3,4", default_value = "1")]
            channel: u32,
        },
        /// Set the current of the ouput or config
        Current {
            #[structopt(help = "ampere")]
            a: u32,
            #[structopt(help = "milli ampere")]
            ma: u32,
            // TODO: add more robust type which only allways avialable ids
            #[structopt(help = "1,2,3,4", default_value = "1")]
            channel: u32,
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

    #[derive(structopt::StructOpt, Debug)]
    #[structopt(about = "Remote controls a KA3000 power supply")]
    #[structopt(global_settings(& [structopt::clap::AppSettings::ColoredHelp]))]
    pub struct Ka3000 {
        #[structopt(subcommand)]
        pub command: Command,
    }
}

fn main() -> io::Result<()> {
    let _ = cli::Ka3000::from_args();
    std::process::exit(1);
}
