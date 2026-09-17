# SimpleUART

Interactive command-line tool for working with UART and other serial ports.

`suart` discovers available ports, asks for connection settings, then provides a
small terminal for sending lines and viewing incoming bytes. It is useful for
checking embedded devices, boot logs, sensors, and simple serial protocols.

## Features

- Port selection from detected devices or manual port name entry
- Preset or custom baud rate and read timeout
- Configurable data bits, stop bits, and parity
- ASCII, hexadecimal, or combined output for received data
- Optional `LF`, `CR`, or `CRLF` appended to sent lines
- A confirmation screen before opening the connection

## Requirements

- Rust and Cargo (edition 2024)
- Access to the target serial device
- On Linux, permission to open the device, commonly through the `dialout` group

## Build and Run

```bash
git clone https://github.com/LilPatr1ck/suart.git
cd suart
cargo run --release
```

The wizard guides you through the serial settings. After the port is opened,
type a line and press Enter to send it. Incoming data is printed immediately.
Press `Ctrl+C` to end the session.

To check the project without connecting to hardware:

```bash
cargo check
```

## Typical Configuration

For a common USB-to-UART adapter connected to a microcontroller, start with:

| Setting | Value |
| --- | --- |
| Baud rate | `115200` |
| Data bits | `8` |
| Stop bits | `1` |
| Parity | `None` |
| Display mode | `ASCII` |
| Line ending | `None` |

The device must use the same serial parameters. If a device expects commands
terminated by a newline, select `LF` or `CRLF` as required by its protocol.

## License

License information has not been added yet.
