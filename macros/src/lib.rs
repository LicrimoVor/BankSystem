use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
mod from_sql;
mod to_sql;
mod transaction;

/// Макрос say_hello
///
/// Пример использования:
/// ```rust
/// use macros::say_hello;
///
/// say_hello!("world!");
/// >> Hello world!
/// ```
#[proc_macro]
pub fn say_hello(input: TokenStream) -> TokenStream {
    let msg = parse_macro_input!(input as syn::LitStr); // ожидаем строковый литерал
    let expanded = quote! {
        println!("{}", #msg);
    };
    expanded.into()
}

/// Макрос ToSql
///
/// Пример использования:
/// ```rust
/// use macros::ToSql;
///
/// #[derive(Debug, ToSql, PartialEq)]
/// struct User {
///    id: i32,
///    name: String,
///    age: i32,
/// }
///
/// let user = User {
///     id: 1,
///     name: "Alice".into(),
///     age: 30,
/// };
/// assert_eq!(user.to_sql("users"), "INSERT INTO users (id, name, age) VALUES ('1', 'Alice', '30');");
/// ```
#[proc_macro_derive(ToSql)]
pub fn to_sql_derive(input: TokenStream) -> TokenStream {
    to_sql::to_sql_derive(input)
}

/// Макрос FromSql
///
/// Пример использования:
/// ```rust
/// use macros::FromSql;
///
/// #[derive(Debug, FromSql, PartialEq)]
/// struct User {
///    id: i32,
///    name: String,
///    age: i32,
/// }
///
/// let user = User::from_sql("INSERT INTO users (id, name,age, status) VALUES('1','Bob','35', 'Offline');");
/// assert_eq!(user, User { id: 1, name: "Bob".into(), age: 35 });
/// ```
#[proc_macro_derive(FromSql)]
pub fn from_sql_derive(input: TokenStream) -> TokenStream {
    from_sql::from_sql_derive(input)
}

#[proc_macro_derive(Transaction, attributes(transaction))]
pub fn transaction_derive(input: TokenStream) -> TokenStream {
    transaction::transaction_derive(input)
}
