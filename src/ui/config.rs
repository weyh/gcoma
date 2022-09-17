use serde::{Deserialize, Serialize};
use yansi::Color::RGB;

use crate::session_core::session_group::SessionGroup;

#[derive(Serialize, Deserialize)]
pub struct MyColor {
    r: u8,
    g: u8,
    b: u8,
}

macro_rules! mycolor_to_yansicolour {
    ($color:expr) => {
        RGB($color.r as u8, $color.g as u8, $color.b as u8)
    };
}

const DEFAULT_PRINARY_COLOR: MyColor = MyColor {
    r: 248,
    g: 248,
    b: 242,
};

const DEFAULT_SUCCESS_COLOR: MyColor = MyColor {
    r: 80,
    g: 250,
    b: 123,
};

const DEFAULT_HIGHLIGHT_COLOR: MyColor = MyColor {
    r: 241,
    g: 250,
    b: 140,
};

const DEFAULT_ERROR_COLOR: MyColor = MyColor {
    r: 241,
    g: 250,
    b: 140,
};

#[derive(Serialize, Deserialize)]
pub struct UIColors {
    pub primary_color: MyColor,
    pub success_color: MyColor,
    pub error_color: MyColor,
    pub highlight_color: MyColor,
}

impl UIColors {
    pub fn primary(&self) -> yansi::Color {
        mycolor_to_yansicolour!(&self.primary_color)
    }

    pub fn success(&self) -> yansi::Color {
        mycolor_to_yansicolour!(&self.success_color)
    }

    pub fn error(&self) -> yansi::Color {
        mycolor_to_yansicolour!(&self.error_color)
    }

    pub fn highlight(&self) -> yansi::Color {
        mycolor_to_yansicolour!(&self.highlight_color)
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
