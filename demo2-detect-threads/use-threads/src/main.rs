fn consume(_: String) {}

fn main() {
    let input = String::from("abcdefg");
    let _ = std::thread::spawn(move || {
        consume(input);
    });
}
