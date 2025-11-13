use hyprland::data::*;
use hyprland::prelude::*;
use regex::Regex;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Write;
use std::num::ParseIntError;
use std::process::Command;

struct ClientRegion {
    at: (i16, i16),
    size: (i16, i16),
}

impl Display for ClientRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "{},{} {}x{}",
            self.at.0, self.at.1, self.size.0, self.size.1
        );
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseRegionError {
    /// The string does not follow the expected “x,y wxh” layout.
    InvalidFormat,
    /// One of the integer components could not be parsed.
    InvalidNumber(ParseIntError),
}

impl std::fmt::Display for ParseRegionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ParseRegionError::InvalidFormat => write!(f, "invalid region format"),
            ParseRegionError::InvalidNumber(e) => write!(f, "invalid number: {}", e),
        }
    }
}

impl std::error::Error for ParseRegionError {}

impl From<ParseIntError> for ParseRegionError {
    fn from(err: ParseIntError) -> Self {
        ParseRegionError::InvalidNumber(err)
    }
}

impl TryFrom<&str> for ClientRegion {
    type Error = ParseRegionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let dimensions_regex = Regex::new(r"^\s*(-?\d+),\s*(-?\d+)\s+(-?\d+)x(-?\d+)\s*$")
            .expect("Could not create regex");

        let caps = dimensions_regex
            .captures(value)
            .ok_or(ParseRegionError::InvalidFormat)?;

        let x = caps[1].parse::<i16>()?;
        let y = caps[2].parse::<i16>()?;
        let w = caps[3].parse::<i16>()?;
        let h = caps[4].parse::<i16>()?;

        Ok(ClientRegion {
            at: (x, y),
            size: (w, h),
        })
    }
}

//impl<T> std::slice::Join<T> for {}

fn select_window(clients: &Clients) -> std::result::Result<&Client, String> {
    let client_regions: Vec<ClientRegion> = clients
        .into_iter()
        .filter_map(|client| {
            if client.mapped == true {
                return Some(ClientRegion {
                    at: client.at.clone(),
                    size: client.size.clone(),
                });
            } else {
                return None;
            }
        })
        .collect();

    let client_regions = client_regions
        .iter()
        .map(|client| format!("{}", client))
        .collect::<Vec<String>>()
        .join("\n");

    let Ok(mut child) = Command::new("./wlprop.sh")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .current_dir("./")
        .spawn()
    else {
        return Err("Failed to slurp, do you have it installed?".to_string());
    };

    let mut stdin = child.stdin.take().expect("Could not create stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(client_regions.as_bytes())
            .expect("Failed sending window data to slurp (could not write to stdin)");
    });

    if let Ok(selected) = child.wait_with_output() {
        let output_string = str::from_utf8(&selected.stdout)
            .expect("slurp's output(stdout) can't be decoded as text(UTF-8)");

        if let Ok(selected_region) = ClientRegion::try_from(output_string) {
            return match clients.iter().find(|client| {
                return client.at.0 == selected_region.at.0
                    && client.at.1 == selected_region.at.1
                    && client.size.0 == selected_region.size.0
                    && client.size.1 == selected_region.size.1;
            }) {
                Some(selected_client) => Ok(selected_client),
                None => Err("slurp output did not match a region".to_string()),
            };
        } else {
            return Err("Could not parse slurp data".to_string());
        }
    } else {
        return Err("Could not run slurp".to_string());
    }
}

struct Generate {
    float: bool,
    persistentsize: bool,
    tile: bool,
    fullscreen: bool,
}

pub fn exec(float: bool, persistentsize: bool, tile: bool, fullscreen: bool) {
    if let Ok(clients) = Clients::get() {
        let selected_client = select_window(&clients);
        match selected_client {
            Ok(client) => {
                println!("Config: ");
                println!("");
                if float {
                    println!("windowrule = float, initialTitle:{}", client.initial_title);
                }
                if tile {
                    println!("windowrule = tile, initialTitle:{}", client.initial_title);
                }
            }
            Err(err) => println!("{err}"),
        }
    }
}
