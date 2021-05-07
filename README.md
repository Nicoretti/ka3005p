![Rust](https://github.com/Nicoretti/ka3005p/workflows/Rust/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/ka3005p.svg)](https://crates.io/crates/ka3005p)
[![documentation](https://docs.rs/ka3005p/badge.svg)](https://docs.rs/ka3005p)

# KA3005P
Command line tool to control a KA3005P bench power supply through its serial interface.

Example (Getting Help):
```
user@host ~$ ka3005p -h
ka3005p 0.1.2
Controls a KA3005P bench power supply through its serial interface

USAGE:
    ka3005p <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    beep           Enbale/Disable Beep
    current        Set the current of the ouput or config
    help           Prints this message or the help of the given subcommand(s)
    interactive    Read commands from stdin and execute them
    load           Loads config settings of specified no
    ocp            Enbale/Disable over current protection
    ovp            Enbale/Disable over voltage protection
    power          Turns on or off the ouput of the power supply
    save           Saves current pannel settings to specified config
    status         Return status inforation about the power spply
    voltage        Set the voltage of the ouput or config
```

```
user@host ~$ ka30005p help power
ka3005p-power 0.1.0
Turns on or off the ouput of the power supply

USAGE:
    ka3005p power <switch>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <switch>    on/off
```

## Do automated ramps using the interactive mode
Using the interactive mode you can send continues stream of commands to the power supply.
This can be used e.g. to apply an automated voltage ramp.

```shell
user@host ~$ python3 ramp.py -f 10 -t 20 -p 10 | ka3005p interactive
```

For more details check out the `ramp.py` script.

# License
Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
