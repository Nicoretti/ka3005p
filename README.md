# KA3005P
Command line tool to control a KA3005P bench power supply through its serial interface.

Example (Getting Help):
```
user@host ~$ ka3005p help
Controls a KA3005P bench power supply through its serial interface

USAGE:
    ka3005p <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    beep       Enbale/Disable Beep
    current    Set the current of the ouput or config
    help       Prints this message or the help of the given subcommand(s)
    load       Loads config settings of specified no
    ocp        Enbale/Disable over current protection
    ovp        Enbale/Disable over voltage protection
    power      Turns on or off the ouput of the power supply
    save       Saves current pannel settingts to specified config
    status     Return status inforation about the power spply
    voltage    Set the voltage of the ouput or config
```

```
user@host ~$ ka3000 help power
ka3000-power 0.1.0
Turns on or off the ouput of the power supply

USAGE:
    ka3005p power <switch>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <switch>    on/off
```

# License
Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
