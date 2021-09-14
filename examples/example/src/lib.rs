// pub mod person;
// pub mod ffi;
// pub mod ignored;

pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[repr(C)]
pub struct Instant(std::time::Instant);

impl Instant {
    fn now() -> Self {
        Self(std::time::Instant::now())
    }

    fn elapsed(&self) -> std::time::Duration {
        self.0.elapsed()
    }
}

pub fn now() -> *mut Instant {
    let instant = Instant::now();
    Box::into_raw(Box::new(instant))
}

pub fn elapsed(instant: *mut Instant) {
    unsafe {
        println!("{:#?}", (*instant).elapsed())
    }
}
