

fn main() {
    let x = u16::from(13u8); // visa versa doesnt work (overflow etc.). can also be done with as u16 (13u8 as u16)
    assert_eq!(x, 13u16);

    let a: i32 = 3;
    let b: f64 = 4.;

    let sum = add(a, b);
    println!("{}", sum);
}

// fn add<T, U>(a:T, b:U) -> f64 
// where
//     T: Into<f64>,
//     U: Into<f64>
// {
//     a.into() + b.into()
// }

// OR

fn add<T, U>(a:T, b:U) -> f64 
where
    f64: From<T> + From<U>
{
    f64::from(a) + f64::from(b)
}