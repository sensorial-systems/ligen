use crate::duration::Duration;

#[ligen(opaque)]
pub struct Instant(std::time::Instant);

impl Instant {
    pub(crate) fn now() -> Self {
        Self(std::time::Instant::now())
    }

    pub(crate) fn elapsed(&self) -> Duration {
        Duration(self.0.elapsed())
    }
}
