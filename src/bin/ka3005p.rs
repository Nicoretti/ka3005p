use anyhow;
use anyhow::Context;
use human_panic;
use ka3005p;
use std::convert::TryInto;
use std::io::BufRead;
use structopt::StructOpt;

fn main() -> ::anyhow::Result<(), anyhow::Error> {
    human_panic::setup_panic!();
    let args = ka3005p::cli::Ka3005p::from_args();
    let mut serial = ka3005p::find_serial_port()?;
    match args.command {
        ka3005p::cli::Command::Status => {
            println!("{}", ka3005p::status(serial.as_mut())?);
        }
        ka3005p::cli::Command::Interactive => {
            for line in std::io::BufReader::new(std::io::stdin()).lines() {
                let normalized = String::from(line?.trim());
                let mut argv: Vec<&str> = normalized.split(" ").collect();
                argv.insert(0, "ka3005p");
                let arguments = ka3005p::cli::Ka3005p::from_iter(argv.into_iter());
                ka3005p::execute(
                    serial.as_mut(),
                    arguments
                        .command
                        .clone()
                        .try_into()
                        .with_context(|| "unsupported command conversion")?,
                )?;
            }
        }
        _ => {
            ka3005p::execute(
                serial.as_mut(),
                args.command
                    .clone()
                    .try_into()
                    .with_context(|| "unsupported command conversion")?,
            )?;
        }
    };
    std::process::exit(1);
}
