use ligen_parsing::parser::{ParserConfig, Parser, ParserConfigSet};
use ligen_python_parser::parser::{PythonParser, PythonParserConfig};

use self::ir::Editor;

use super::panes::{Pane, PaneManager};
use crate::prelude::*;

pub mod ir;
pub mod settings;
pub mod widget;
pub mod menu_button;
pub mod parsing;