use proc_macro2::{TokenStream, TokenTree};
use quote::quote;
use syn::{Attribute, ExprClosure, Field, Ident, Item, ItemStruct, LitStr, parse_macro_input, Token, Type};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use command_parser::ParseError;

fn get_fields(d: &syn::ItemStruct) -> syn::Result<&Punctuated<Field, Token![,]>> {
    if let syn::ItemStruct {
                                 fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
                                 ..
                             } = d {
        return Ok(named)
    }
    Err(syn::Error::new_spanned(d, "Must define on a Struct, not Enum".to_string()))
}

fn generate_builder_struct_fields_def(fields: &Punctuated<Field, Token![,]>) -> syn::Result<TokenStream>{
    let idents:Vec<_> = fields.iter().map(|f| {&f.ident}).collect();
    let types:Vec<_> = fields.iter().map(|f| {&f.ty}).collect();

    Ok(quote!(
        #(#idents: #types::from_str("")),*
    ).into())
}

#[proc_macro_attribute]
pub fn command(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = parse_macro_input!(attr as Attribute);
    eprintln!("{:#?}", args);


    let st = parse_macro_input!(item as ItemStruct);
    eprintln!("{:#?}", st);

    let st_ident = &st.ident;

    let fields = get_fields(&st).unwrap();
    let builder_struct_fields_def = generate_builder_struct_fields_def(fields).unwrap();


    quote!(
        #st

        impl FromStr for #st_ident {
            type Err = command_parser::ParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                #builder_struct_fields_def
            }
        }
    ).into()
}
