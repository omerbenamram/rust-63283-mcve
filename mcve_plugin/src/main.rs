#![feature(plugin)]
#![plugin(string_encryption_plugin)]

fn main() {
    let x = e!("hello");
    println!("{}", x);
}
