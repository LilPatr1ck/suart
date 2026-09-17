use crate::config::SerialConfig;
use serialport::SerialPort;

/// Opens and prepares the serial connection.
pub struct PortManager;

impl PortManager {
    /// Opens the configured port and clears pending output before the session starts.
    pub fn initialize(config: &SerialConfig) -> Result<Box<dyn SerialPort>, serialport::Error> {
        let mut port = config.open_port()?;
        port.flush()?;
        Ok(port)
    }
}
