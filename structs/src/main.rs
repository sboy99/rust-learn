mod entities;


fn main() {
    let name: String = String::from("Sagar");
    let age: u32 = 30;
    let user: entities::User=  entities::User::new(name, age);
    println!("{:?}",user);
}
