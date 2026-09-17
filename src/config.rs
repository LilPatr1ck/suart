use std::fmt;
use strum::VariantArray;

/// A trait extension to automate console menus and string conversions for enums.
pub trait EnumItems: strum::VariantArray + std::fmt::Display + PartialEq + Copy + Default {
    
    /// Returns a vector containing the string representations of all enum variants.
    /// Dynamically appends a `(Default)` suffix to the default variant at runtime.
    fn to_string_vec() -> Vec<String> {
        Self::VARIANTS
            .iter()
            .map(|variant| {
                if variant == &Self::default() {
                    format!("{} (Default)", variant)
                } else {
                    format!("{}", variant)
                }
            })
            .collect()
    }

    /// Renders a native `dialoguer` selection menu in the terminal.
    fn interact_select(prompt: &str) -> Self {
        let items = Self::to_string_vec();

        let default_idx = Self::VARIANTS
            .iter()
            .position(|&variant| variant == Self::default())
            .unwrap_or(0);

        let selection_index = dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt(prompt)
            .default(default_idx)
            .items(&items)
            .interact()
            .unwrap();

        Self::VARIANTS[selection_index]
    }

    /// Renders a manual text input prompt with an optional fallback value.
    fn interact_input<N>(prompt: &str, default: N) -> N 
    where
        N: std::str::FromStr + Clone + fmt::Display,
        <N as std::str::FromStr>::Err: fmt::Display,
    {
        dialoguer::Input::<N>::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt(prompt)
            .default(default)
            .interact_text()
            .unwrap()
    }
}

impl<T: strum::VariantArray + std::fmt::Display + Copy + Default + PartialEq> EnumItems for T {}


/// Runtime representation of baud rate data (either selected or entered manually).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BaudRate {
    Fixed(u32),
    Custom(u32),
}

impl fmt::Display for BaudRate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fixed(val) => write!(f, "{} bps", val),
            Self::Custom(val) => write!(f, "{} bps (Custom)", val),
        }
    }
}


/// Runtime representation of the connection hardware timeout.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Timeout {
    Fixed(u32),
    Custom(u32),
}

impl fmt::Display for Timeout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fixed(0) => write!(f, "0 ms (No timeout)"),
            Self::Fixed(val) => write!(f, "{} ms", val),
            Self::Custom(val) => write!(f, "{} ms (Custom)", val),
        }
    }
}


/// Selection for the serial port parity configuration.
#[derive(Debug, Clone, Copy, PartialEq, VariantArray, Default)]
pub enum ParityStyleSelection { 
    #[default] 
    None, 
    Odd, 
    Even 
}

impl fmt::Display for ParityStyleSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Odd => write!(f, "Odd"),
            Self::Even => write!(f, "Even"),
        }
    }
}


/// Selection for standard and custom serial port baud rates.
#[derive(Debug, Clone, Copy, PartialEq, VariantArray, Default)]
pub enum BaudRateSelection {
    B9600,
    B19200,
    B38400,
    B57600,
    #[default]
    B115200,
    Custom, 
}

impl fmt::Display for BaudRateSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::B9600 => write!(f, "9600 bps"),
            Self::B19200 => write!(f, "19200 bps"),
            Self::B38400 => write!(f, "38400 bps"),
            Self::B57600 => write!(f, "57600 bps"),
            Self::B115200 => write!(f, "115200 bps"),
            Self::Custom => write!(f, "Custom (Enter manually)..."),
        }
    }
}


/// Selection for the number of data bits per frame.
#[derive(Debug, Clone, Copy, PartialEq, VariantArray, Default)]
pub enum DataBitsSelection {
    Five,
    Six,
    Seven,
    #[default]
    Eight,
}

impl fmt::Display for DataBitsSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Five => write!(f, "5 bits"),
            Self::Six => write!(f, "6 bits"),
            Self::Seven => write!(f, "7 bits"),
            Self::Eight => write!(f, "8 bits"),
        }
    }
}


/// Selection for the number of stop bits per frame.
#[derive(Debug, Clone, Copy, PartialEq, VariantArray, Default)]
pub enum StopBitsSelection { 
    #[default]
    One,
    Two,
}

impl fmt::Display for StopBitsSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::One => write!(f, "1 bit"),
            Self::Two => write!(f, "2 bits"),
        }
    }
}


/// Selection for read/write timeouts, including option for manual configuration.
#[derive(Debug, Clone, Copy, PartialEq, VariantArray, Default)]
pub enum TimeoutSelection {
    #[default]
    T0,
    T10,
    T100,
    T500,
    Custom,
}

impl fmt::Display for TimeoutSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::T0 => write!(f, "0 ms (No timeout)"),
            Self::T10 => write!(f, "10 ms"),
            Self::T100 => write!(f, "100 ms"),
            Self::T500 => write!(f, "500 ms"),
            Self::Custom => write!(f, "Custom timeout (Enter manually)..."),
        }
    }
}


/// Selection for incoming data rendering mode in the terminal output.
#[derive(Debug, Clone, Copy, PartialEq, VariantArray, Default)]
pub enum DisplayModeSelection {
    #[default]
    Ascii,
    Hex,
    AsciiHex,
}

impl fmt::Display for DisplayModeSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ascii => write!(f, "ASCII"),
            Self::Hex => write!(f, "HEX"),
            Self::AsciiHex => write!(f, "ASCII + HEX"),
        }
    }
}


/// Selection for character-based line endings appended to outgoing packets.
#[derive(Debug, Clone, Copy, PartialEq, VariantArray, Default)]
pub enum LineBreakSelection {
    #[default]
    None,
    Lf,
    Cr,
    Crlf,
}

impl fmt::Display for LineBreakSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Lf => write!(f, "LF (\\n)"),
            Self::Cr => write!(f, "CR (\\r)"),
            Self::Crlf => write!(f, "CRLF (\\r\\n)"),
        }
    }
}


/// Holds parsed and strongly-typed active serial port parameters.
#[derive(Debug, Clone)]
pub struct SerialConfig {
    pub port_name: String,
    pub baud_rate: BaudRate,
    pub data_bits: DataBitsSelection,
    pub stop_bits: StopBitsSelection,        
    pub timeout_ms: Timeout,      
    pub parity: ParityStyleSelection,
    pub display_mode: DisplayModeSelection, 
    pub line_break: LineBreakSelection,     
}

impl SerialConfig {
    /// Opens the physical serial port based on the active structural parameters.
    pub fn open_port(&self) -> Result<Box<dyn serialport::SerialPort>, serialport::Error> {
        let raw_baud = match self.baud_rate {
            BaudRate::Fixed(val) => val,
            BaudRate::Custom(val) => val,
        };

        let raw_timeout = match self.timeout_ms {
            Timeout::Fixed(val) => val,
            Timeout::Custom(val) => val,
        };

        let sp_data_bits = match self.data_bits {
            DataBitsSelection::Five => serialport::DataBits::Five,
            DataBitsSelection::Six => serialport::DataBits::Six,
            DataBitsSelection::Seven => serialport::DataBits::Seven,
            DataBitsSelection::Eight => serialport::DataBits::Eight,
        };

        let sp_stop_bits = match self.stop_bits {
            StopBitsSelection::One => serialport::StopBits::One,
            StopBitsSelection::Two => serialport::StopBits::Two,
        };

        let sp_parity = match self.parity {
            ParityStyleSelection::None => serialport::Parity::None,
            ParityStyleSelection::Odd => serialport::Parity::Odd,
            ParityStyleSelection::Even => serialport::Parity::Even,
        };

        serialport::new(&self.port_name, raw_baud)
            .data_bits(sp_data_bits)
            .stop_bits(sp_stop_bits)
            .parity(sp_parity)
            .timeout(std::time::Duration::from_millis(raw_timeout as u64))
            .open()
    }
}
