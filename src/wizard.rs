use crate::config::{
    EnumItems, SerialConfig, BaudRateSelection, DataBitsSelection, 
    StopBitsSelection, TimeoutSelection, ParityStyleSelection, 
    DisplayModeSelection, LineBreakSelection, BaudRate, Timeout
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

            // 2. Baud Rate Selection and Packaging
            let baud_selection = BaudRateSelection::interact_select("Select Baud Rate");
            let baud_rate = match baud_selection {
                BaudRateSelection::B9600 => BaudRate::Fixed(9600),
                BaudRateSelection::B19200 => BaudRate::Fixed(19200),
                BaudRateSelection::B38400 => BaudRate::Fixed(38400),
                BaudRateSelection::B57600 => BaudRate::Fixed(57600),
                BaudRateSelection::B115200 => BaudRate::Fixed(115200),
                BaudRateSelection::Custom => {
                    let val = BaudRateSelection::interact_input("Enter custom Baud Rate", 115200);
                    BaudRate::Custom(val)
                }
            };

            // 3. Frame Layout Selections
            let data_bits = DataBitsSelection::interact_select("Select Data Bits");
            let stop_bits = StopBitsSelection::interact_select("Select Stop Bits");

            // 4. Read/Write Hardware Timeout Packaging
            let timeout_selection = TimeoutSelection::interact_select("Select Read/Write Timeout");
            let timeout_ms = match timeout_selection {
                TimeoutSelection::T0 => Timeout::Fixed(0),
                TimeoutSelection::T10 => Timeout::Fixed(10),
                TimeoutSelection::T100 => Timeout::Fixed(100),
                TimeoutSelection::T500 => Timeout::Fixed(500),
                TimeoutSelection::Custom => {
                    let val = TimeoutSelection::interact_input("Enter custom timeout (ms)", 100);
                    Timeout::Custom(val)
                }
            };

            // 5. Native Enum Application Parameters
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

            // 6. Final Clean Summary Review Gate
            println!("\n========================================");
            println!("        REVIEW YOUR CONFIGURATION       ");
            println!("========================================");
            println!("  Port Name:    {}", config.port_name);
            println!("  Baud Rate:    {}", config.baud_rate);
            println!("  Data Bits:    {}", config.data_bits);
            println!("  Stop Bits:    {}", config.stop_bits);
            println!("  Timeout:      {}", config.timeout_ms);
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
