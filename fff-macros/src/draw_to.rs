use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::*;

pub fn do_draw_to(attr: TokenStream, item: TokenStream) -> TokenStream {
    let target = parse_macro_input!(attr as Ident);
    let body = parse_macro_input!(item as Item);

    if let Item::Impl(mut im) = body {
        let used: Vec<_> = im
            .items
            .iter()
            .map(|it| {
                if let ImplItem::Method(mm) = it {
                    return mm.sig.ident.to_string();
                }
                panic!("invalid impl Draw")
            })
            .collect();

        im.items.extend(get_methods(used, target));

        TokenStream::from(quote! {
            #im
        })
    } else {
        panic!("require a impl Draw block")
    }
}

fn to_m(input: proc_macro2::TokenStream) -> ImplItem {
    let m = TokenStream::from(input);
    syn::parse::<ImplItem>(m).unwrap()
}

fn get_methods(used: Vec<String>, target: Ident) -> Vec<ImplItem> {
    let mut ms: Vec<ImplItem> = Vec::new();

    if !used.contains(&"get_rect".to_string()) {
        ms.push(to_m(quote! {
            fn get_rect(&self) -> &crate::ui::base::shape::Rect {
                self.#target.get_rect()
            }
        }))
    }

    if !used.contains(&"move_to".to_string()) {
        ms.push(to_m(quote! {
            fn move_to(&mut self, point: &crate::ui::base::shape::Point) {
                self.#target.move_to(point);
            }
        }))
    }

    if !used.contains(&"ensure".to_string()) {
        ms.push(to_m(quote! {
            fn ensure(&mut self, min: &crate::ui::base::shape::Size, max: &crate::ui::base::shape::Size) -> crate::ui::base::shape::Size {
                self.#target.ensure(min, max)
            }
        }))
    }

    if !used.contains(&"do_draw".to_string()) {
        ms.push(to_m(quote! {
            fn do_draw(&mut self) {
                self.#target.do_draw();
            }
        }))
    }

    if !used.contains(&"clear".to_string()) {
        ms.push(to_m(quote! {
            fn clear(&mut self) {
                self.#target.clear();
            }
        }))
    }

    ms
}
