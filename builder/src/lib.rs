extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use syn::export::ToTokens;
use quote::quote;
use std::process::exit;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let syntax_tree = parse_macro_input!(input as DeriveInput);
    println!("{:#?}", syntax_tree);

    let name = &syntax_tree.ident;
    let builder_name = format!("{}Builder", name);
    let builder_ident = syn::Ident::new(&builder_name, name.span());

    let extended = quote! {
        pub struct #builder_ident {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

        impl #name {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }
    };
    extended.into()
}
