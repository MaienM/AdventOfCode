use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, format_ident, quote};
use syn::{
    Error, Expr, ItemStatic, Lit, LitStr, parse::Parser, parse_macro_input, parse_quote,
    spanned::Spanned,
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
            test: true,
        }
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
            let part: &Option<Expr> = $expr;
            match part {
                Some(Expr::Lit(lit)) => match &lit.lit {
                    Lit::Str(lit) => {
                        let string = parse_string_expr!(lit, $indent);
                        parse_quote!(Some(#string))
                    }
                    Lit::Int(lit) => {
                        let num = lit.base10_digits();
                        parse_quote!(Some(#num))
                    }
                    Lit::Float(lit) => {
                        let num = lit.base10_digits();
                        parse_quote!(Some(#num))
                    }
                    lit => {
                        parse_quote!(Some(stringify!(#lit)))
                    }
                },
                Some(_) => panic!("Part solution {part:?} cannot be converted to a static string."),
                None => parse_quote!(None),
            }
        }
    };
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
                    );
                }
            }
        } else if meta.path.is_ident("part1") {
            args.part1 = Some(meta.value()?.parse()?);
        } else if meta.path.is_ident("part2") {
            args.part2 = Some(meta.value()?.parse()?);
        } else if meta.path.is_ident("notest") {
            args.test = false;
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
        let result_indent = format!("{}    ", args.indent);

        let name = example.ident.to_string();
        let input = parse_string_expr!(example.expr, &args.indent);
        let part1: Expr = parse_part_arg!(&args.part1, &result_indent);
        let part2: Expr = parse_part_arg!(&args.part2, &result_indent);
        *example.expr = parse_quote! {
            ::puzzle_runner::derived::Example {
                name: #name,
                input: #input,
                part1: #part1,
                part2: #part2,
            }
        };
        *example.ty = parse_quote!(::puzzle_runner::derived::Example);
    };

    let mut result = quote!(#example);

    if args.test {
        for (part, expr) in [("part1", &args.part1), ("part2", &args.part2)] {
            if expr.is_some() {
                let var_ident = &example.ident;
                let fn_ident = format_ident!("{}_{}", var_ident.to_string().to_lowercase(), part);
                let part = format_ident!("{part}");
                result.extend(quote! {
                    #[cfg(test)]
                    #[test]
                    fn #fn_ident() {
                        assert_eq!(#part(#var_ident.input).to_string(), #var_ident.#part.unwrap());
                    }
                });
            }
        }
    }

    result.into()
}
