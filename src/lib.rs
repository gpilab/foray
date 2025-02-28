use std::{collections::BTreeMap, path::PathBuf};

use iced::Font;

pub type StableMap<K, V> = BTreeMap<K, V>;
pub const DEFAULT_FONT: Font = Font::with_name("CaskaydiaCove");
pub const MATH_FONT: Font = Font::with_name("DejaVu Math TeX Gyre");
pub const SYMBOL_FONT: Font = Font::with_name("CaskaydiaCove Nerd Font");

pub mod app;
pub mod file_watch;
pub mod graph;
pub mod gui_node;
pub mod interface;
pub mod math;
pub mod nodes;
pub mod python;
pub mod style;
pub mod widget;

pub fn nodes_dir() -> PathBuf {
    PathBuf::from("/home/john/projects/gpi_v2/nodes/")
}
pub fn venv_dir() -> PathBuf {
    PathBuf::from("/home/john/projects/gpi_v2/nodes/.venv")
}
