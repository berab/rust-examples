use std::mem::drop;

fn main() {
    let v = vec![1, 2, 3];
    println!("{:?}", v);
    drop(v); // explicitly drop the vector    
}
