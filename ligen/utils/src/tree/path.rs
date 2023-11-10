
pub struct Path<'a, Segment>
where Segment: 'a
{
    pub segments: Vec<Segment>,
    phantom: std::marker::PhantomData<&'a Segment>
}

impl<'a> Path<'a, &'a str> {
    fn from_string(path: &'a str) -> Path<'a, &str> {
        let segments = vec![path];
        let phantom = Default::default();
        Self { segments, phantom }
    }
}

pub trait IntoPath<'a, Segment>
where Segment: 'a
{
    fn into_path(self) -> Path<'a, Segment>;
}

impl<'a, Segment> IntoPath<'a, Segment> for Segment
where Segment: 'a
{
    fn into_path(self) -> Path<'a, Segment> {
        let segments = vec![self];
        let phantom = Default::default();
        Path { segments, phantom }
    }
}

impl<'a, Segment> IntoPath<'a, Segment> for Vec<Segment>
where Segment: 'a
{
    fn into_path(self) -> Path<'a, Segment> {
        let segments = self;
        let phantom = Default::default();
        Path { segments, phantom }
    }
}

impl<'a, Segment> IntoPath<'a, Segment> for &'a [Segment]
where Segment: Copy
{
    fn into_path(self) -> Path<'a, Segment> {
        let segments = self.to_vec();
        let phantom = Default::default();
        Path { segments, phantom }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_single() {
        let value = "A";
        let value = std::slice::from_ref(&value);
        let path: Path<'_, &str> = value.into_path();
        assert_eq!(path.segments, ["A"]);
    }

    #[test]
    fn from_array() {
        let array = ["A", "B", "C"];
        let slice = array.as_slice();
        let path: Path<'_, &str> = slice.into_path();
        assert_eq!(path.segments, ["A", "B", "C"]);
    }

    #[test]
    fn from_vector() {
        let vector = vec!["A", "B", "C"];
        let path: Path<'_, &str> = vector.into_path();
        assert_eq!(path.segments, ["A", "B", "C"]);
    }
}
