use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, Fields};
use syn::{DeriveInput, parse_macro_input};

pub fn to_sql_derive(input: TokenStream) -> TokenStream {
    // Парсим вход в proc_macro2 TokenStream
    let input: DeriveInput = parse_macro_input!(input);
    let name = input.ident;

    let (field_names, field_values): (Vec<_>, Vec<_>) = match input.data {
        Data::Struct(ref data) => match &data.fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .map(|f| {
                    let ident = f.ident.as_ref().unwrap();
                    (ident, quote! { self.#ident })
                })
                .unzip(),
            _ => panic!("ToSql can only be derived for structs with named fields"),
        },
        _ => panic!("ToSql can only be derived for structs"),
    };

    // Генерация кода с proc_macro2 + quote
    let expanded: TokenStream2 = quote! {
        impl #name {
            pub fn to_sql(&self, table: &str) -> String {
                let columns = vec![#(stringify!(#field_names)),*].join(", ");
                let values = vec![#(format!("'{}'", #field_values)),*].join(", ");
                format!("INSERT INTO {} ({}) VALUES ({});", table, columns, values)
            }
        }
    };

    println!("{expanded}",);

    // Преобразуем proc_macro2::TokenStream обратно в proc_macro::TokenStream
    TokenStream::from(expanded)
}
