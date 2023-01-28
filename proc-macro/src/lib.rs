#[allow(unused_imports)]
use {
    ::core::ops::Not as _,
    ::proc_macro::{TokenStream, *},
};

#[proc_macro_attribute]
pub fn named(params: TokenStream, input: TokenStream) -> TokenStream {
    named_impl(params.into(), input.into())
        .unwrap_or_else(|err| {
            let err = Some(TokenTree::from(Literal::string(err)));
            quote!(::core::compile_error! { #err })
        })
        .into()
}

fn named_impl(params: TokenStream, input: TokenStream) -> Result<TokenStream, &'static str> {
    // parse::Nothing for `params`.
    if let Some(_) = params.into_iter().next() {
        return Err("unexpected attribute arguments".into());
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
            break Some(TokenTree::from(Literal::string(&fname)));
        }
        input.push(tt);
    };

    let g = match input.last_mut() {
        Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => g,
        _ => return Err("expected a `fn`"),
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

/// Mini `quote!` implementation,
/// can only interpolate `impl IntoIterator<Item = TokenTree>`.
macro_rules! quote_ {
    (
        @$q:tt
        { $($code:tt)* } $($rest:tt)*
    ) => (
        $q.push(
            TokenTree::Group(Group::new(
                Delimiter::Brace,
                quote!($($code)*)
            ))
        );
        quote!(@$q $($rest)*);
    );

    (
        @$q:tt
        [ $($code:tt)* ]
        $($rest:tt)*
    ) => (
        $q.push(
            TokenTree::Group(Group::new(
                Delimiter::Bracket,
                quote!($($code)*)
            ))
        );
        quote!(@$q $($rest)*);
    );

    (
        @$q:tt
        ( $($code:tt)* )
        $($rest:tt)*
    ) => (
        $q.push(
            TokenTree::Group(Group::new(
                Delimiter::Parenthesis,
                quote!($($code)*)
            ))
        );
        quote!(@$q $($rest)*);
    );

    (
        @$q:tt
        #$var:ident
        $($rest:tt)*
    ) => (
        $q.extend($var);
        quote!(@$q $($rest)*);
    );

    (
        @$q:tt
        $tt:tt $($rest:tt)*
    ) => (
        $q.extend(
            stringify!($tt)
                .parse::<TokenStream>()
                .unwrap()
        );
        quote!(@$q $($rest)*);
    );

    (
        @$q:tt
        /* nothign left */
    ) => ();

    (
        $($code:tt)*
    ) => ({
        let mut _q = Vec::<TokenTree>::new();
        quote!(@_q $($code)*);
        _q.into_iter().collect::<TokenStream>()
    });
}
use quote_ as quote;
