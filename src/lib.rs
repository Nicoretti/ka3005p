struct Current {
    a: u32,
    m_a: u32,
}

struct Voltage {
    v: u32,
    m_v: u32,
}

enum Switch {
    On,
    Off,
}

enum Command {
    Status,
    Beep,
    Power,
    Output,
    OverVoltageProtection,
    OverCurrentProtection,
    Save,
    Load,
    Identification,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {}
}
