
pub struct Path<'a, Segment>
where Segment: 'a
{
    pub segments: Vec<Segment>,
    phantom: std::marker::PhantomData<&'a Segment>
}

impl<'a> From<&'a str> for Path<'a, &'a str> {
    fn from(value: &'a str) -> Path<'a, &'a str> {
        let segments = value.split("::").collect();
        let phantom = Default::default();
        Self { segments, phantom }
    }
}

impl<'a, Segment> From<Vec<Segment>> for Path<'a, Segment>
where Segment: 'a
{
    fn from(value: Vec<Segment>) -> Path<'a, Segment> {
        let segments = value;
        let phantom = Default::default();
        Path { segments, phantom }
    }
}

impl<'a, Segment> From<&'a [Segment]> for Path<'a, Segment>
where Segment: Copy
{
    fn from(value: &'a [Segment]) -> Path<'a, Segment> {
        let segments = value.to_vec();
        let phantom = Default::default();
        Path { segments, phantom }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_string() {
        let path = Path::from("A");
        assert_eq!(path.segments, ["A"]);
        let path = Path::from("A::B");
        assert_eq!(path.segments, ["A", "B"]);
    }

    #[test]
    fn from_array() {
        let array = ["A", "B", "C"];
        let slice = array.as_slice();
        let path: Path<'_, _> = Path::from(slice);
        assert_eq!(path.segments, ["A", "B", "C"]);
    }

    #[test]
    fn from_vector() {
        let vector = vec!["A", "B", "C"];
        let path = Path::from(vector);
        assert_eq!(path.segments, ["A", "B", "C"]);
    }
}
