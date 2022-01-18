fn main() {
    for i in 1..=100{
        let a = i.to_string();
        let to_print = match(i % 3 == 0, i% 5 == 0) {
            (true, true) => "fizzbuzz",
            (true, false) => "fizz",
            (false, true) => "buzz",
            (false, false) => &a,
        };

        println!("{}", to_print);
    }
}