from ka3005p import PowerSupply

def main():
    power_supply = PowerSupply()
    print(f"Current Voltage: {power_supply.voltage}")


if __name__ == '__main__':
    main()

