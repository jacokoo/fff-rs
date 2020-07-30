use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

fn change_str(str: &str) -> String {
    let mut r = String::new();
    for (idx, c) in str.chars().enumerate() {
        if c.is_lowercase() {
            r.push(c);
        } else {
            if idx != 0 {
                r.push('-');
            }
            r.push(c.to_ascii_lowercase());
        }
    }
    r
}

#[proc_macro]
pub fn kebab_str(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::Ident);
    let out = change_str(&input.to_string());
    TokenStream::from(quote! {
        #out
    })
}
