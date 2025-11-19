use macros::{FromSql, ToSql, say_hello};

macro_rules! say {
    ($message:expr) => {
        println!("{}", $message);
    };
}

#[derive(ToSql, Debug, FromSql)]
struct User {
    id: i32,
    name: String,
    age: i32,
}

fn main() {
    say_hello!("Привет из процедурного макроса!");

    say!("Привет, мир!");
    say!("Сегодня мы учим макросы в Rust <3");

    let user = User {
        id: 1,
        name: "Alice".into(),
        age: 30,
    };
    println!("{}", user.to_sql("users"));
    let user2 = User::from_sql(
        r"INSERT INTO users (id, name,age, status) VALUES('1','Bob','35', 'Offline');",
    );

    print!("{:?}", user2);
}
