use std::io::Read;
use tiktoken_rs::cl100k_base;

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let bpe = cl100k_base().unwrap();
    let tokens = bpe.encode_with_special_tokens(&input);
    println!("{}", tokens.len());
}