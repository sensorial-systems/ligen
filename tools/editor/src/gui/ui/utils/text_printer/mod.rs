use std::fmt::Display;

use crate::gui::ui::editor::settings::Settings;

#[derive(Default)]
pub struct Paper {
    text: String
}

impl Paper {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn print_word(&mut self, word: impl Display) {
        if !self.text.is_empty() {
            self.text.push(' ');
        }
        self.text.push_str(word.to_string().as_ref());
    }
}

#[derive(Default)]
pub struct Printer;

impl Printer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn print<P: Fn(&mut Paper)>(&self, printer: P) -> String {
        let mut paper = Paper::new();
        printer(&mut paper);
        paper.text
    }

    pub fn print_with<P: Fn(&mut Paper)>(&self, mut paper: Paper, printer: P) -> String {
        printer(&mut paper);
        paper.text
    }
}

pub trait TextPrinter {
    type Input;
    fn print(&self, settings: &Settings, paper: &mut Paper, input: &Self::Input) -> &Self;
}

pub trait SelfPrinter {
    fn print(&self, settings: &Settings, paper: &mut Paper) -> &Self;
}
