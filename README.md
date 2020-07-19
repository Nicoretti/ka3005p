#  KA3000
Is a command line tool to remote control an KA3005P power supply.

Example (Getting Help):
```
user@host ~$ ka3000 help
Remote controls a KA3000 power supply

USAGE:
    ka3000 <SUBCOMMAND>

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
    ka3000 power <switch>

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



