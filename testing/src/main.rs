fn main() {
    let stra: &[u8] = &[65u8, 97u8, 98u8];
    println!(
        "Hello, world! {}",
        String::from_utf8(stra.to_vec()).unwrap()
    );
}
