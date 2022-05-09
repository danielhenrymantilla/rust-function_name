extern crate proc_macro; use ::proc_macro::TokenStream;
use ::syn::{self,
    ItemFn,
    parse_macro_input,
    spanned::Spanned,
};
use ::quote::{
    quote,
    quote_spanned,
};

#[cfg(not(feature = "test"))]
macro_rules! CRATE_NAME {() => (
    ::core::convert::identity::<syn::Ident>(syn::parse_str(
        &::proc_macro_crate::crate_name("function_name-proc-macro")
            .expect("Cargo.toml must have a function_name dependency")
    ).unwrap())
)}
#[cfg(feature = "test")]
macro_rules! CRATE_NAME {() => (
    ::core::convert::identity::<syn::Ident>(syn::parse_quote! {
        function_name
    })
)}

const IDENT_SUFFIX: &'static str = "__hack__";
const RAW_PREFIX: &'static str = "r#";

#[proc_macro_attribute] pub
fn named (params: TokenStream, input: TokenStream) -> TokenStream
{
    if params.clone().into_iter().next().is_some() {
        return syn::Error::new_spanned(
            if true { params.into() } else { quote!() },
            "#[named] does not take arguments",
        ).to_compile_error().into();
    }
    let mut input_fn = parse_macro_input!(input as ItemFn);
    let ident = syn::Ident::new(
        &format!("{}{}", input_fn.ident.to_string().replace(RAW_PREFIX, ""), IDENT_SUFFIX),
        input_fn.ident.span(),
    );
    let _crate = CRATE_NAME!();
    let block = *input_fn.block;
    *input_fn.block = syn::parse_quote! {
        {
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            #[derive(::#_crate::named_hack)]
            enum #ident {}

            #block
        }
    };
    TokenStream::from(quote_spanned! { input_fn.span() =>
        #input_fn
    })
}

#[doc(hidden)]
#[proc_macro_derive(named_hack)] pub
fn hack (input: TokenStream) -> TokenStream
{
    let input: syn::DeriveInput = parse_macro_input!(input);
    let ident = input.ident.to_string();
    let ident = &ident[.. ident.len() - IDENT_SUFFIX.len()];
    let fname = syn::LitStr::new(ident, input.ident.span());
    TokenStream::from(quote! {
        macro_rules! function_name {() => (
            #fname
        )}
    })
}
