use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

// const REGEX_FILEDS: &'static str = r"INSERT INTO (.*?) \s*\((.*?)\)";
const REGEX_VALS: &str = r"VALUES\s*\((.*?)\)";

/// Реализация макроса `#[derive(FromSql)]`
pub fn from_sql_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Собираем поля структуры
    let fields = if let syn::Data::Struct(data) = &input.data {
        data.fields
            .iter()
            .map(|f| f.ident.clone().unwrap())
            .collect::<Vec<_>>()
    } else {
        panic!("FromSql can only be derived for structs");
    };

    // Генерируем код с итератором по значениям
    let assigns = fields.iter().map(|f| {
        quote! {
            #f: vals.next().unwrap().parse().expect("Cannot parse field"),
        }
    });

    let expanded = quote! {
        use regex::Regex;

        impl #name {
            pub fn from_sql(row: &str) -> Self{
                // let re = Regex::new(#REGEX_FILEDS).unwrap();
                // let fields = re.captures(row).unwrap().get(2).unwrap().as_str();

                let re = Regex::new(#REGEX_VALS).unwrap();
                let vals = re.captures(row).unwrap().get(1).unwrap().as_str();
                let mut vals = vals
                    .split(',')
                    .map(|v| v.trim().trim_matches('\''));

                Self {
                    #(#assigns)*
                }
            }
        }
    };

    // println!("{}", expanded);

    TokenStream::from(expanded)
}
