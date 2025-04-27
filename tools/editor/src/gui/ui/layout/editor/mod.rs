use ligen_transformer::prelude::*;
use ligen_python_parser::{PythonParser, PythonParserConfig};

use self::ir::Editor;

use super::panes::{Pane, PaneManager};
use crate::prelude::*;

pub mod ir;
pub mod settings;
pub mod widget;
pub mod menu_button;
pub mod parsers;
pub mod generators;