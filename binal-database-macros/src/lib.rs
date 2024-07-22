use quote::quote;

use syn::parse_macro_input;
use syn::DeriveInput;

use proc_macro::TokenStream;
use syn::{Field, Ident, Type};

#[proc_macro_derive(Object, attributes(id))]
pub fn derive_object(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let name_string = name.to_string();

    let structure = match ast.data {
        syn::Data::Struct(s) => s,
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => todo!(),
    };

    let mut id: Option<&Field> = None;

    let mut names = Vec::<&Ident>::new();
    let mut types = Vec::<&Type>::new();

    for field in &structure.fields {
        for attr in &field.attrs {
            if let Some(ident) = attr.path().get_ident() {
                if ident.to_string() == "id" {
                    id = Some(field);
                }
            }
        }

        names.push(field.ident.as_ref().unwrap());
        types.push(&field.ty);
    }

    if id.is_none() {
         return syn::Error::new(
        ast.ident.span().into(),
            "Exactly one ID attibute must be defined in an Object",
        )
        .to_compile_error()
        .into();
    }

    let column_names = names.iter().map(|name| name.to_string());
    let column_types = types.iter();

    let id_name = id.as_ref().unwrap().ident.as_ref().unwrap();
    let id_type = &id.as_ref().unwrap().ty;

    let code = quote! {
        impl crate::traits::Object<#id_type> for #name {
            const NAME: &'static str = #name_string;

            const COLUMN_NAMES: &'static [&'static str] = &[#(#column_names),*];
            const COLUMN_TYPES: &'static [&'static str];

            fn id<'a>(&'a self) -> &#id_type {
                &self.#id_name
            }
        }
    };

    code.into()
}

#[proc_macro]
pub fn generate_upsert(input: TokenStream) -> TokenStream {

}

