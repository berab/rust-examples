use std::collections::HashMap;

fn main() {
    let mut letters = HashMap::new();

    for ch in "some text".chars() {
        let counter = letters.entry(ch).or_insert(0); // check if there is one already (using .entry() -> pointer), else create one with 0 (it will be 1 using inc. the pointer after)
        *counter += 1; // increasing by using the pointer
    }

    println!("{:?}", letters)
}
