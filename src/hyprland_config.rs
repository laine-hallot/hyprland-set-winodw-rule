use std::{
    fmt::{Display, Error, Formatter, Result as FmtResult},
    num::ParseIntError,
    str::FromStr,
};

use crate::system_info;
use color_eyre::eyre;
use hyprland::data::Client;

enum WindowMode {
    Tile,
    Float,
}

impl ToString for WindowMode {
    fn to_string(&self) -> String {
        match self {
            Self::Float => "float".to_string(),
            Self::Tile => "tile".to_string(),
        }
    }
}

enum Parameter {
    Class(String),
    Title(String),
    InitialClass(String),
    InitialTitle(String),
}

impl ToString for Parameter {
    fn to_string(&self) -> String {
        match self {
            Self::Class(class) => class.clone(),
            Self::Title(title) => title.clone(),
            Self::InitialClass(initial_class) => initial_class.clone(),
            Self::InitialTitle(initial_title) => initial_title.clone(),
        }
    }
}

struct WindowRule {
    mode: WindowMode,
    parameters: Vec<Parameter>,
}

impl Display for WindowRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parameters = self
            .parameters
            .iter()
            .map(|param| param.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        return write!(f, "windowrule = {}, {}", self.mode.to_string(), parameters,);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseRuleError {
    /// The string does not follow the expected “x,y wxh” layout.
    InvalidFormat,
    /// One of the integer components could not be parsed.
    InvalidNumber(ParseIntError),
}

impl std::fmt::Display for ParseRuleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ParseRuleError::InvalidFormat => write!(f, "invalid region format"),
            ParseRuleError::InvalidNumber(e) => write!(f, "invalid number: {}", e),
        }
    }
}

impl std::error::Error for ParseRuleError {}

impl From<ParseIntError> for ParseRuleError {
    fn from(err: ParseIntError) -> Self {
        ParseRuleError::InvalidNumber(err)
    }
}

pub fn generate_config_for(client: &Client) -> eyre::Result<()> {
    let hyprland_dir = system_info::get_hyprland_dir()?;

    if hyprland_dir.join("").exists() {
        // check for existing rule file for client
    }

    return Ok(());
}
