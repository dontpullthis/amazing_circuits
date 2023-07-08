pub mod desktop;
pub mod toolbar;

use bevy::prelude::*;

pub const FONT: &str = "/usr/share/fonts/truetype/msttcorefonts/arial.ttf";

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);