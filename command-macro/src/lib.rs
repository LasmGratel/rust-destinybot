#![feature(proc_macro_diagnostic)]

use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{Ident, LitStr, parse_macro_input, Token, Type};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

struct Command {
    ident: Ident,
    name: LitStr,
    fields: Punctuated<Type, Token![,]>,
}

impl Parse for Command {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;
        let name = input.parse::<LitStr>()?;

        eprintln!("{:?}", input);
        Ok(Command {
            ident,
            name,
            fields: input.parse_terminated(Type::parse)?,

        })
    }
}

#[proc_macro]
pub fn command(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = parse_macro_input!(input as Command);

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &Command) -> TokenStream {
    let ident = &ast.ident;
    let name = &ast.name;
    let fields = &ast.fields;

    let field_parsing = fields.iter().map(|f| {
        quote! {
            println!("ggg");
            (#f).from_str("ggg");
        }
    });

    let gen = quote! {
        struct #ident;

        impl FromStr for #ident {
            type Err = ParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                println!("fff");
                #(#field_parsing),*

                Ok(Self {})
            }
        }
    };
    gen.into()
}