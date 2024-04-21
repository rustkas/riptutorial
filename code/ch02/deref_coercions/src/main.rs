use std::rc::Rc;

/// Given two types `T` and `U`, `&T` will coerce (implicitly convert) to `&U` if and only if `T` implements `Deref<Target=U>`
fn main() {
    fn foo(_a: &[i32]) {
        // code
    }

    fn bar(_s: &str) {
        // code
    }

    let v = vec![1, 2, 3];
    foo(&v); // &Vec<i32> coerces into &[i32] because Vec<T> impls Deref<Target=[T]>

    let s = "Hello world".to_string();
    let rc = Rc::new(s);
    // This works because Rc<T> impls Deref<Target=T> ∴ &Rc<String> coerces into
    // &String which coerces into &str. This happens as much as needed at compile time.
    bar(&rc);
}
