use ligen_macro::inner_ligen;

inner_ligen!(ignore);

pub trait Trait {}

pub struct Ignored;

impl Trait for Ignored {}

impl Ignored {
    // `&dyn Trait` isn't support and it will panic. ligen_macro::ignore!() should avoid that.
    pub fn boxed(_object: &dyn Trait) -> Box<dyn Trait> {
        Self.into()
    }
}
