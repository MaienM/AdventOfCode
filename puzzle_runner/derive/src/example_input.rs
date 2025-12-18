use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, format_ident, quote};
use syn::{
    Error, Expr, ItemStatic, Lit, LitStr, meta::ParseNestedMeta, parse::Parser, parse_macro_input,
    parse_quote, spanned::Spanned,
};

use crate::utils::{ParseNestedMetaExt as _, args_struct, return_err};

args_struct! {
    struct Args {
        /// The indentation that should be stripped from the start of each line.
        indent: String = default " ".repeat(8),
        /// The settings for the parts.
        parts: HashMap<u8, PartArgsBuilder> = initial HashMap::new(),
        /// Whether to generate tests for the example.
        test: bool = default true,
    }
    struct PartArgs {
        /// The expected result for this part.
        expected: String,
        /// The additional argument to pass in when processing the example input.
        arg: Option<Expr> = initial None,
    }
}

fn dedent(text: String, indent: &str) -> Result<String, String> {
    if !text.contains('\n') {
        return Ok(text);
    }

    let text = text
        .strip_prefix('\n')
        .ok_or_else(|| "must begin with a newline".to_owned())?;
    let text = text
        .trim_end_matches(' ')
        .strip_suffix('\n')
        .ok_or_else(|| "must end with a newline".to_owned())?;

    let mut lines = Vec::new();
    for line in text.split('\n') {
        lines.push(match line {
            "" => "",
            line => line.strip_prefix(indent).ok_or_else(|| {
                format!("non-empty line doesn't start with indent ({indent:?}): {line:?}")
            })?,
        });
    }
    Ok(lines.join("\n"))
}

struct ExampleStringParser<'a>(&'a str);
impl Parser for ExampleStringParser<'_> {
    type Output = String;

    fn parse2(self, tokens: TokenStream2) -> Result<Self::Output, Error> {
        let indent = self.0;

        let span = tokens.span();
        let text = syn::parse::<LitStr>(tokens.into())?.value();

        dedent(text, indent).map_err(|e| Error::new(span, e))
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

fn parse_indent(meta: &ParseNestedMeta) -> Result<String, Error> {
    match meta.value()?.parse::<Lit>()? {
        Lit::Str(indent) => Ok(indent.value()),
        Lit::Int(n) => Ok(" ".repeat(n.base10_parse()?)),
        _ => Err(meta.error("unsupported value, must be either a string or an integer")),
    }
}

fn parse_args(input: TokenStream) -> Result<Args, String> {
    let mut builder = Args::build();
    let args_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("indent") {
            meta.set_empty_option(&mut builder.indent, parse_indent(&meta)?)?;
        } else if meta.path.is_ident("notest") {
            meta.set_empty_option(&mut builder.test, false)?;
        } else if let Some(ident) = meta.path.get_ident()
            && let Some(num) = ident.to_string().strip_prefix("part")
            && let Ok(num) = num.parse()
        {
            let mut part = PartArgs::build();
            part.expected = Some(meta.parse_stringify()?);
            builder.parts.insert(num, part);
        } else if let Some(first) = meta.path.segments.first()
            && first.arguments.is_empty()
            && let Some(num) = first.ident.to_string().strip_prefix("part")
            && let Ok(num) = num.parse::<u8>()
        {
            let part = builder
                .parts
                .get_mut(&num)
                .ok_or_else(|| meta.error(format!("must appear after part{num} argument")))?;
            let name = if meta.path.segments.len() == 2
                && let Some(ident) = meta.path.segments.last()
                && ident.arguments.is_empty()
            {
                ident.ident.to_string()
            } else {
                return Err(meta.error("unsupported property"));
            };
            match name.as_str() {
                "arg" => {
                    meta.set_empty_option(&mut part.arg, meta.value()?.parse()?)?;
                }
                _ => {
                    return Err(meta.error("unsupported property"));
                }
            }
        } else {
            return Err(meta.error("unsupported property"));
        }
        Ok(())
    });
    args_parser.parse(input).map_err(|e| e.to_string())?;
    builder.finalize()
}

pub fn main(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let args = return_err!(parse_args(input));
    let mut parts = HashMap::new();
    for (num, part) in args.parts {
        let mut part = return_err!(part.finalize());
        part.expected = return_err!(dedent(part.expected, &format!("{}    ", args.indent)));
        parts.insert(num, part);
    }

    let mut example = parse_macro_input!(annotated_item as ItemStatic);
    if example.ty != parse_quote!(&str) {
        return Error::new(example.ty.span(), "must be of type &str")
            .to_compile_error()
            .into();
    }
    {
        let name = example.ident.to_string();
        let input = parse_string_expr!(example.expr, &args.indent);
        let parts: Vec<_> = parts
            .iter()
            .map(|(num, part)| {
                let expected = &part.expected;
                quote!(map.insert(#num, #expected);)
            })
            .collect();
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
        for (num, part) in parts {
            let var_ident = &example.ident;
            let fn_ident = format_ident!("{}_part{}", var_ident.to_string().to_lowercase(), num);
            let partident = format_ident!("part{num}");
            let arg = match part.arg {
                Some(arg) => quote!(, #arg),
                None => quote!(),
            };
            result.extend(quote! {
                #[cfg(test)]
                #[test]
                fn #fn_ident() {
                    assert_eq!(#partident(#var_ident.input #arg).to_string(), #var_ident.parts[&#num]);
                }
            });
        }
    }

    result.into()
}
