use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::Parser, parse_macro_input, parse_quote, spanned::Spanned, Error, Expr, ItemStatic, Lit,
    LitStr,
};

struct Args {
    /// The indentation that should be stripped from the start of each line.
    indent: String,
    /// The expected result for part 1.
    part1: Option<Expr>,
    /// The expected result for part 1.
    part2: Option<Expr>,
    /// Whether to generate tests for the example.
    test: bool,
}
impl Default for Args {
    fn default() -> Self {
        Self {
            indent: " ".repeat(8),
            part1: None,
            part2: None,
            test: false,
        }
    }
}

struct ExampleStringParser<'a>(&'a str);
impl<'a> Parser for ExampleStringParser<'a> {
    type Output = String;

    fn parse2(self, tokens: TokenStream2) -> Result<Self::Output, Error> {
        let indent = self.0;

        let span = tokens.span();
        let text = syn::parse::<LitStr>(tokens.into())?.value();
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

fn get_part_args(part: &Option<Expr>) -> Expr {
    if let Some(expr) = part {
        parse_quote!(Some(stringify!(#expr)))
    } else {
        parse_quote!(None)
    }
}

pub fn example_input(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let mut args = Args::default();
    let args_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("indent") {
            match meta.value()?.parse::<Lit>()? {
                Lit::Str(indent) => args.indent = indent.value(),
                Lit::Int(n) => args.indent = " ".repeat(n.base10_parse()?),
                _ => {
                    return Err(
                        meta.error("unsupported value, must be either a string or an integer")
                    )
                }
            }
        } else if meta.path.is_ident("part1") {
            args.part1 = Some(meta.value()?.parse()?);
        } else if meta.path.is_ident("part2") {
            args.part2 = Some(meta.value()?.parse()?);
        } else if meta.path.is_ident("test") {
            args.test = true;
        } else {
            return Err(meta.error("unsupported property"));
        }
        Ok(())
    });
    parse_macro_input!(input with args_parser);

    let mut example = parse_macro_input!(annotated_item as ItemStatic);
    if example.ty != parse_quote!(&str) {
        return Error::new(example.ty.span(), "must be of type &str")
            .to_compile_error()
            .into();
    }
    {
        let parser = ExampleStringParser(&args.indent);
        let expr = example.expr.to_token_stream().into();
        let text = parse_macro_input!(expr with parser);

        let part1 = get_part_args(&args.part1);
        let part2 = get_part_args(&args.part2);
        *example.expr = parse_quote! {
            aoc::derived::Example {
                input: #text,
                part1: #part1,
                part2: #part2,
            }
        };
        *example.ty = parse_quote!(aoc::derived::Example);
    };

    let mut result = quote!(#example);

    if args.test {
        for (part, expr) in [("part1", &args.part1), ("part2", &args.part2)] {
            if let Some(expr) = expr {
                let ident = &example.ident;
                let lident = format_ident!("{}_{}", ident.to_string().to_lowercase(), part);
                let part = format_ident!("{part}");
                result.extend(quote! {
                    #[cfg(test)]
                    #[test]
                    fn #lident() {
                        assert_eq!(#part(#ident.input), #expr);
                    }
                })
            }
        }
    }

    result.into()
}
