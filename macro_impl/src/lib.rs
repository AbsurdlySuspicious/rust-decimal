extern crate proc_macro;

use proc_macro::TokenStream;

use proc_macro_hack::proc_macro_hack;

use quote::quote;

use rust_decimal::Decimal;

use std::str::FromStr;

#[proc_macro_hack]
pub fn dec(input: TokenStream) -> TokenStream {
    let mut source = input.to_string();

    // If it starts with `- ` then get rid of the extra space
    // to_string will put a space between tokens
    if source.starts_with("- ") {
        source.remove(1);
    }

    let decimal = match Decimal::from_str(&source[..]) {
        Ok(d) => d,
        Err(e) => panic!("Unexpected decimal format for {}: {}", source, e),
    };

    let unpacked = decimal.unpack();
    // We need to further unpack these for quote for now
    let value = unpacked.value;
    let negative = unpacked.is_negative;
    let scale = unpacked.scale;
    let expanded = quote! {
        ::rust_decimal::Decimal::from_parts(#value, #negative, #scale)
    };
    expanded.into()
}
