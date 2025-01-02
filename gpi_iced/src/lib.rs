#![feature(iter_map_windows)]

use ordermap::OrderMap as OM;

pub mod app;
pub mod graph;
pub mod math;
pub mod node_data;
pub mod nodes;
pub mod style;
pub mod widget;
pub mod wires;

pub type OrderMap<K, V> = OM<K, V>;
