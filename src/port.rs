use crate::config::SerialConfig;
use serialport::SerialPort;

/// Manages physical serial interface initialization and low-level state resetting.
pub struct PortManager;

impl PortManager {
    /// Opens the serial connection and flushes hardware buffers to ensure a clean transaction state.
    pub fn initialize(config: &SerialConfig) -> Result<Box<dyn SerialPort>, serialport::Error> {
        let mut port = config.open_port()?;
        port.flush()?;
        Ok(port)
    }
}
