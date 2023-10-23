#[macro_export]
macro_rules! dynamic_parser {
    ($name:ident, $full_parser:path, $symbol_parser:path, $output:ty, $($first:ty, $($rest:ty),*)?) => {
        use $crate::prelude::*;
        use $crate::parser::Parser;

        trait DynamicParser<'a>:
        $(
            Parser<$first, Output = $output>
            $(+ Parser<$rest, Output = $output>)*
            + Parser<&'a str, Output = $output>
        )?
        {}

        pub struct $name {
            parser: Box<dyn for<'a> DynamicParser<'a>>
        }

        impl $name {
            pub fn full() -> Self {
                let parser = Box::new(<$full_parser>::default());
                Self { parser }
            }

            pub fn symbol() -> Self {
                let parser = Box::new(<$symbol_parser>::default());
                Self { parser }
            }
        }

        impl Default for FunctionParser {
            fn default() -> Self {
                Self::full()
            }
        }

        impl Parser<&str> for $name {
            type Output = $output;
            fn parse(&self, input: &str) -> Result<Self::Output> {
                self.parser.parse(input)
            }
        }

        $(
            impl Parser<$first> for $name {
                type Output = $output;
                fn parse(&self, input: $first) -> Result<Self::Output> {
                    self.parser.parse(input)
                }
            }

            $(
                impl Parser<$rest> for $name {
                    type Output = $output;
                    fn parse(&self, input: $rest) -> Result<Self::Output> {
                        self.parser.parse(input)
                    }
                }                    
            )+
        )?
    };
}

pub use dynamic_parser;