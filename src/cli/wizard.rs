use crate::config::{
    EnumItems, SerialConfig, BaudRateSelection, DataBitsSelection, 
    StopBitsSelection, TimeoutSelection, ParityStyleSelection, 
    DisplayModeSelection, LineBreakSelection
};
use dialoguer::{theme::ColorfulTheme, Select, Confirm};

/// Orchestrates the step-by-step interactive CLI wizard to configure a serial port.
pub struct SerialWizard;

impl SerialWizard {
    /// Runs the interactive terminal wizard loop and returns a validated `SerialConfig`.
    /// 
    /// If the user rejects the configuration on the final review screen, 
    /// the wizard resets and restarts the process automatically.
    pub fn run() -> SerialConfig {
        loop {
            println!("========================================");
            println!("     SERIAL PORT CONFIGURATION WIZARD   ");
            println!("========================================\n");

            // 1. Hardware Port Detection and Selection
            let port_name: String = match serialport::available_ports() {
                Ok(ports) if !ports.is_empty() => {
                    let mut port_list: Vec<String> = ports
                        .iter()
                        .map(|p| p.port_name.clone())
                        .collect();
                    
                    port_list.push("Enter manually...".to_string());

                    let idx = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Select target Serial Port")
                        .default(0)
                        .items(&port_list)
                        .interact()
                        .unwrap();

                    if idx == port_list.len() - 1 {
                        BaudRateSelection::interact_input("Enter Port Name", "COM1".to_string())
                    } else {
                        port_list[idx].clone()
                    }
                }
                _ => {
                    println!("No active serial ports detected in the system.");
                    BaudRateSelection::interact_input("Enter Port Name manually", "COM1".to_string())
                }
            };

            // 2. Baud Rate Selection and Custom Parser
            let baud_selection = BaudRateSelection::interact_select("Select Baud Rate");
            let baud_rate: u32 = match baud_selection {
                BaudRateSelection::B9600 => 9600,
                BaudRateSelection::B19200 => 19200,
                BaudRateSelection::B38400 => 38400,
                BaudRateSelection::B57600 => 57600,
                BaudRateSelection::B115200 => 115200,
                BaudRateSelection::Custom => {
                    BaudRateSelection::interact_input("Enter custom Baud Rate", 115200)
                }
            };

            // 3. Data Bits Frame Mapping
            let data_bits_selection = DataBitsSelection::interact_select("Select Data Bits");
            let data_bits: u8 = match data_bits_selection {
                DataBitsSelection::Five => 5,
                DataBitsSelection::Six => 6,
                DataBitsSelection::Seven => 7,
                DataBitsSelection::Eight => 8,
            };

            // 4. Stop Bits Frame Mapping
            let stop_bits_selection = StopBitsSelection::interact_select("Select Stop Bits");
            let stop_bits: u8 = match stop_bits_selection {
                StopBitsSelection::One => 1,
                StopBitsSelection::Two => 2,
            };

            // 5. Read/Write Hardware Timeout Parser
            let timeout_selection = TimeoutSelection::interact_select("Select Read/Write Timeout");
            let timeout_ms: u32 = match timeout_selection {
                TimeoutSelection::T0 => 0,
                TimeoutSelection::T10 => 10,
                TimeoutSelection::T100 => 100,
                TimeoutSelection::T500 => 500,
                TimeoutSelection::Custom => {
                    TimeoutSelection::interact_input("Enter custom timeout (ms)", 100)
                }
            };

            // 6. Native Enum Application Parameters
            let parity = ParityStyleSelection::interact_select("Select Parity");
            let display_mode = DisplayModeSelection::interact_select("Select Display Mode");
            let line_break = LineBreakSelection::interact_select("Select Line Endings");

            let config = SerialConfig {
                port_name,
                baud_rate,
                data_bits,
                stop_bits,
                timeout_ms,
                parity,
                display_mode,
                line_break,
            };

            // 7. Final Configuration Summary and Confirmation Gate
            println!("\n========================================");
            println!("        REVIEW YOUR CONFIGURATION       ");
            println!("========================================");
            println!("  Port Name:    {}", config.port_name);
            println!("  Baud Rate:    {} bps", config.baud_rate);
            println!("  Data Bits:    {}", config.data_bits);
            println!("  Stop Bits:    {}", config.stop_bits);
            println!("  Timeout:      {} ms", config.timeout_ms);
            println!("  Parity:       {}", config.parity);
            println!("  Display Mode: {}", config.display_mode);
            println!("  Line Ending:  {}", config.line_break);
            println!("========================================\n");

            let confirmed = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Is this configuration correct?")
                .default(true)
                .interact()
                .unwrap();

            if confirmed {
                println!("\nConfiguration successfully saved!\n");
                return config;
            }

            println!("\nRestarting wizard...\n");
        }
    }
}
