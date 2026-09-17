use crate::config::{SerialConfig, DisplayModeSelection, LineBreakSelection};
use std::io::{self, Read, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Manages interactive bidirectional input/output streaming loops over an open port.
pub struct SerialTerminal;

impl SerialTerminal {
    /// Launches concurrent background threads for reading from the port and catching user input.
    pub fn start(config: SerialConfig, mut port: Box<dyn serialport::SerialPort>) {
        println!("--- Active Terminal Session Engaged ---");
        println!("Type standard characters and press Enter to transmit. Use Ctrl+C to terminate session.\n");

        let mut reader_port = port.try_clone().expect("Failed to split hardware reference context for asynchronous reading");
        let display_mode = config.display_mode;
        
        // Create an asynchronous channel to bridge terminal input with the main transmitter loop
        let (tx, rx) = mpsc::channel::<Vec<u8>>();

        // Stream A: Dedicated Asynchronous Hardware Buffer Monitor Loop
        thread::spawn(move || {
            let mut buffer: [u8; 1024] = [0; 1024];
            loop {
                match reader_port.read(&mut buffer) {
                    Ok(bytes_count) if bytes_count > 0 => {
                        Self::process_incoming(&buffer[..bytes_count], display_mode);
                    }
                    Ok(_) => {}
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                        // Regular timeout event; yield frame context to proceed polling safely
                    }
                    Err(err) => {
                        eprintln!("\n[Hardware connection severed: {}]", err);
                        break;
                    }
                }
            }
        });

        // Stream B: Dedicated User Console Stdin Monitor Thread
        // This isolates the blocking read_line call from both the hardware logic and the OS drivers
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

                    // Process dynamic Line Ending injections
                    match config.line_break {
                        LineBreakSelection::None => {}
                        LineBreakSelection::Lf => bytes_to_send.push(b'\n'),
                        LineBreakSelection::Cr => bytes_to_send.push(b'\r'),
                        LineBreakSelection::Crlf => {
                            bytes_to_send.push(b'\r');
                            bytes_to_send.push(b'\n');
                        }
                    }

                    // Forward processed transmission frames to the transmitter pipeline
                    if tx.send(bytes_to_send).is_err() {
                        break;
                    }
                }
            }
        });

        // Stream C: Main Thread Low-Latency Hardware Transmitter Pipeline
        // Listens to the channel thread and writes directly to the physical serial bus
        loop {
            // Receive data frames with a short timeout to prevent complete thread starvation
            if let Ok(payload) = rx.recv_timeout(Duration::from_millis(50)) {
                if let Err(err) = port.write_all(&payload) {
                    eprintln!("[Transmission failure: {}]", err);
                } else {
                    let _ = port.flush();
                }
            }
        }
    }

    /// Evaluates, reformats, and flushes raw ingress packet arrays based on runtime requirements.
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
