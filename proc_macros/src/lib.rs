mod error;

use crate::error::{Error, Result};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

#[proc_macro_derive(GetDays)]
pub fn get_days_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_get_days(&ast)
}

fn impl_get_days(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let days = days("./solutions".to_string());
    let days_segment = quote! {
        #(map.insert(#days);)*
    };
    let en = quote! {
        impl GetDays for #name {
            fn get_days() -> HashMap<String, fn(i32, String, String) -> String> {
                let mut map: HashMap<String, fn(i32, String, String) -> String > = HashMap::new();
                #days_segment
                return map;
            }
        }
    };
    en.into()
}

fn days(path: String) -> Vec<TokenStream2> {
    let rel_path = path;

    let dir = match env::var_os("CARGO_MANIFEST_DIR") {
        Some(manifest_dir) => PathBuf::from(manifest_dir).join(rel_path),
        None => PathBuf::from(rel_path),
    };
    let tuples: Vec<TokenStream2> = match source_file_names(dir) {
        Ok(names) => names.into_iter().map(|day| build_tuple(day)).collect(),
        Err(_) => {
            vec![]
        }
    };
    return tuples;
}

fn build_tuple(mut day: String) -> TokenStream2 {
    day.remove(0);
    let day_quotes = format!("{}", day);
    let module_name = format!("d{}", day);
    let struct_name = format!("D{}", day);
    let module_ident = Ident::new(&module_name, Span::call_site());
    let struct_ident = Ident::new(&struct_name, Span::call_site());
    let tuple = quote! {
        #day_quotes.to_string(), #module_ident::#struct_ident::solve as fn(i32, String, String) -> String
    };
    return tuple;
}

fn source_file_names<P: AsRef<Path>>(dir: P) -> Result<Vec<String>> {
    let mut names = Vec::new();
    let mut failures = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }

        let file_name = entry.file_name();
        if file_name == "mod.rs" || file_name == "lib.rs" || file_name == "main.rs" {
            continue;
        }

        let path = Path::new(&file_name);
        if path.extension() == Some(OsStr::new("rs")) {
            match file_name.into_string() {
                Ok(mut utf8) => {
                    utf8.truncate(utf8.len() - ".rs".len());
                    names.push(utf8);
                }
                Err(non_utf8) => {
                    failures.push(non_utf8);
                }
            }
        }
    }

    failures.sort();
    if let Some(failure) = failures.into_iter().next() {
        return Err(Error::Utf8(failure));
    }

    if names.is_empty() {
        return Err(Error::Empty);
    }

    names.sort();
    Ok(names)
}
