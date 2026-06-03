use crate::tokenizer::Tokenizer;

mod tokenizer;

fn main() {
    let corpus = std::fs::read_to_string("corpus.txt").expect("Failed to load corpus");

    let tokenizer = Tokenizer::trainer(&corpus).train(100);

    let text = "This is a very interesting tokenizer! :D";
    let encoded = tokenizer.encode(text);
    let decoded = tokenizer.decode(&encoded);

    println!("{:?}", encoded);
    println!("{:?}", decoded);
}
