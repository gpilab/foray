use derive_more::derive::Display;
use iced::widget::{text, Text};
use serde::{Deserialize, Serialize};
use std::{error, time::Instant};

use crate::SYMBOL_FONT;

#[derive(Clone, Debug, Default, Display, PartialEq, Eq)]
pub enum NodeStatus {
    #[default]
    Idle,
    #[display("Running")]
    Running(Instant),
    Error(NodeError),
}

impl NodeStatus {
    pub fn icon(&self) -> Text {
        match self {
            NodeStatus::Idle => text(""),
            NodeStatus::Running(_) => text(""),
            NodeStatus::Error(_) => text("").style(text::danger).font(SYMBOL_FONT),
        }
    }

    pub fn text_element(&self) -> Text {
        match self {
            NodeStatus::Idle => text(""),
            NodeStatus::Running(_) => text("running"),
            NodeStatus::Error(err) => text(err.to_string()).style(text::danger),
        }
    }
}

//TODO: Cleanup errors and make them more discrete where possible
#[derive(Debug, Display, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum NodeError {
    Input(String),
    Output(String),
    Config(String),
    Syntax(String),
    FileSys(String),
    Runtime(String),
    MissingCompute(String),
    #[default]
    Other,
}
impl NodeError {
    pub fn input_error(input_name: impl Into<String>) -> NodeError {
        NodeError::Input(format!("Input '{:}' not found", input_name.into()))
    }
}

impl error::Error for NodeError {}
