use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{ItemStruct, Result, Visibility};
use syn::parse::Nothing;

#[proc_macro_attribute]
pub fn nested_state(args: TokenStream, input: TokenStream) -> TokenStream {
  let args = TokenStream2::from(args);
  let input = TokenStream2::from(input);
  let result = match parse_and_expand(args.clone(), input.clone()) {
    Ok(token_stream) => token_stream,
    Err(parse_error) => parse_error.to_compile_error(),
  };
  TokenStream::from(result)
}


fn parse_and_expand(args: TokenStream2, input: TokenStream2) -> Result<TokenStream2> {
  let mut item = parse(args, input)?;
  expand(&mut item)
}


fn parse(args: TokenStream2, input: TokenStream2) -> Result<ItemStruct> {
  syn::parse2::<Nothing>(args)?;
  syn::parse2(input)
}

fn expand(original: &mut ItemStruct) -> Result<TokenStream2> {
  for field in &mut original.fields {
    field.vis = Visibility::Inherited
  }
  Ok(quote! {
    #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
    #original
  })
}

