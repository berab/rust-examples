// The fact that there is no NULL value in rust, makes it good for mem. management

fn main() {
    let v = vec![42; 10];
    
    println!("{:?}", v);
    
 
    // let noner = v.get(11);
    // let inner = v[5];
    // let noinner = v[11];
 
    let mut some = v.get(9);

    println!("{:?}", some);
    
    // if let Some(inner) = some { // if some is NOT None, gets in
    //     println!("{:?}, {:?}, YEAH!", some, inner)
    // }


    // if some.is_none() {
    //     println!("No meaning of life yet.");
    //     return;
    // }
    
    // let inner = match some {
    //     Some(inner) => inner,
    //     None => &0,
    // };
    
    // let inner = some.unwrap(); // does the same thing above but gives a problem in None case so we return before that happens (line 24)
    // let inner = some.unwrap_or(&0); // if none, gives &0 (line 29)
    // let inner = some.take(); // takes the inside to inner and makes some a None. (takes what some has xd)
    // println!("{:?}, {:?}", inner, some);

    // let inner = some.map(|v| v*2);
    let inner = some.filter(|v| *v % 3 == 0); // if dividable by 3, Some(inner) else None

    some.expect("to find the meaning of life in the details of the Rust language");
    println!("{:?}", inner);
}