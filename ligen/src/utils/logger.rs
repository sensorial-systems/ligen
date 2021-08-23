use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
/// Logger struct used for Display in the ligen crates
pub struct Logger {}

impl Logger {
    /// log function for the Logger struct
    pub fn log<D: Display>(d: D) {
        println!("{}", d);
    }
}
