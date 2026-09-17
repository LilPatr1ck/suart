use std::fmt;
use strum::VariantArray;

// Blanket implementation:
// Automatically applies the EnumItems trait to any type `T` that fulfills the bounds.
impl<T: strum::VariantArray + std::fmt::Display + Copy + Default + PartialEq> EnumItems for T {}

#[derive(Debug, Clone)]
pub struct SerialConfig {
    pub port_name: String,
    pub baud_rate: u32,
    pub data_bits: u8,
    pub parity: ParityStyleSelection,
}


#[derive(Debug, Clone, Copy, PartialEq, VariantArray)]
pub enum ParityStyleSelection { None, Odd, Even }

impl fmt::Display for ParityStyleSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParityStyleSelection::None => write!(f, "None"),
            ParityStyleSelection::Odd => write!(f, "Odd"),
            ParityStyleSelection::Even => write!(f, "Even"),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, VariantArray)]
pub enum BaudRateSelection {
    B9600,
    B19200,
    B38400,
    B57600,
    B115200,
    Custom, 
}


impl fmt::Display for BaudRateSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BaudRateSelection::B9600 => write!(f, "9600 bps"),
            BaudRateSelection::B19200 => write!(f, "19200 bps"),
            BaudRateSelection::B38400 => write!(f, "38400 bps"),
            BaudRateSelection::B57600 => write!(f, "57600 bps"),
            BaudRateSelection::B115200 => write!(f, "115200 bps"),
            BaudRateSelection::Custom => write!(f, "Custom (Enter manually)..."),
        }
    }
}







/// A trait extension to automate console menus and string conversions for enums.
///
/// **How it works:**
/// Any enum that derives `VariantArray`, implements `Display`, implements `Copy`, 
/// and implements `Default` automatically receives these helper methods via a blanket implementation.
/// This completely removes boilerplate code when creating multiple CLI configuration menus.
pub trait EnumItems: strum::VariantArray + std::fmt::Display + Copy + Default + PartialEq {
    
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
    ///
    /// # Example
    /// ```rust
    /// #[derive(VariantArray, Default, Clone, Copy, PartialEq)]
    /// enum Parity { #[default] None, Even, Odd }
    /// impl std::fmt::Display for Parity { ... }
    /// 
    /// // Standard call without passing explicit indexes:
    /// let selected_parity = Parity::interact_select("Select Parity:");
    /// ```
    fn interact_select(prompt: &str) -> Self {
        let items = Self::to_string_vec();

        // Automatically locate the position of the default variant in the array
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
}