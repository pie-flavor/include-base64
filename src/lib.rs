//! include-base64 is a library for including a file as a base64 string, à la [`std::include_str!`].
#![deny(missing_docs)]
extern crate proc_macro;
#[macro_use]
extern crate syn;

use std::{
    error::Error,
    fs::File,
    io::{self, BufReader},
    path::Path,
};

use base64::{write::EncoderStringWriter, CharacterSet, Config};
use proc_macro::TokenStream;
use quote::quote;
use syn::LitStr;

/// Includes a file as a base-64 string literal at compile time.
///
/// # Example
///
/// ```rust
/// # use include_base64::include_base64;
/// const MY_FILE_BUT_IN_BASE64: &str = include_base64!("my_file.txt");
/// ```
#[proc_macro]
pub fn include_base64(input: TokenStream) -> TokenStream {
    let string = parse_macro_input!(input as LitStr).value();
    include_from_file(Path::new(&string)).unwrap_or_else(|s| {
        let s = s.to_string();
        quote!(compile_error!(#s)).into()
    })
}

fn include_from_file(path: &Path) -> Result<TokenStream, Box<dyn Error>> {
    let len = path.metadata()?.len();
    if len / 8 > usize::MAX as u64 {
        return Err(format!("File too big (max: {})", usize::MAX).into());
    }
    let mut file_reader = BufReader::new(File::open(path)?);
    let mut string_writer = EncoderStringWriter::new(Config::new(CharacterSet::UrlSafe, true));
    io::copy(&mut file_reader, &mut string_writer)?;
    let string = string_writer.into_inner();
    Ok(quote!(#string).into())
}
