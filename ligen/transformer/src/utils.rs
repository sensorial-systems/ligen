#[derive(Debug)]
pub struct WithSource<T> {
    pub source: String,
    pub ast: T
}

impl<T> WithSource<T> {
    pub fn new(source: impl AsRef<str>, ast: T) -> Self {
        let source = source.as_ref().to_string();
        Self { source, ast }
    }

    pub fn sub<U>(&self, ast: U) -> WithSource<U> {
        WithSource::new(&self.source, ast)
    }
}