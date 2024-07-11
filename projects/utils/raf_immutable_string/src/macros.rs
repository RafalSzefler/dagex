#[macro_export]
macro_rules! imm {
    ( $text: expr ) => {
        {
            let txt = { $text } as &str;
            immutable_string::ImmutableString::new(txt).expect("Couldn't construct ImmutableString instance.")
        }
    };
}

pub use imm;
