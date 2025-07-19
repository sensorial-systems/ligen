use crate::*;

pub fn identifier_as_path() -> Path {
    Path {
        segments: vec!["u8".into()],
    }
}

pub fn path() -> Path {
    Path {
        segments: vec![
            "std".into(),
            "convert".into(),
            "TryFrom".into()
        ]
    }
}