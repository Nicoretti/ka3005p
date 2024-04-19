//! doc
use crate::{find_serial_port, list_serial_ports, Command, Ka3005p, Status, Switch};
use anyhow::Error;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::PyErr;

// TODO's:
//
// * Build python extension based on feature flag
// * Add custom Exception to the module see: https://pyo3.rs/main/exception
// * Expose IDN
// * Expose Mode status
// * Expose Channel status
// * Expose Lock status
// * Export Lock, Channel, Mode and Switch Enum types to python
// * Wrap in python (src) based project
//   - provide typeshed
//   - provide readme
//   - provide usage examples
//

struct Ka3005pError(Error);

impl From<Ka3005pError> for PyErr {
    fn from(error: Ka3005pError) -> Self {
        PyException::new_err(error.0.to_string())
    }
}

impl From<Error> for Ka3005pError {
    fn from(error: Error) -> Self {
        Self(error)
    }
}

#[pyclass]
/// Represents a power supply device.
struct PowerSupply {
    inner: Ka3005p,
}

// helper methods
impl PowerSupply {
    /// Execute a command on the power supply.
    ///
    /// Args:
    ///     command: Command to be executed.
    ///
    /// Returns:
    ///     Result of executing the command.
    fn _execute(&mut self, command: Command) -> PyResult<()> {
        Ok(self
            .inner
            .execute(command)
            .map_err(Into::<Ka3005pError>::into)?)
    }

    /// Get the status of the power supply.
    ///
    /// Returns:
    ///     Status of the power supply.
    fn _status(&mut self) -> PyResult<Status> {
        Ok(self.inner.status().map_err(Into::<Ka3005pError>::into)?)
    }
}

#[pymethods]
impl PowerSupply {
    #[new]
    /// Initialize a new PowerSupply instance.
    ///
    /// Args:
    ///     serial_port: Optional serial port for communication.
    ///
    /// Returns:
    ///     New instance of PowerSupply.
    fn new(serial_port: Option<&str>) -> PyResult<Self> {
        let supply = match serial_port {
            Some(port) => PowerSupply {
                inner: Ka3005p::new(port).map_err(Into::<Ka3005pError>::into)?,
            },
            None => PowerSupply {
                inner: find_serial_port().map_err(Into::<Ka3005pError>::into)?,
            },
        };
        Ok(supply)
    }

    /// List all available and compatible power supplies.
    ///
    /// Returns:
    ///     A list of strings representing serial ports controlling power supplies.
    #[staticmethod]
    fn list_power_supplies() -> PyResult<Vec<String>> {
        Ok(list_serial_ports()
            .into_iter()
            .map(|p| p.port_name)
            .collect())
    }

    /// Execute a raw command on the power supply.
    ///
    /// Args:
    ///     command: Raw command string.
    ///
    /// Returns:
    ///     Response from executing the command.
    fn execute(&mut self, command: &str) -> PyResult<Vec<u8>> {
        Ok(self
            .inner
            .run_command(command)
            .map_err(Into::<Ka3005pError>::into)?)
    }

    /// Get the output current setting of the power supply.
    #[getter]
    fn get_current(&mut self) -> PyResult<f32> {
        let status = self._status()?;
        Ok(status.current)
    }

    /// Set the output current of the power supply.
    /// Args:
    ///     i: ampere's to be set.
    #[setter]
    fn set_current(&mut self, i: f32) -> PyResult<()> {
        let command = Command::Current(i);
        self._execute(command)
    }

    /// Get the output voltage setting of the power supply.
    #[getter]
    fn get_voltage(&mut self) -> PyResult<f32> {
        let status = self._status()?;
        Ok(status.voltage)
    }

    /// Set the output voltage of the power supply.
    ///
    /// Args:
    ///     v: volt's to be set.
    #[setter]
    fn set_voltage(&mut self, v: f32) -> PyResult<()> {
        let command = Command::Voltage(v);
        self._execute(command)
    }

    /// Get the status information of the power supply.
    #[getter]
    fn get_status(&mut self) -> PyResult<String> {
        let status = self._status()?;
        Ok(format!("{}", status))
    }

    /// Enable the output of the the power supply.
    fn enable(&mut self) -> PyResult<()> {
        let command = Command::Power(Switch::On);
        self._execute(command)
    }

    /// Disable the output of the the power supply.
    fn disable(&mut self) -> PyResult<()> {
        let command = Command::Power(Switch::Off);
        self._execute(command)
    }

    /// Get the power supply's off/on state.
    fn is_off(&mut self) -> PyResult<bool> {
        let status = self._status()?;
        Ok(!Into::<bool>::into(status.flags.output))
    }

    /// Get the power supply's on/off state.
    fn is_on(&mut self) -> PyResult<bool> {
        let status = self._status()?;
        Ok(status.flags.output.into())
    }

    /// Is beeping enabled.
    ///
    /// Returns:
    ///     `True` if beeping is enabled, otherwise `False`.
    #[getter]
    fn get_beep(&mut self) -> PyResult<bool> {
        let status = self._status()?;
        Ok(status.flags.beep.into())
    }

    /// Set the beep state of the power supply.
    #[setter]
    fn set_beep(&mut self, enable: bool) -> PyResult<()> {
        let command = Command::Beep(Switch::from(enable));
        self._execute(command)
    }

    /// Save the current settings/configuration of the power supply.
    ///
    /// Args:
    ///     id: Memory slot to save to (M: 1-5).
    fn save(&mut self, id: u32) -> PyResult<()> {
        let command = Command::Save(id);
        self._execute(command)
    }

    /// Load stored settings/configuration to the power supply.
    ///
    /// Args:
    ///     id: Memory slot to load from (M: -5).
    fn load(&mut self, id: u32) -> PyResult<()> {
        let command = Command::Load(id);
        self._execute(command)
    }

    /// Set the over current protection state of the power supply.
    ///
    /// Args:
    ///     enable: ocp if `True`, otherwise disable ocp.
    #[setter]
    fn set_ocp(&mut self, enable: bool) -> PyResult<()> {
        let command = Command::Ocp(Switch::from(enable));
        self._execute(command)
    }

    /// Set the over voltage protection state of the power supply.
    ///
    /// Args:
    ///     enable: ovp if `True`, otherwise disable ovp.
    #[setter]
    fn set_ovp(&mut self, enable: bool) -> PyResult<()> {
        let command = Command::Ovp(Switch::from(enable));
        self._execute(command)
    }
}

/// Python module for interfacing with the PowerSupply class.
#[pymodule]
fn ka3005p(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PowerSupply>()?;
    Ok(())
}
