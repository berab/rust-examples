fn reverse(input: &str) -> String {
    // let mut new_str = String::from("");
    // for n in (0..input.chars().count()).rev(){
    //     new_str.push(input.chars().nth(n).unwrap());
    //     // println!("sa {}", new_str);
    // }
    // return new_str
    input.rsplit("").collect()
}

fn main() {
    let my_str = String::from("selamin aleykum adim azrail");
    println!("{}",my_str.chars().nth(26).unwrap());
    println!("{}",my_str.chars().count());
    println!("{}", reverse(&my_str));
}
