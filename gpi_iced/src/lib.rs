#![feature(iter_map_windows)]

use iced::Font;
use indexmap::IndexMap;

pub type OrderMap<K, V> = IndexMap<K, V>;
pub const DEFAULT_FONT: Font = Font::with_name("Canterell");
pub const MATH_FONT: Font = Font::with_name("DejaVu Math TeX Gyre");
pub const SYMBOL_FONT: Font = Font::with_name("CaskaydiaCove Nerd Font");

pub mod app;
pub mod graph;
pub mod interface;
pub mod math;
pub mod node_data;
pub mod nodes;
pub mod python;
pub mod style;
pub mod widget;
