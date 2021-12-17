use ligen_macro::ligen;

pub mod time;
pub use time::Instant;
pub use time::instant::Instant as RenamedInstant;
pub use time::duration::*;

pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

pub fn now() -> Instant {
    Instant::now()
}

pub fn elapsed(instant: *mut Instant) -> *mut Duration {
    unsafe {
        Box::into_raw(Box::new((*instant).elapsed()))
    }
}

pub fn print_duration(duration: *mut Duration) {
    unsafe {
        println!("{:#?}", (*duration).0);
    }
}