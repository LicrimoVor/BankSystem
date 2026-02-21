// src/main.rs

include!("./bindgen.rs");

fn main() {
    println!("Squared eleven: {}", unsafe { square(11) });
}
