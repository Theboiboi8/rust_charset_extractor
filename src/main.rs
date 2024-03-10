fn main() {
    let input : String = std::fs::read_to_string("extract_from.txt")
        .expect("Error reading file");

    println!("Found charset {:#?}", extract_charset(input.as_str()));
}

fn extract_charset(input: &str) -> String {
    let mut charset : String = String::new();

    for character in input.chars() {
        if !charset.contains(character) {
            charset.push(character);
        }
    }

    charset
}