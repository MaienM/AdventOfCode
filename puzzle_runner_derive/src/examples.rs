use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, format_ident, quote};
use syn::{
    Error, Expr, ItemStatic, Lit, LitStr, parse::Parser, parse_macro_input, parse_quote,
    spanned::Spanned,
};

use crate::utils::{ParseNestedMetaExt as _, args_struct, return_err};

args_struct! {
    struct Args {
        /// The indentation that should be stripped from the start of each line.
        indent: String = " ".repeat(8),
        /// The expected results for the parts.
        parts: HashMap<u8, Expr> = HashMap::new(),
        /// Whether to generate tests for the example.
        test: bool = true,
    }
}

struct ExampleStringParser<'a>(&'a str);
impl Parser for ExampleStringParser<'_> {
    type Output = String;

    fn parse2(self, tokens: TokenStream2) -> Result<Self::Output, Error> {
        let indent = self.0;

        let span = tokens.span();
        let text = syn::parse::<LitStr>(tokens.into())?.value();

        if !text.contains('\n') {
            return Ok(text);
        }

        let text = text
            .strip_prefix('\n')
            .ok_or_else(|| Error::new(span, "must begin with a newline"))?;
        let text = text
            .trim_end_matches(' ')
            .strip_suffix('\n')
            .ok_or_else(|| Error::new(span, "must end with a newline"))?;

        let mut lines = Vec::new();
        for line in text.split('\n') {
            lines.push(match line {
                "" => "",
                line => line.strip_prefix(indent).ok_or_else(|| {
                    Error::new(
                        span,
                        format!("non-empty line doesn't start with indent ({indent:?}): {line:?}"),
                    )
                })?,
            });
        }
        let text = lines.join("\n");

        Ok(text)
    }
}

macro_rules! parse_string_expr {
    ($expr:expr, $indent:expr) => {
        {
            let parser = ExampleStringParser($indent);
            let expr = ($expr).to_token_stream().into();
            parse_macro_input!(expr with parser)
        }
    };
}

macro_rules! parse_part_arg {
    ($expr:expr, $indent:expr) => {
        {
            let part: &Expr = $expr;
            match part {
                Expr::Lit(lit) => match &lit.lit {
                    Lit::Str(lit) => {
                        let string = parse_string_expr!(lit, $indent);
                        parse_quote!(#string)
                    }
                    Lit::Int(lit) => {
                        let num = lit.base10_digits();
                        parse_quote!(#num)
                    }
                    Lit::Float(lit) => {
                        let num = lit.base10_digits();
                        parse_quote!(#num)
                    }
                    lit => {
                        parse_quote!(stringify!(#lit))
                    }
                },
                _ => panic!("Part solution {part:?} cannot be converted to a static string."),
            }
        }
    };
}

pub fn example_input(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let mut builder = Args::build();
    let args_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("indent") {
            let value = match meta.value()?.parse::<Lit>()? {
                Lit::Str(indent) => Ok(indent.value()),
                Lit::Int(n) => Ok(" ".repeat(n.base10_parse()?)),
                _ => Err(meta.error("unsupported value, must be either a string or an integer")),
            }?;
            meta.set_empty_option(&mut builder.indent, value)?;
        } else if meta.path.is_ident("notest") {
            meta.set_empty_option(&mut builder.test, false)?;
        } else if let Some(ident) = meta.path.get_ident()
            && let Some(num) = ident.to_string().strip_prefix("part")
            && let Ok(num) = num.parse()
        {
            if builder.parts.is_none() {
                builder.parts = Some(HashMap::new());
            }
            builder
                .parts
                .as_mut()
                .unwrap()
                .insert(num, meta.value()?.parse()?);
        } else {
            return Err(meta.error("unsupported property"));
        }
        Ok(())
    });
    parse_macro_input!(input with args_parser);
    let args = return_err!(builder.finalize());

    let mut example = parse_macro_input!(annotated_item as ItemStatic);
    if example.ty != parse_quote!(&str) {
        return Error::new(example.ty.span(), "must be of type &str")
            .to_compile_error()
            .into();
    }
    {
        let result_indent = format!("{}    ", args.indent);

        let name = example.ident.to_string();
        let input = parse_string_expr!(example.expr, &args.indent);
        let parts = {
            let mut parts = Vec::new();
            for (num, part) in &args.parts {
                let part: Expr = parse_part_arg!(part, &result_indent);
                parts.push(quote!(map.insert(#num, #part);));
            }
            parts
        };
        *example.expr = parse_quote! {
            ::std::sync::LazyLock::new(||
                ::puzzle_runner::derived::Example {
                    name: #name,
                    input: #input,
                    parts: {
                        let mut map = ::std::collections::HashMap::new();
                        #(#parts)*
                        map
                    },
                }
            )
        };
        *example.ty = parse_quote!(::std::sync::LazyLock<::puzzle_runner::derived::Example>);
    };

    let mut result = quote!(#example);

    if args.test {
        for num in args.parts.keys() {
            let var_ident = &example.ident;
            let fn_ident = format_ident!("{}_part{}", var_ident.to_string().to_lowercase(), num);
            let part = format_ident!("part{num}");
            result.extend(quote! {
                #[cfg(test)]
                #[test]
                fn #fn_ident() {
                    assert_eq!(#part(#var_ident.input).to_string(), #var_ident.parts[&#num]);
                }
            });
        }
    }

    result.into()
}
