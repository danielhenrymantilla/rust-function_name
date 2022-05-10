extern crate proc_macro;

#[allow(unused_imports)]
use {
    ::proc_macro::{
        TokenStream,
    },
    ::syn::{*,
        spanned::Spanned,
        Result,
    },
    ::quote::{
        quote,
        quote_spanned,
    },
};

#[cfg(not(feature = "test"))]
macro_rules! CRATE_NAME {() => (
    ::core::convert::identity::<Ident>(parse_str(
        &::proc_macro_crate::crate_name("function_name")
            .expect("Cargo.toml must have a function_name dependency")
    ).unwrap())
)}
#[cfg(feature = "test")]
macro_rules! CRATE_NAME {() => (
    ::core::convert::identity::<Ident>(parse_quote! {
        function_name
    })
)}

const IDENT_SUFFIX: &'static str = "__hack__";

#[proc_macro_attribute] pub
fn named (params: TokenStream, input: TokenStream)
  -> TokenStream
{
    let _: parse::Nothing = parse_macro_input!(params);
    let mut input_fn = parse_macro_input!(input as ItemFn);
    let mut ident =
        parse_str::<Ident>(
            &format!("{}{}", input_fn.sig.ident.to_string(), IDENT_SUFFIX),
        )
        .unwrap()
    ;
    ident.set_span(input_fn.sig.ident.span());
    let _crate = CRATE_NAME!();
    input_fn.block.stmts.insert(0, parse_quote!(
        #[allow(dead_code, non_camel_case_types)]
        #[derive(::#_crate::named_hack)]
        enum #ident {}
    ));
    TokenStream::from(quote_spanned!(input_fn.span()=>
        #input_fn
    ))
}

#[doc(hidden)] /** Not part of the public API */
#[proc_macro_derive(named_hack)] pub
fn hack (input: TokenStream)
  -> TokenStream
{
    let input: DeriveInput = parse_macro_input!(input);
    let ident = input.ident.to_string();
    let ident = &ident[.. ident.len() - IDENT_SUFFIX.len()];
    let fname = LitStr::new(ident, input.ident.span());
    quote!(
        macro_rules! function_name {() => (
            #fname
        )}
    ).into()
}
