fn main() {
    println!("Type a sentence and press Enter.");

    let mut sentence = String::new();

    std::io::stdin()
        .read_line(&mut sentence)
        .expect("Failed to read line.");

    let sentence = sentence.trim();
    let mut index: usize = 0;
    for c in sentence.chars() {
        if c == ' ' {
            break;
        }
        index = index + 1;
    }

    println!("The first word has {} characters and is '{}'.", index, &sentence[..index]);
}
