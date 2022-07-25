use std::fmt::Formatter;
use dioxus::prelude::Props;

#[derive(PartialEq, Props, Clone)]
pub struct AdSize {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub fluid: bool,
}

impl std::fmt::Display for AdSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.fluid {
            write!(f, "fluid")
        } else {
            write!(f, "[{}, {}]", self.width.unwrap_or(0), self.height.unwrap_or(0))
        }
    }
}

impl AdSize {
    pub fn default() -> Self {
        Self {
            width: Some(320),
            height: Some(50),
            fluid: false,
        }
    }

    pub fn fluid() -> Self {
        Self {
            width: None,
            height: None,
            fluid: true,
        }
    }

    pub fn sized(width: u32, height: u32) -> Self {
        Self {
            width: Some(width),
            height: Some(height),
            fluid: false,
        }
    }
    pub fn to_script(&self) -> String {
        if self.fluid {
            format!("\"fluid\"")
        } else {
            format!("[{}, {}]", self.width.unwrap_or(0), self.height.unwrap_or(0))
        }
    }
}