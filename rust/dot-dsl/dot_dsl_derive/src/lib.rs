// https://doc.rust-lang.org/book/ch19-06-macros.html
extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Attrs)]
pub fn with_attrs_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = ast.ident;
    let gen = quote! {
        impl Attrs for #name {
            fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                self.attrs = attrs
                        .iter()
                        .map(|(a, b)| (a.to_string(), b.to_string()))
                        .collect();
                self
            }

            fn get_attr(&self, key: &str) -> Option<&str> {
                if let Some(v) = self.attrs.get(key) {
                    return Some(v.as_str());
                }
                None
            }
        }
    };
    gen.into()
}
