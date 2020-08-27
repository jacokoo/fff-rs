use proc_macro::{TokenStream, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::*;
use syn::parse_macro_input;
use syn::*;

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

mod cus {
    syn::custom_punctuation!(Dot, .);
}

struct Data {
    name: Ident,
    target: Ident,
    fns: Vec<Item>,
}

impl Data {}

impl Parse for Data {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>().expect("struct name is required");
        input.parse::<cus::Dot>().expect("'.' is required");
        let target = input.parse::<Ident>().expect("target name is required");
        let mut fns = Vec::new();
        while !input.is_empty() {
            fns.push(input.parse()?);
        }
        Ok(Data { name, target, fns })
    }
}

#[proc_macro]
pub fn draw_to(input: TokenStream) -> TokenStream {
    println!("{:?}", input);
    let data = parse_macro_input!(input as Data);
    let name = data.name;
    let target = data.target;
    let mut ms: Vec<proc_macro2::TokenStream> = data.fns.iter().map(|it| quote! { #it }).collect();

    let used: Vec<_> = data
        .fns
        .iter()
        .map(|it| {
            return if let Item::Fn(ff) = it {
                return Some(ff.sig.ident.to_string());
            } else {
                None
            };
        })
        .filter(|it| it.is_some())
        .map(|it| it.unwrap())
        .collect();

    if !used.contains(&"get_rect".to_string()) {
        ms.push(quote! {
            fn get_rect(&self) -> &Rect {
                self.#target.get_rect()
            }
        })
    }

    if !used.contains(&"move_to".to_string()) {
        ms.push(quote! {
            fn move_to(&mut self, point: &Point) {
                self.#target.move_to(point);
            }
        })
    }

    if !used.contains(&"ensure".to_string()) {
        ms.push(quote! {
            fn ensure(&mut self, min: &Size, max: &Size) -> Size {
                self.#target.ensure(min, max)
            }
        })
    }

    if !used.contains(&"do_draw".to_string()) {
        ms.push(quote! {
            fn do_draw(&mut self) {
                self.#target.do_draw();
            }
        })
    }

    if !used.contains(&"clear".to_string()) {
        ms.push(quote! {
            fn clear(&mut self) {
                self.#target.clear();
            }
        })
    }

    println!("{:?}", &used);

    let mut ss = proc_macro2::TokenStream::new();
    ss.extend(ms);

    let s = quote! {
        impl Draw for #name {
            #ss
        }
    };

    return TokenStream::from(s);
}
