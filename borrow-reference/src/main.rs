fn main() {
    let mut s1 = String::from("Some Sentence");
    let s2 = &mut s1;
    s2.push_str(" string");
    borrow(&mut s1);
    borrow(&mut s1);
    borrow(&mut s1);
    println!("Inside main: {}", s1);
}

fn borrow(borrowed: &mut String) {
    borrowed.push_str(" Hanky Panky");
    println!("Inside Borrow: {}", borrowed);
}
