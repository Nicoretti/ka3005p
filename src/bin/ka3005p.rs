#![deny(warnings)]
use anyhow::Context;
use std::convert::TryInto;
use std::io::BufRead;
use std::process::exit;
use structopt::StructOpt;

fn main() -> ::anyhow::Result<(), anyhow::Error> {
    human_panic::setup_panic!();
    let args = ka3005p::cli::Ka3005p::from_args();

    if let ka3005p::cli::Command::List { verbose } = args.command {
        let devices;
        if verbose {
            // Verbose. List everything
            devices = serialport::available_ports()?;
        } else {
            // Just print devices and then exit
            devices = ka3005p::list_serial_ports();
        }

        println!("{:#?}", devices);
        exit(0);
    }

    let mut serial;
    if let Some(device) = args.device {
        // User specified a device. Use that
        serial = ka3005p::Ka3005p::new(&device)?;
    } else {
        // Otherwise find the device automatically
        serial = ka3005p::find_serial_port()?;
    }

    match args.command {
        ka3005p::cli::Command::Status => {
            println!("{}", serial.status()?);
        }
        ka3005p::cli::Command::Interactive => {
            for line in std::io::BufReader::new(std::io::stdin()).lines() {
                let normalized = String::from(line?.trim());
                let mut argv: Vec<&str> = normalized.split(' ').collect();
                argv.insert(0, "ka3005p");
                let arguments = ka3005p::cli::Ka3005p::from_iter(argv.into_iter());
                serial.execute(
                    arguments
                        .command
                        .try_into()
                        .with_context(|| "unsupported command conversion")?,
                )?;
            }
        }
        _ => {
            serial.execute(
                args.command
                    .try_into()
                    .with_context(|| "unsupported command conversion")?,
            )?;
        }
    };
    exit(0);
}
