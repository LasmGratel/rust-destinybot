#![feature(proc_macro_diagnostic)]

use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{ExprClosure, Ident, LitStr, parse_macro_input, Token, Type};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use command_parser::ParseError;

struct Command {
    ident: Ident,
    name: LitStr,
    fields: Punctuated<Type, Token![,]>,
    on_call: ExprClosure,
}

impl Parse for Command {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;
        let name = input.parse::<LitStr>()?;

        Ok(Command {
            ident,
            name,
            on_call: input.parse::<ExprClosure>()?,
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
            #f::from_str("blabla").map_err(|e| command_parser::ParseError::Argument(e.into()))?;
        }
    });


    let gen = quote! {
        struct #ident;

        impl FromStr for #ident {
            type Err = command_parser::ParseError;

            fn from_str(s: &str) -> Result<Self, command_parser::ParseError> {
                println!(#name);
                #(#field_parsing)*

                Ok(Self {})
            }
        }
    };

    gen.into()
}