fn main() {
    let s: String = String::from("hello");
    let bytes: &[u8] = s.as_bytes();

    println!("{:?}", bytes);
}
