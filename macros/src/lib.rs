use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Ident, LitInt};

extern crate syn;
#[macro_use]
extern crate quote;

#[proc_macro_derive(Register, attributes(addr))]
pub fn register(input: TokenStream) -> TokenStream {
    let m = parse_macro_input!(input as DeriveInput);
    let addr = parse_addr(&m);

    impl_register(&m.ident, &addr).into()
}

fn parse_addr(ast: &DeriveInput) -> LitInt {
    ast.attrs
        .iter()
        .filter(|a| a.path().segments.len() == 1 && a.path().segments[0].ident == "addr")
        .nth(0)
        .expect("addr attribute required for deriving Register!")
        .parse_args()
        .expect("Invalid addr args")
}

fn impl_register(name: &Ident, addr: &LitInt) -> proc_macro2::TokenStream {
    // ... do stuff with `parameters`
    quote! {
        impl Register for #name {
            fn addr() -> u8 {
                #addr
            }

            fn bytes(self) -> [u8; 4] {
                u32::from(self).to_be_bytes()
            }
        }
    }
}
