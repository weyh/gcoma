use ansi_term::Colour::RGB;
use serde::{Deserialize, Serialize};

use crate::session_core::session_group::SessionGroup;

#[derive(Serialize, Deserialize)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

const DEFAULT_PRINARY_COLOR: Color = Color {
    r: 248,
    g: 248,
    b: 242,
};

const DEFAULT_SUCCESS_COLOR: Color = Color {
    r: 80,
    g: 250,
    b: 123,
};

const DEFAULT_HIGHLIGHT_COLOR: Color = Color {
    r: 241,
    g: 250,
    b: 140,
};

const DEFAULT_ERROR_COLOR: Color = Color {
    r: 241,
    g: 250,
    b: 140,
};

#[derive(Serialize, Deserialize)]
pub struct UIColors {
    pub primary_color: Color,
    pub success_color: Color,
    pub error_color: Color,
    pub highlight_color: Color,
}

impl UIColors {
    pub fn primary(&self) -> ansi_term::Colour {
        RGB(
            self.primary_color.r,
            self.primary_color.g,
            self.primary_color.b,
        )
    }

    pub fn success(&self) -> ansi_term::Colour {
        RGB(
            self.success_color.r,
            self.success_color.g,
            self.success_color.b,
        )
    }

    pub fn error(&self) -> ansi_term::Colour {
        RGB(self.error_color.r, self.error_color.g, self.error_color.b)
    }

    pub fn highlight(&self) -> ansi_term::Colour {
        RGB(
            self.highlight_color.r,
            self.highlight_color.g,
            self.highlight_color.b,
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub colors: UIColors,
    pub session_groups: Vec<SessionGroup>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            version: env!("CARGO_PKG_VERSION").to_string(),
            colors: UIColors {
                primary_color: DEFAULT_PRINARY_COLOR,
                success_color: DEFAULT_SUCCESS_COLOR,
                error_color: DEFAULT_ERROR_COLOR,
                highlight_color: DEFAULT_HIGHLIGHT_COLOR,
            },
            session_groups: Vec::new(),
        }
    }
}
