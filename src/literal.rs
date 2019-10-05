#[derive(Debug)]
pub enum Literal {
    String(String),
    Bool(bool),
    Char(char),
    Integer(i64),
    UnsignedInteger(u64),
    Float(f64)
}

impl Literal {
    pub fn parse(lit: &syn::Lit) -> Literal {
        match lit {
            syn::Lit::Str(litstr) => {
                Literal::String(litstr.value())
            },
            syn::Lit::Byte(litbyte) => {
                Literal::UnsignedInteger(litbyte.value() as u64)
            },
            syn::Lit::Char(litchar) => {
                Literal::UnsignedInteger(litchar.value() as u64)
            },
            syn::Lit::Int(litint) => {
                Literal::Integer(litint.base10_parse().unwrap())
            },
            syn::Lit::Float(litfloat) => {
                Literal::Float(litfloat.base10_parse().unwrap())
            },
            syn::Lit::Bool(litbool) => {
                Literal::Bool(litbool.value)
            },
            _ => Literal::String(String::from(""))
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Literal::String(value) => format!("{}", value),
            Literal::Bool(value) => format!("{}", value),
            Literal::Char(value) => format!("{}", value),
            Literal::Integer(value) => format!("{}", value),
            Literal::UnsignedInteger(value) => format!("{}", value),
            Literal::Float(value) => format!("{}", value)
        }
    }
}
