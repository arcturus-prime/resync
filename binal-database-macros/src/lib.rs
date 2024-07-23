use quote::quote;

use syn::parse_macro_input;
use syn::DeriveInput;

use proc_macro::TokenStream;
use syn::{Field, Ident, Type};

fn convert_to_sql_type(type_: &Type) -> String {
    match type_ {
        Type::Array(_) => todo!(),
        Type::BareFn(_) => todo!(),
        Type::Group(g) => String::from("Deez"),
        Type::ImplTrait(_) => todo!(),
        Type::Infer(i) => String::from("djasd"),
        Type::Macro(_) => todo!(),
        Type::Never(_) => todo!(),
        Type::Paren(_) => todo!(),
        Type::Path(p) => String::from("what"),
        Type::Ptr(_) => todo!(),
        Type::Reference(_) => todo!(),
        Type::Slice(_) => todo!(),
        Type::TraitObject(_) => todo!(),
        Type::Tuple(_) => todo!(),
        Type::Verbatim(v) => v.to_string(),
        _ => String::from("whar"),
    }

}

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

    let mut id: Vec<&Field> = Vec::new();

    let mut names = Vec::<&Ident>::new();
    let mut types = Vec::<&Type>::new();

    for field in &structure.fields {
        let mut is_id = false;

        for attr in &field.attrs {
            if let Some(ident) = attr.path().get_ident() {
                if ident.to_string() == "id" {
                    id.push(field);
                    is_id = true;
                }
            }
        }

        if !is_id {
            names.push(field.ident.as_ref().unwrap());
            types.push(&field.ty);
        }
    }

    if id.len() != 1 {
        return syn::Error::new(
            ast.ident.span().into(),
            "Exactly one ID attibute must be defined in an Object",
        )
        .to_compile_error()
        .into();
    }

    let column_names = names.iter().map(|name| name.to_string());
    let column_types = types.iter().map(|type_| convert_to_sql_type(type_));

    let id_name = id[0].ident.as_ref().unwrap();
    let id_type = &id[0].ty;

    let id_name_string = id_name.to_string();
    let id_type_string = convert_to_sql_type(id_type);

    let row_range = (1..names.len()).into_iter();

    let code = quote! {
        impl crate::traits::Object<#id_type> for #name {
            type Row = (#(#types),*);

            const NAME: &'static str = #name_string;

            const ID_NAME: &'static str = #id_name_string;
            const ID_TYPE: &'static str = #id_type_string;

            const COLUMN_NAMES: &'static [&'static str] = &[#(#column_names),*];
            const COLUMN_TYPES: &'static [&'static str] = &[#(#column_types),*];

            fn to_row(&self) -> Self::Row {
                (self.#id_name, #(self.#names),*)
            }

            fn from_row(row: &Self::Row) -> Self {
                Self {
                    #id_name: row.0, #(#names: row.#row_range,)*
                }
            }

            fn id<'a>(&'a self) -> &#id_type {
                &self.#id_name
            }
        }
    };

    code.into()
}
