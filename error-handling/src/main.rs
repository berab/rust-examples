use std::fs::File;
use std::io::{Read, ErrorKind, Error};

// fn exit(x: i32) {
//     if x == 0 {
//         panic!("we got a 0"); // shut down the program. might be useful in some spesific cases
//     }
//     println!("things are fine!");
// }


// fn exit(x: Option<i32>) {
//     match x {
//         Some(0) => panic!("we got a 0"),
//         Some(x) => panic!("we got a {}, which is fine.", x),
//         None => println!("we got nothing, better than a 0 tho..."),
//     }
// }

fn read_file() -> Result<String, Error> {
    let f = File::open("text.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(& mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main() {

    // exit(Some(1));
    // exit(None);
    // exit(Some(0));

    // let f = File::open("text.txt"); // gives us a result as output
    let f = File::open("text.txt").expect("Could not open file");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => {
    //         match File::create("text.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => {
    //                 panic!("could not create file: {:?}",
    //                 e
    //                 )
    //             },
    //     },
    //     Err(error) => {
    //         panic!(
    //             "could not open the file: {:?}",
    //             error
    //         )
    //     },
    // };
}
