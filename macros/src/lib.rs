extern crate proc_macro;
use proc_macro::TokenStream;
use quote::ToTokens;
use regex::Regex;
use syn::{parse_macro_input, Item, ItemMod};

// hey this was gonna be a cool thing to help me map addresses + shifts + masks
// to actual struct definitions, but it turns out the API code sucks and doesn't
// name the registers consistently
struct TMC2240Field {}

#[proc_macro_attribute]
pub fn generate_fields(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let re = Regex::new(r"^TMC2240_([A-Z_]+)_([A-Z]+)$").unwrap();

    let m = parse_macro_input!(input as ItemMod);
    let items = m.clone().content.unwrap().1;
    for item in items {
        if let Item::Const(c) = item {
            let name = c.ident.to_string();
            if let Some(m) = re.captures(&name) {
                if let Some(field) = m.get(1) {
                    println!("{}", field.as_str());
                    continue;
                } else {
                    continue;
                }
                if let Some(kind) = m.get(2) {
                    continue;
                } else {
                    continue;
                }
            };
        }
    }

    m.to_token_stream().into()
}
// TMC2240_SPI_STATUS_RESET_FLAG_MASK
