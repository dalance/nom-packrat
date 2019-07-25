#![recursion_limit = "128"]

extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{self, parse_macro_input, parse_quote, AttributeArgs, FnArg, ItemFn, Stmt, Type};

/// Declare packrat storage
///
/// # Arguments
/// * An output type of parser. The type must implement `Clone`.
///
/// # Examples
///
/// ```compile_fail
/// storage!(String);
/// ```
#[proc_macro]
pub fn storage(item: TokenStream) -> TokenStream {
    let t = parse_macro_input!(item as Type);
    let gen = quote! {
        thread_local!(
            pub(crate) static PACKRAT_STORAGE: core::cell::RefCell<
                std::collections::HashMap<(&'static str, *const u8), Option<(#t, usize)>>
            > = {
                core::cell::RefCell::new(std::collections::HashMap::new())
            }
        );
    };
    gen.into()
}

/// Custom attribute for packrat parser
#[proc_macro_attribute]
pub fn packrat_parser(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as AttributeArgs);
    let item = parse_macro_input!(item as ItemFn);
    impl_packrat_parser(&attr, &item)
}

fn impl_packrat_parser(_attr: &AttributeArgs, item: &ItemFn) -> TokenStream {
    let before = impl_packrat_parser_bofore(&item);
    let body = impl_packrat_parser_body(&item);
    let after = impl_packrat_parser_after(&item);

    let mut item = item.clone();

    item.block.stmts.clear();
    item.block.stmts.push(before);
    item.block.stmts.push(body);
    item.block.stmts.push(after);

    item.into_token_stream().into()
}

fn impl_packrat_parser_bofore(item: &ItemFn) -> Stmt {
    let ident = &item.ident;

    let input = if let Some(x) = &item.decl.inputs.first() {
        match x.value() {
            FnArg::Captured(arg) => &arg.pat,
            _ => panic!("function with #[packrat_parser] must have an argument"),
        }
    } else {
        panic!("function with #[packrat_parser] must have an argument");
    };

    parse_quote! {
        let org_input = if let Some(x) = crate::PACKRAT_STORAGE.with(|storage| {
            if let Some(x) = storage.borrow_mut().get(&(stringify!(#ident), #input.as_bytes().as_ptr())) {
                if let Some((x, y)) = x {
                    return Some(Some((x.clone(), *y)))
                } else {
                    return Some(None)
                }
            } else {
                return None
            }
        }) {
            if let Some((x, y)) = x {
                use nom::InputTake;
                let (s, _) = #input.take_split(y);
                return Ok((s, x))
            } else {
                return Err(nom::Err::Error(nom::error::make_error(#input, nom::error::ErrorKind::Fix)));
            }
        } else {
            #input
        };
    }
}

fn impl_packrat_parser_body(item: &ItemFn) -> Stmt {
    let body = item.block.as_ref();
    parse_quote! {
        let body_ret = { #body };
    }
}

fn impl_packrat_parser_after(item: &ItemFn) -> Stmt {
    let ident = &item.ident;

    parse_quote! {
        {
            let ptr = org_input.as_bytes().as_ptr();
            if let Ok((s, x)) = &body_ret {
                use nom::Offset;
                let len = org_input.offset(&s);
                crate::PACKRAT_STORAGE.with(|storage| {
                    storage.borrow_mut().insert((stringify!(#ident), ptr), Some((x.clone(), len)));
                });
            } else {
                crate::PACKRAT_STORAGE.with(|storage| {
                    storage.borrow_mut().insert((stringify!(#ident), ptr), None);
                });
            }
            body_ret
        }
    }
}
