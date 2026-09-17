use crate::config::{SerialConfig, DisplayModeSelection, LineBreakSelection};
use std::io::{self, Read, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Runs the interactive session for an opened serial port.
pub struct SerialTerminal;

impl SerialTerminal {
    /// Reads incoming bytes and sends user-entered lines concurrently.
    pub fn start(config: SerialConfig, mut port: Box<dyn serialport::SerialPort>) {
        println!("--- Active Terminal Session Engaged ---");
        println!("Type standard characters and press Enter to transmit. Use Ctrl+C to terminate session.\n");

        let mut reader_port = port
            .try_clone()
            .expect("Failed to clone the serial port for reading");
        let display_mode = config.display_mode;
        
        // The stdin thread cannot write to the port directly because the port is also read here.
        let (tx, rx) = mpsc::channel::<Vec<u8>>();

        // Keep reading independently so incoming data is shown while stdin waits for a line.
        thread::spawn(move || {
            let mut buffer: [u8; 1024] = [0; 1024];
            loop {
                match reader_port.read(&mut buffer) {
                    Ok(bytes_count) if bytes_count > 0 => {
                        Self::process_incoming(&buffer[..bytes_count], display_mode);
                    }
                    Ok(_) => {}
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                        // A timeout is expected: it lets the loop check the port again.
                    }
                    Err(err) => {
                        eprintln!("\n[Hardware connection severed: {}]", err);
                        break;
                    }
                }
            }
        });

        // read_line blocks, so stdin gets its own thread and cannot stop serial reads.
        thread::spawn(move || {
            let mut user_input = String::new();
            loop {
                user_input.clear();
                if io::stdin().read_line(&mut user_input).is_ok() {
                    let trimmed = user_input.trim_end();
                    if trimmed.is_empty() {
                        continue;
                    }

                    let mut bytes_to_send = trimmed.as_bytes().to_vec();

                    // The selected terminator is added after trimming the terminal's newline.
                    match config.line_break {
                        LineBreakSelection::None => {}
                        LineBreakSelection::Lf => bytes_to_send.push(b'\n'),
                        LineBreakSelection::Cr => bytes_to_send.push(b'\r'),
                        LineBreakSelection::Crlf => {
                            bytes_to_send.push(b'\r');
                            bytes_to_send.push(b'\n');
                        }
                    }

                    if tx.send(bytes_to_send).is_err() {
                        break;
                    }
                }
            }
        });

        // Only this loop writes to the port, keeping serial writes ordered.
        loop {
            if let Ok(payload) = rx.recv_timeout(Duration::from_millis(50)) {
                if let Err(err) = port.write_all(&payload) {
                    eprintln!("[Transmission failure: {}]", err);
                } else {
                    let _ = port.flush();
                }
            }
        }
    }

    /// Prints an incoming chunk in the selected representation.
    fn process_incoming(bytes: &[u8], mode: DisplayModeSelection) {
        match mode {
            DisplayModeSelection::Ascii => {
                print!("{}", String::from_utf8_lossy(bytes));
                let _ = io::stdout().flush();
            }
            DisplayModeSelection::Hex => {
                for byte in bytes {
                    print!("{:02X} ", byte);
                }
                let _ = io::stdout().flush();
            }
            DisplayModeSelection::AsciiHex => {
                let ascii = String::from_utf8_lossy(bytes);
                let hex: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
                println!("\n[HEX: {}] -> ASCII: {}", hex.join(" "), ascii.trim());
            }
        }
    }
}
