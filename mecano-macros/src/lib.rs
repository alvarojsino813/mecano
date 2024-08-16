use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::Item;

static mut ALL_MODES : Vec<String> = Vec::new();

#[proc_macro_attribute]
pub fn word_source(_attr : TokenStream, item : TokenStream) -> TokenStream {

    let input = syn::parse_macro_input!(item as syn::Item);

    let expanded;

    match input {
        Item::Struct(definition) => {
            eprintln!("Flipas con el struct: {}", definition.ident);
            let lower_name = definition.ident.to_string().to_lowercase();

            let upper_ident = format_ident!("_NAME{}", 
                definition.ident.to_string().to_uppercase());

            expanded = quote! {
                {
                    const #upper_ident = #lower_name
                    unsafe {
                        crate::ALL_MODES.push(&lower_name);
                    }
                }
            };
        }
        _ => unimplemented!(),

    }

    return TokenStream::from(expanded);
}
