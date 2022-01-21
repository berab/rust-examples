use std::fmt::{Formatter, Display, Result};

struct Greeting {
    name: String,
}

impl Greeting {
    fn new<T: AsRef<str>>(name: T) -> Self { // this part is interesting. to accept both a slice and string. otherwise we can accept only a slice or string. 
        Greeting {                              // cannot be just T since as_ref() doest accept anything else than string/slice so does not compile
            name: name.as_ref().to_string(),
        }
    }
}

impl Display for Greeting {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Hallo, {}!", self.name)
    }
}

fn main() {
    let greeting = Greeting::new("Linz".to_string());
    println!("{}", greeting)
}
