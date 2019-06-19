extern crate proc_macro; use ::proc_macro::TokenStream;
use ::syn::{
    ItemFn,
    parse_macro_input,
    parse_quote,
};

#[proc_macro_attribute] pub
fn function_name (params: TokenStream, input: TokenStream) -> TokenStream
{
    if params.into_iter().next().is_some() {
        return TokenStream::from(::quote::quote! {
            compile_error!(
                "#[function_name] does not take arguments"
            );
        });
    }
    let mut fn_decl = parse_macro_input!(input as ItemFn);
    let block = *fn_decl.block;
    let ident = &fn_decl.ident;
    *fn_decl.block = parse_quote! {
        {
            #[allow(non_camel_case_types)]
            #[derive(function_name::function_name_hack)]
            enum #ident {}

            #block
        }
    };
    TokenStream::from(::quote::quote!{
        #fn_decl
    })
}

#[doc(hidden)]
#[proc_macro_derive(function_name_hack)] pub
fn hack (input: TokenStream) -> TokenStream
{
    let input: ::syn::DeriveInput = parse_macro_input!(input);
    let fname = ::syn::LitStr::new(&input.ident.to_string(), input.ident.span());
    TokenStream::from(::quote::quote!{
        macro_rules! function_name {() => (
            #fname
        )}
    })
}

