use ligen_ir::Library;

pub mod path;
pub mod alias;

pub trait Transform<In: Sized, Out: Sized> {
    fn transform(&self, data: &In) -> Out;
}

pub trait Transformable: Sized + Clone {
    fn transform<T: Transform<Self, Self>>(&self, transform: T) -> Self {
        transform.transform(self)
    }

    fn transforms(&self, transforms: &[&dyn Transform<Self, Self>]) -> Self {
        let mut data = self.clone();
        for transform in transforms {
            data = transform.transform(&data);
        }
        data
    }
}

impl Transformable for Library {}