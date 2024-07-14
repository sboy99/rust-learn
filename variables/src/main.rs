fn first_word(sentence: String) -> String {
    let mut ans: String = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;
}

fn main() {
    let sentence: String = String::from("Hello World!");
    let word: String = first_word(sentence);
    println!("{}", word);
}
