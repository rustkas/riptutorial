// This function takes two integers and a function that performs some operation on the two arguments
fn apply_function<T>(a: i32, b: i32, func: T) -> i32
where
    T: Fn(i32, i32) -> i32,
{
    // apply the passed function to arguments a and b
    func(a, b)
}

// Box in the return type moves the function from the stack to the heap
fn curried_adder(a: i32) -> Box<dyn Fn(i32) -> i32> {
    // 'move' applies move semantics to a, so it can outlive this function call
    Box::new(move |b| a + b)
}

fn main() {
    {
        // lambda expressions can have explicitly annotated return types
        let floor_func = |x: f64| -> i64 { x.floor() as i64 };
        println!("{}", floor_func(3.14));
    }
    {
        // let's define three lambdas, each operating on the same parameters
        let sum = |a, b| a + b;
        let product = |a, b| a * b;
        let diff = |a, b| a - b;

        // And now let's pass them to apply_function along with some arbitary values
        println!("3 + 6 = {}", apply_function(3, 6, sum));
        println!("-4 * 9 = {}", apply_function(-4, 9, product));
        println!("7 - (-3) = {}", apply_function(7, -3, diff));
    }
    println!("3 + 4 = {}", curried_adder(3)(4));

    {
        // variable definition outside the lambda expression...
        let lucky_number: usize = 663;

        // but the our function can access it anyway, thanks to the closures
        let print_lucky_number = || println!("{}", lucky_number);

        // finally call the closure
        print_lucky_number();
    }

    {
        // A simple adder function defined as a lambda expression.
        // Unlike with regular functions, parameter types often may be omitted because the
        // compiler can infer their types
        let adder = |a, b| a + b;
        // Lambdas can span across multiple lines, like normal functions.
        let multiplier = |a: i32, b: i32| {
            let c = b;
            let b = a;
            let a = c;
            a * b
        };

        // Since lambdas are anonymous functions, they can be called like other functions
        println!("{}", adder(3, 5));
        println!("{}", multiplier(3, 5));
    }
}
