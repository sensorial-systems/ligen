#[macro_export]
macro_rules! as_constraint {
    ($typ:ty) => {
        $typ
    };
    ($typ:ty | $constraint:ty) => {
        $constraint
    };
}

#[macro_export]
macro_rules! pick_output {
    ($global:ty) => {
        $global
    };
    ($global:ty, $local:ty) => {
        $local
    }
}

#[macro_export]
macro_rules! trait_definition {
    ($output:ty, $first:ty $(| $first_constraint:ty)? $(=> $first_output:ty)? $(, $rest:ty $(| $rest_constraint:ty)? $(=> $rest_output:ty)?)+) => {
        trait DynamicParser<'a>:
        Parser<$crate::as_constraint!($first $(| $first_constraint)?), Output = $crate::pick_output!($output $(, $first_output)?)>
        $(+ Parser<$crate::as_constraint!($rest $(| $rest_constraint)?), Output = $crate::pick_output!($output $(, $rest_output)?)>)+
        {}
    }
}

#[macro_export]
macro_rules! trait_implementation {
    ($name:ident, $output:ty, $first:ty $(| $first_constraint:ty)? $(=> $first_output:ty)? $(, $rest:ty $(| $rest_constraint:ty)? $(=> $rest_output:ty)?)+) => {
        impl Parser<$first> for $name {
            type Output = $crate::pick_output!($output $(, $first_output)?);
            fn parse(&self, input: $first) -> Result<Self::Output> {
                self.parser.parse(input)
            }
        }

        $(
            impl Parser<$rest> for $name {
                type Output = $crate::pick_output!($output $(, $rest_output)?);
                fn parse(&self, input: $rest) -> Result<Self::Output> {
                    self.parser.parse(input)
                }
            }                    
        )+
    
    };
}

#[macro_export]
macro_rules! dynamic_parser {
    ($name:ident, $full_parser:path, $symbol_parser:path, $output:ty, $($input:tt)*) => {
        use $crate::prelude::*;
        use $crate::parser::Parser;

        $crate::trait_definition!($output, $($input)*);
        $crate::trait_implementation!($name, $output, $($input)*);

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

        impl Default for $name {
            fn default() -> Self {
                Self::full()
            }
        }
    };
}

pub use dynamic_parser;
