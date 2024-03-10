fn main() {
    let input : String = std::fs::read_to_string("extract_from.txt")
    .expect("Error reading file");

    println!("Found charset {:?}", extract_charset(input));
}

fn extract_charset(input: String) -> String {
    let mut charset : Vec<&str> = Vec::new();

    for character in input.chars() {
        let char_as_string : String = String::from(character);
        let char_as_str : &str = char_as_string.as_str();

        if !charset.contains(&char_as_str) {
            charset.push(char_as_str);
        } else {
            println!("Found duplicate: {character}");
        }
    }

    let output : String = charset.into_iter().collect::<String>();

    output
}