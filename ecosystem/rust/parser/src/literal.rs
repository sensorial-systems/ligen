use ligen::idl::Literal;
use ligen::transformer::prelude::*;

#[derive(Default)]
pub struct RustLiteralParser;

impl Transformer<syn::Lit, Literal> for RustLiteralParser {
    fn transform(&self, lit: syn::Lit, _config: &Config) -> Result<Literal> {
        Ok(match lit {
            syn::Lit::Str(litstr) => Literal::String(litstr.value()),
            syn::Lit::Verbatim(litverb) => Literal::String(litverb.to_string()),
            syn::Lit::ByteStr(litbytestr) => {
                Literal::String(String::from_utf8_lossy(&litbytestr.value()).into_owned())
            }
            syn::Lit::Byte(litbyte) => Literal::UnsignedInteger(litbyte.value() as u64),
            syn::Lit::Char(litchar) => Literal::Character(litchar.value()),
            syn::Lit::Int(litint) => Literal::Integer(
                litint
                    .base10_parse()
                    .map_err(|e| Error::Message(format!("Failed to parse integer: {e}")))?,
            ),
            syn::Lit::Float(litfloat) => Literal::Float(
                litfloat
                    .base10_parse()
                    .map_err(|e| Error::Message(format!("Failed to parse float: {e}")))?,
            ),
            syn::Lit::Bool(litbool) => Literal::Boolean(litbool.value),
            syn::Lit::CStr(litcstr) => Literal::String(
                litcstr
                    .value()
                    .to_str()
                    .map_err(|e| Error::Message(format!("Failed to parse CStr: {e}")))?
                    .to_string(),
            ),
            _ => return Err(Error::Message("Failed to parse literal".into())),
        })
    }
}

impl Transformer<syn::punctuated::Punctuated<syn::Expr, syn::token::Comma>, Literal>
    for RustLiteralParser
{
    fn transform(
        &self,
        input: syn::punctuated::Punctuated<syn::Expr, syn::token::Comma>,
        config: &Config,
    ) -> Result<Literal> {
        let mut result = Vec::new();
        for element in input {
            result.push(self.transform(element, config)?);
        }
        Ok(Literal::Array(result))
    }
}

impl Transformer<syn::Ident, Literal> for RustLiteralParser {
    fn transform(&self, input: syn::Ident, _config: &Config) -> Result<Literal> {
        Ok(Literal::String(input.to_string()))
    }
}

/// Where [`constants`] puts a module's constants in the config, so that a later expression can
/// refer to an earlier one by name.
///
/// Namespaced under `ligen` like every other key the config carries, and separate from anything a
/// generator reads: nothing outside this file and the module parser knows it is here.
pub const CONSTANT_PREFIX: &str = "ligen::constant";

/// Records a constant under the name later expressions will refer to it by.
pub fn declare_constant(config: &mut Config, name: &str, value: Literal) {
    config.set(format!("{CONSTANT_PREFIX}::{name}"), value);
}

/// Looks a constant up by the last segment of its path.
///
/// The last segment only, so that `MAX` and `some::module::MAX` find the same thing. Rust would
/// distinguish them; a parser that has no import graph cannot, and answering for the name it does
/// know is more useful than refusing every qualified path.
fn constant(config: &Config, path: &syn::Path) -> Option<Literal> {
    let name = path.segments.last()?.ident.to_string();
    config.get(format!("{CONSTANT_PREFIX}::{name}")).cloned()
}

impl Transformer<syn::Expr, Literal> for RustLiteralParser {
    /// Evaluates a constant expression down to the value it denotes.
    ///
    /// Not an interpreter, and not trying to be: it folds the shapes that appear where a literal is
    /// expected — in a `const` item, and in the length of an array type — and refuses everything
    /// else. That is the difference between `[u8; 32]` being writable as `[u8; ADDRESS_LEN]` and a
    /// program having to spell every length out with a comment explaining why.
    ///
    /// Refusing is still the right answer for anything with a value this cannot know. A parser that
    /// guessed at `size_of::<T>()` would put a number in the IDL that the compiler disagrees with,
    /// and every client generated from it would be wrong in a way nothing would catch.
    fn transform(&self, input: syn::Expr, config: &Config) -> Result<Literal> {
        match input {
            syn::Expr::Lit(lit) => self.transform(lit, config),
            // Parentheses and the invisible groups a macro expansion leaves behind say nothing
            // about the value.
            syn::Expr::Paren(paren) => self.transform(*paren.expr, config),
            syn::Expr::Group(group) => self.transform(*group.expr, config),
            // `10 as usize` is the same ten. The target type is the declaration's business, and the
            // IDL records it there.
            syn::Expr::Cast(cast) => self.transform(*cast.expr, config),
            syn::Expr::Array(array) => self.transform(array.elems, config),
            // `[0; 32]`, which is a literal in every sense a reader means by the word and in none
            // that `syn` does.
            syn::Expr::Repeat(repeat) => {
                let element = self.transform(*repeat.expr, config)?;
                let length = self.transform(*repeat.len, config)?;
                let length = as_count(&length).ok_or_else(|| {
                    Error::Message("Array repeat length isn't a whole number.".into())
                })?;
                Ok(Literal::Array(vec![element; length]))
            }
            syn::Expr::Unary(unary) => {
                let value = self.transform(*unary.expr, config)?;
                negate(&unary.op, value)
            }
            syn::Expr::Binary(binary) => {
                let left = self.transform(*binary.left, config)?;
                let right = self.transform(*binary.right, config)?;
                arithmetic(&binary.op, left, right)
            }
            // `SOME_ARRAY.len()`, which is how a count derived from a table is usually written and
            // is knowable here for exactly the same reason the table is. No other method is: the
            // rest need types and trait resolution, which is a compiler's job.
            syn::Expr::MethodCall(call) if call.method == "len" && call.args.is_empty() => {
                match self.transform(*call.receiver, config)? {
                    Literal::Array(elements) => Ok(Literal::Integer(elements.len() as i64)),
                    Literal::String(value) => Ok(Literal::Integer(value.len() as i64)),
                    _ => Err(Error::Message(
                        "Only an array or a string has a length that can be worked out.".into(),
                    )),
                }
            }
            // A name, which is a value only if the module said what it was. See [`constants`].
            syn::Expr::Path(path) => constant(config, &path.path).ok_or_else(|| {
                Error::Message(format!(
                    "Undefined constant: {}.",
                    path.path
                        .segments
                        .last()
                        .map(|segment| segment.ident.to_string())
                        .unwrap_or_default()
                ))
            }),
            _ => Err(Error::Message(
                "Failed to parse literal from expression".into(),
            )),
        }
    }
}

/// A literal as a count, for the places that need one: how long an array is, how many times a
/// repeat repeats. Negative and fractional numbers are not counts.
fn as_count(literal: &Literal) -> Option<usize> {
    match literal {
        Literal::UnsignedInteger(value) => usize::try_from(*value).ok(),
        Literal::Integer(value) => usize::try_from(*value).ok(),
        _ => None,
    }
}

fn negate(op: &syn::UnOp, value: Literal) -> Result<Literal> {
    match (op, value) {
        (syn::UnOp::Neg(_), Literal::Integer(value)) => Ok(Literal::Integer(-value)),
        (syn::UnOp::Neg(_), Literal::Float(value)) => Ok(Literal::Float(-value)),
        // An unsigned literal written with a minus in front is a negative number that happened to
        // be parsed before its sign.
        (syn::UnOp::Neg(_), Literal::UnsignedInteger(value)) => i64::try_from(value)
            .map(|value| Literal::Integer(-value))
            .map_err(|_| Error::Message("Negated integer is out of range.".into())),
        (syn::UnOp::Not(_), Literal::Boolean(value)) => Ok(Literal::Boolean(!value)),
        _ => Err(Error::Message(
            "Unsupported operator in a constant expression.".into(),
        )),
    }
}

/// Folds arithmetic on two constants.
///
/// Integers are folded as `i128` and given back in the narrowest of the two shapes that fits, so
/// that a subtraction of two unsigned constants stays unsigned and a result that has gone below
/// zero says so rather than wrapping. Anything that overflows is refused: a wrong number in an IDL
/// is worse than a parse error, because nothing downstream is in a position to notice it.
fn arithmetic(op: &syn::BinOp, left: Literal, right: Literal) -> Result<Literal> {
    // A float anywhere in the expression makes the whole of it a float. Two integers are folded as
    // integers, so that `7 / 2` is three the way Rust means it rather than three and a half.
    if matches!(left, Literal::Float(_)) || matches!(right, Literal::Float(_)) {
        let (Some(left), Some(right)) = (as_float(&left), as_float(&right)) else {
            return Err(Error::Message(
                "Only numbers can be combined in a constant expression.".into(),
            ));
        };
        let value = match op {
            syn::BinOp::Add(_) => left + right,
            syn::BinOp::Sub(_) => left - right,
            syn::BinOp::Mul(_) => left * right,
            syn::BinOp::Div(_) => left / right,
            syn::BinOp::Rem(_) => left % right,
            _ => {
                return Err(Error::Message(
                    "Unsupported operator in a constant expression.".into(),
                ))
            }
        };
        return Ok(Literal::Float(value));
    }

    let unsigned =
        matches!(left, Literal::UnsignedInteger(_)) && matches!(right, Literal::UnsignedInteger(_));
    let (Some(left), Some(right)) = (as_integer(&left), as_integer(&right)) else {
        return Err(Error::Message(
            "Only numbers can be combined in a constant expression.".into(),
        ));
    };

    let value = match op {
        syn::BinOp::Add(_) => left.checked_add(right),
        syn::BinOp::Sub(_) => left.checked_sub(right),
        syn::BinOp::Mul(_) => left.checked_mul(right),
        syn::BinOp::Div(_) => left.checked_div(right),
        syn::BinOp::Rem(_) => left.checked_rem(right),
        syn::BinOp::Shl(_) => u32::try_from(right)
            .ok()
            .and_then(|by| left.checked_shl(by)),
        syn::BinOp::Shr(_) => u32::try_from(right)
            .ok()
            .and_then(|by| left.checked_shr(by)),
        syn::BinOp::BitAnd(_) => Some(left & right),
        syn::BinOp::BitOr(_) => Some(left | right),
        syn::BinOp::BitXor(_) => Some(left ^ right),
        _ => {
            return Err(Error::Message(
                "Unsupported operator in a constant expression.".into(),
            ))
        }
    };
    let value = value.ok_or_else(|| {
        Error::Message("Constant expression overflows or divides by zero.".into())
    })?;

    if unsigned && value >= 0 {
        u64::try_from(value)
            .map(Literal::UnsignedInteger)
            .map_err(|_| Error::Message("Constant expression is out of range.".into()))
    } else {
        i64::try_from(value)
            .map(Literal::Integer)
            .map_err(|_| Error::Message("Constant expression is out of range.".into()))
    }
}

fn as_integer(literal: &Literal) -> Option<i128> {
    match literal {
        Literal::Integer(value) => Some(*value as i128),
        Literal::UnsignedInteger(value) => Some(*value as i128),
        _ => None,
    }
}

fn as_float(literal: &Literal) -> Option<f64> {
    match literal {
        Literal::Float(value) => Some(*value),
        Literal::Integer(value) => Some(*value as f64),
        Literal::UnsignedInteger(value) => Some(*value as f64),
        _ => None,
    }
}

impl Transformer<syn::ExprLit, Literal> for RustLiteralParser {
    fn transform(&self, input: syn::ExprLit, config: &Config) -> Result<Literal> {
        self.transform(input.lit, config)
    }
}

impl Transformer<proc_macro::TokenStream, Literal> for RustLiteralParser {
    fn transform(&self, input: proc_macro::TokenStream, config: &Config) -> Result<Literal> {
        self.transform(proc_macro2::TokenStream::from(input), config)
    }
}

impl Transformer<proc_macro2::TokenStream, Literal> for RustLiteralParser {
    fn transform(&self, input: proc_macro2::TokenStream, config: &Config) -> Result<Literal> {
        syn::parse2::<syn::Lit>(input)
            .map_err(|e| Error::Message(format!("Failed to parse literal: {e:?}")))
            .and_then(|literal| self.transform(literal, config))
    }
}

impl Parser<Literal> for RustLiteralParser {
    fn parse(&self, input: impl AsRef<str>, config: &Config) -> Result<Literal> {
        let input = input.as_ref();
        if let Ok(lit) = syn::parse_str::<syn::Lit>(input) {
            Ok(self.transform(lit, config)?)
        } else {
            Ok(Literal::Unknown(input.to_string()))
        }
    }
}

#[cfg(test)]
mod constant_expressions {
    use super::*;
    use crate::prelude::*;

    /// Folds an expression the way a `const` item or an array length would be folded.
    fn fold(source: &str) -> Result<Literal> {
        let expression = syn::parse_str::<syn::Expr>(source).expect("a Rust expression");
        RustLiteralParser.transform(expression, &Config::default())
    }

    /// The same, against a config that already knows some constants.
    fn fold_with(constants: &[(&str, Literal)], source: &str) -> Result<Literal> {
        let mut config = Config::default();
        for (name, value) in constants {
            declare_constant(&mut config, name, value.clone());
        }
        let expression = syn::parse_str::<syn::Expr>(source).expect("a Rust expression");
        RustLiteralParser.transform(expression, &config)
    }

    #[test]
    fn a_plain_literal_is_itself() {
        assert_eq!(fold("42").unwrap(), Literal::Integer(42));
        assert_eq!(fold("-42").unwrap(), Literal::Integer(-42));
        assert_eq!(fold("3.5").unwrap(), Literal::Float(3.5));
        assert_eq!(fold("true").unwrap(), Literal::Boolean(true));
    }

    /// `[0; 4]` is a literal in every sense a reader means by the word and in none that `syn` does,
    /// which is what used to make it fail.
    #[test]
    fn a_repeat_is_the_array_it_stands_for() {
        assert_eq!(
            fold("[7; 3]").unwrap(),
            Literal::Array(vec![Literal::Integer(7); 3])
        );
        assert_eq!(fold("[0; 0]").unwrap(), Literal::Array(Vec::new()));
    }

    #[test]
    fn an_array_is_its_elements() {
        assert_eq!(
            fold("[1, 2, 3]").unwrap(),
            Literal::Array(vec![
                Literal::Integer(1),
                Literal::Integer(2),
                Literal::Integer(3),
            ])
        );
    }

    #[test]
    fn arithmetic_is_folded_to_the_value_it_comes_to() {
        assert_eq!(fold("20 - 2").unwrap(), Literal::Integer(18));
        assert_eq!(fold("(2 + 3) * 4").unwrap(), Literal::Integer(20));
        assert_eq!(fold("1 << 8").unwrap(), Literal::Integer(256));
        assert_eq!(fold("10 as usize").unwrap(), Literal::Integer(10));
        assert_eq!(fold("2 - 20").unwrap(), Literal::Integer(-18));
    }

    /// Integers are folded as integers, so a length works out to the number Rust would use.
    #[test]
    fn dividing_two_integers_gives_an_integer() {
        assert_eq!(fold("7 / 2").unwrap(), Literal::Integer(3));
        assert_eq!(fold("7.0 / 2").unwrap(), Literal::Float(3.5));
    }

    #[test]
    fn a_name_is_the_constant_it_refers_to() {
        let constants = [("PRICE", Literal::Integer(20))];
        assert_eq!(
            fold_with(&constants, "PRICE").unwrap(),
            Literal::Integer(20)
        );
        assert_eq!(
            fold_with(&constants, "PRICE - 2").unwrap(),
            Literal::Integer(18)
        );
        // Qualified the same way it would be written after an import.
        assert_eq!(
            fold_with(&constants, "some::module::PRICE").unwrap(),
            Literal::Integer(20)
        );
        assert_eq!(
            fold_with(&constants, "[0; PRICE]").unwrap(),
            Literal::Array(vec![Literal::Integer(0); 20])
        );
    }

    /// Refusing is the right answer for anything whose value this cannot know. A guess would put a
    /// number in the IDL that the compiler disagrees with, and nothing downstream could catch it.
    /// A count derived from a table, which is how such a thing is usually written and is knowable
    /// here for the same reason the table is.
    #[test]
    fn the_length_of_a_constant_array_is_a_constant() {
        let constants = [(
            "BOUNDARIES",
            Literal::Array(vec![Literal::Integer(1), Literal::Integer(2)]),
        )];
        assert_eq!(
            fold_with(&constants, "BOUNDARIES.len()").unwrap(),
            Literal::Integer(2)
        );
        assert_eq!(
            fold_with(&constants, "BOUNDARIES.len() + 1").unwrap(),
            Literal::Integer(3)
        );
        assert_eq!(
            fold_with(&constants, "[0; BOUNDARIES.len()]").unwrap(),
            Literal::Array(vec![Literal::Integer(0); 2])
        );
    }

    #[test]
    fn what_cannot_be_worked_out_is_refused() {
        assert!(fold("UNDECLARED").is_err());
        assert!(fold("size_of::<u64>()").is_err());
        assert!(
            fold_with(&[("N", Literal::Integer(4))], "N.count_ones()").is_err(),
            "only `len` is knowable without a compiler"
        );
        assert!(fold("[0; UNDECLARED]").is_err());
        assert!(
            fold("1 / 0").is_err(),
            "rather than folding to a wrong number"
        );
        assert!(fold("9223372036854775807 * 9223372036854775807").is_err());
    }
}

#[cfg(test)]
mod test {
    use crate::literal::RustLiteralParser;
    use crate::prelude::*;
    use ligen::idl::literal::mock;
    use ligen::transformer::assert::*;

    #[test]
    fn literal_verbatim() -> Result<()> {
        assert_eq(
            RustLiteralParser,
            mock::literal_verbatim(),
            syn::Lit::Verbatim(proc_macro2::Literal::string("verbatim")),
        )
    }

    #[test]
    fn literal_string() -> Result<()> {
        assert_eq(RustLiteralParser, mock::literal_string(), "\"string\"")
    }

    #[test]
    fn literal_byte_str() -> Result<()> {
        assert_eq(RustLiteralParser, mock::literal_string(), "b\"string\"")
    }

    #[test]
    fn literal_byte() -> Result<()> {
        assert_eq(RustLiteralParser, mock::literal_byte(), "b'A'")
    }

    #[test]
    fn literal_bool() -> Result<()> {
        assert_eq(RustLiteralParser, mock::literal_bool(), "false")
    }

    #[test]
    fn literal_character() -> Result<()> {
        assert_eq(RustLiteralParser, mock::literal_character(), "'A'")
    }

    #[test]
    fn literal_integer() -> Result<()> {
        assert_eq(RustLiteralParser, mock::literal_integer(), "-2")
    }

    #[test]
    fn literal_float() -> Result<()> {
        assert_eq(RustLiteralParser, mock::literal_float(), "3.5")
    }

    #[test]
    fn literal_unknown() -> Result<()> {
        assert_eq(RustLiteralParser, mock::literal_unknown(), ".0") // FIXME: This is actually an expression.
    }
}
