fn main() {
    let mut s1: String = String::from("Some Sentence");
    s1 = give_ownership(s1);
    println!("Inside main: {}", s1)
}

fn give_ownership(str: String) -> String {
    println!("Inside give_ownership: {}", str);
    return str;
}
