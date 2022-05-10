#[allow(unused_imports)]
use {
    ::core::{
        ops::Not as _,
    },
    ::proc_macro::{
        TokenStream,
    },
    ::proc_macro2::{*,
        Span,
        TokenStream as TokenStream2,
    },
    ::quote::{
        quote,
        quote_spanned,
        ToTokens,
    },
    ::syn::{*,
        Result, // explicitly shadow it
    },
};

#[proc_macro_attribute] pub
fn named (
    params: TokenStream,
    input: TokenStream,
) -> TokenStream
{
    named_impl(params.into(), input.into())
        .unwrap_or_else(|err| {
            let mut errors =
                err .into_iter()
                    .map(|err| Error::new(
                        err.span(),
                        format_args!("`#[function_name::named]`: {}", err),
                    ))
            ;
            let mut err = errors.next().unwrap();
            errors.for_each(|cur| err.combine(cur));
            err.to_compile_error()
        })
        .into()
}

fn named_impl (
    params: TokenStream2,
    input: TokenStream2,
) -> Result<TokenStream2>
{
    // parse::Nothing for `params`.
    if let Some(tt) = params.into_iter().next() {
        return Err(Error::new(tt.span(), "unexpected attribute arguments"));
    }

    let ref mut tts = input.into_iter().peekable();

    let mut input = Vec::<TokenTree>::new();

    // `#` `[…]` attributes:
    while matches!(tts.peek(), Some(TokenTree::Punct(p)) if p.as_char() == '#') {
        input.extend(tts.take(2));
    }

    // rest but scan the tt right after `fn`.
    let fname = loop {
        let tt = tts.next().unwrap();
        if matches!(tt, TokenTree::Ident(ref ident) if ident.to_string() == "fn") {
            input.push(tt);
            let fname = tts.peek().unwrap().to_string();
            input.extend(tts);
            break fname;
        }
        input.push(tt);
    };

    let g = match input.last_mut() {
        | Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => g,
        | _ => return Err(Error::new(Span::call_site(), "expected a `fn`")),
    };
    let g_span = g.span();
    *g = Group::new(g.delimiter(), {
        let mut body = quote!(
            macro_rules! function_name {() => (
                #fname
            )}
        );
        body.extend(g.stream());
        body
    });
    g.set_span(g_span);
    Ok(input.into_iter().collect())
}
