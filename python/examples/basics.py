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
