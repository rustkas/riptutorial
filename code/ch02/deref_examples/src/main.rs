
fn f(x: &str) -> &str { x }

fn deref_string() {
    // Compiler will coerce &&&&&&&str to &str and then pass it to our function
   let string = f(&&&&&&&"It's a string");
   println!("{string}"); 
}

fn f2(x: &Vec<i32>) {
    println!("{:?}", x) // [1,2,3]
}

fn working_with_box_arc() {
    let val = Box::new(vec![1,2,3]);
    // Now, thanks to Deref, we still 
    // can use our vector method as if there wasn't any Box
    let folding_result = val.iter().fold(0, |acc, &x| acc + x ); // 6
    println!("Folding result = {folding_result}");
    
    // We pass our Box to the function that takes Vec,
    // Box<Vec> coerces to Vec
    f2(&val);
}

fn main() {
    deref_string();
    working_with_box_arc();
}
