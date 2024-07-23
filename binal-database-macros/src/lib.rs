use quote::quote;

use syn::parse_macro_input;
use syn::DeriveInput;

use proc_macro2::TokenStream;
use syn::{Field, Ident, Type};

fn convert_to_sql_type(type_: &Type) -> String {
    match type_ {
        Type::Path(p) => {
            String::from("deez")
        },
        _ => todo!(),
    }

}

#[proc_macro_derive(Object, attributes(id, bitcode, time))]
pub fn derive_object(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let name_string = name.to_string();

    let structure = match ast.data {
        syn::Data::Struct(s) => s,
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => todo!(),
    };

    let mut id: Vec<&Field> = Vec::new();

    let mut ser_names = Vec::<&Ident>::new();

    let mut names = Vec::<&Ident>::new();
    let mut types = Vec::<&Type>::new();

    for field in &structure.fields {
        let mut add_to_normal = true;

        for attr in &field.attrs {
            if let Some(ident) = attr.path().get_ident() {
                if ident.to_string() == "id" {
                    id.push(field);
                    add_to_normal = false;
                }
                if ident.to_string() == "bitcode" {
                    ser_names.push(field.ident.as_ref().unwrap());
                    add_to_normal = false;
                }
            }
        }

        if add_to_normal {
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

    let column_ser_names = ser_names.iter().map(|name| name.to_string());
    let column_ser_types = ser_names.iter().map(|_| "BLOB" );

    let id_name = id[0].ident.as_ref().unwrap();
    let id_type = &id[0].ty;

    let id_name_string = id_name.to_string();
    let id_type_string = convert_to_sql_type(id_type);

    let ser_types: TokenStream = (0..ser_names.len()).map(|_| { quote!(Vec<u8>,) }).collect();

    let ser_names_row_range = (1..=ser_names.len()).map(|i| syn::Index::from(i));
    let names_row_range = (1 + ser_names.len()..=ser_names.len() + names.len()).map(|i| syn::Index::from(i));

    let code = quote! {
        impl crate::object::Object for #name {
            type Row = (#id_type, #ser_types #(#types),*);
            type Index = #id_type;

            const NAME: &'static str = #name_string;

            const ID_NAME: &'static str = #id_name_string;
            const ID_TYPE: &'static str = #id_type_string;

            const COLUMN_NAMES: &'static [&'static str] = &[#(#column_ser_names,)* #(#column_names),*];
            const COLUMN_TYPES: &'static [&'static str] = &[#(#column_ser_types,)* #(#column_types),*];

            fn to_row(self) -> Self::Row {
                (self.#id_name, #(bitcode::serialize(&self.#ser_names).unwrap(),)* #(self.#names),*)
            }

            fn from_row(row: Self::Row) -> Self {
                Self {
                    #id_name: row.0, #(#ser_names: bitcode::deserialize(&row.#ser_names_row_range).unwrap(),)* #(#names: row.#names_row_range,)*
                }
            }

            fn id<'a>(&'a self) -> &Self::Index {
                &self.#id_name
            }
        }
    };

    code.into()
}
