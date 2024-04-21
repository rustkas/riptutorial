struct Point {
    x: i32,
    y: i32,
}

// This gives an error!
// enum List {
//     Nil,
//     Cons(i32, List)
// }

#[allow(dead_code)]
enum List {
    Nil,
    Cons(i32, Box<List>)
}


fn main() {
    let boxed_vec = Box::new(vec![1, 2, 3]);
    println!("{}", boxed_vec.get(0).unwrap_or(&0));

    let boxed_point = Box::new(Point { x: 0, y: 0 });
    // Notice the *. That dereferences the boxed value into just the value
    match *boxed_point {
        Point { x, y } => println!("Point is at ({}, {})", x, y),
    }
}
