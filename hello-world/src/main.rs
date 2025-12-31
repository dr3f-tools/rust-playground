fn main() {
    let msg = "Hello, world!";
    println!("{}", msg);

    let mut msg_mut = String::from("Hello, ");
    msg_mut.push_str("Rust!");
    println!("{}", msg_mut);
}
