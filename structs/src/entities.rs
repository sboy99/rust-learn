use std::fmt;

// -- USER ENTITY -- //

#[allow(dead_code)]
pub struct User {
    name: String,
    age: u32,
}

#[allow(dead_code)]
impl User {
    pub fn new(name: String, age: u32) -> User {
        User {
            name,
            age,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_age(&self) -> u32 {
        self.age
    }
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "USER\n Name: {}\n Age: {}", self.name, self.age)
    }
    
}