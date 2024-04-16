# KA3005P Python Library
This Python library provides a high-level interface to control Korad, Tenma, RS, Velleman, Stamos, and other compatible power supplies via their serial interface. It is built on top of a Rust library [ka3005p](https://crates.io/crates/ka3005p).

## Installation
Make sure you have Python 3.8 or higher installed. You can install the library from PyPI by running the following command in your terminal:

```bash
pip install ka3005p
```

Note:
If you are using Linux, you may need to add users to the dialout group or adjust the permissions of the serial interfaces that are needed to communicate with the power supply.

To add a user to the dialout group, you can use the following command: 
`sudo usermod -a -G dialout username`(please remember that logging out and logging back in may be required for the changes to take effect).

## Usage

```python
from ka3005p import PowerSupply

# List connected power supplies
devices = PowerSupply.list_power_supplies()

# Take control over a power supply
#
# Attention: Only one handle at the time is allowed to exist for a single PowerSupply.
#
#  Note: If no parameter is specified the first power supply which was found will be used.
power_supply = PowerSupply(devices[0])


# Prepare output voltage and current
power_supply.voltage = 12.0
power_supply = 0.5

# Turn on the output
power_supply.enable()

# Read voltage and current  
v = power_supply.voltage
a = power_supply.current

# Store current settings in memory slot 1
power_supply.save(1)

# Turn off the output
power_supply.disable()

# Load settings from memory slot 2
power_supply.load(2)
```

## Building from Source
If you need to build the library from the source, you'll need Python development headers and Rust installed:

1. Clone the repository:
   ```bash
   git clone git@github.com:nicoretti/ka3005p.git
   cd ka3005p
   ```

2. Build and install using maturin (install it if it's not installed):
   ```bash
   pip install maturin
   maturin develop
   ```

3. To build a wheel:
   ```bash
   maturin build --release
   ```

## License
This project is licensed under either of
                                                                                                                 
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
                                                                                                                 
- MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
                                                                                                                 
at your option.
