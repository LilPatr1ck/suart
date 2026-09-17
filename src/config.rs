use std::fmt;
use strum::VariantArray;

/// A trait extension to automate console menus and string conversions for enums.
///
/// **How it works:**
/// Any enum that derives `VariantArray`, implements `Display`, implements `Copy`, 
/// implements `Default`, and implements `PartialEq` automatically receives these helper methods 
/// via a blanket implementation. This completely removes boilerplate code when creating multiple CLI configuration menus.
pub trait EnumItems: strum::VariantArray + std::fmt::Display + PartialEq + Copy + Default {
    
    /// Returns a vector containing the string representations of all enum variants.
    /// 
    /// Useful if you need to manually pass the string items to a custom UI element.
    /// Called as an associated function: `YourEnum::to_string_vec()`
    fn to_string_vec() -> Vec<String> {
        Self::VARIANTS
            .iter()
            .map(|variant| format!("{}", variant))
            .collect()
    }

    /// Renders a native `dialoguer` selection menu in the terminal and returns 
    /// the concrete enum variant chosen by the user.
    ///
    /// The default highlighted item in the menu is automatically determined 
    /// by the enum's `Default` implementation.
    ///
    /// # Arguments
    /// * `prompt` - The text question displayed to the user in the console.
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
    /// 
    /// If the user presses Enter without typing anything, the `default` value is chosen.
    /// Automatically validates that the user input matches the requested type `N`.
    ///
    /// # Arguments
    /// * `prompt` - The text question displayed to the user in the console.
    /// * `default` - The fallback value used if the user submits an empty line.
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
            Self::B115200 => write!(f, "115200 bps (Default)"),
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
            Self::Eight => write!(f, "8 bits (Default)"),
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
            Self::One => write!(f, "1 bit (Default)"),
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
            Self::T0 => write!(f, "0 ms (No timeout / Default)"),
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
            Self::Ascii => write!(f, "ASCII (Default)"),
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
            Self::None => write!(f, "None (Default)"),
            Self::Lf => write!(f, "LF (\\n)"),
            Self::Cr => write!(f, "CR (\\r)"),
            Self::Crlf => write!(f, "CRLF (\\r\\n)"),
        }
    }
}


/// Holds parsed and strongly-typed active serial port parameters.
/// 
/// Ready to be verified or integrated directly into standard serial interface objects.
#[derive(Debug, Clone)]
pub struct SerialConfig {
    pub port_name: String,
    pub baud_rate: u32,
    pub data_bits: u8,
    pub stop_bits: u8,        
    pub timeout_ms: u32,      
    pub parity: ParityStyleSelection,
    pub display_mode: DisplayModeSelection, 
    pub line_break: LineBreakSelection,     
}