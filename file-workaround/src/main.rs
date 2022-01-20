use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

struct Info {
    name: String,
    age: i32,
    rating: i32,
}

fn main()  {
    write_this();
    write_that();

    let user1 = Info {
        name: String::from("beran"),
        age: 25,
        rating: 9999,
    };

    write_info(&user1);
    write_info_prop(&user1);
}

fn write_this() -> Result<()> {
    let mut file = File::create("foo1.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}

fn write_that() -> Result<()> {
    let mut file = File::create("foo2.txt").unwrap();
    if let Err(e) = file.write_all(b"Hello, world!") {
        return Err(e)
    }
    
    Ok(())

}

fn write_info(info: &Info) -> Result<()> {
    // Early return on error
    let mut file = match File::create("my_best_friends.txt") {
           Err(e) => return Err(e),
           Ok(f) => f,
    };
    if let Err(e) = file.write_all(format!("name: {}\n", info.name).as_bytes()) {
        return Err(e)
    }
    if let Err(e) = file.write_all(format!("age: {}\n", info.age).as_bytes()) {
        return Err(e)
    }
    if let Err(e) = file.write_all(format!("rating: {}\n", info.rating).as_bytes()) {
        return Err(e)
    }
    Ok(())
}

fn write_info_prop(info: &Info) -> Result<()> {
    let mut file = File::create("my_best_friends_prop.txt")?;
    // Early return on error
    file.write_all(format!("name: {}\n", info.name).as_bytes())?;
    file.write_all(format!("age: {}\n", info.age).as_bytes())?;
    file.write_all(format!("rating: {}\n", info.rating).as_bytes())?;
    Ok(())
}