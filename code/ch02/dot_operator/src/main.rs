fn main() {
    let mut name: String = "hello world".to_string();
    println!("{name}");
    // no deref happens here because push is defined in String itself
    name.push('!');

    println!("{name}");

    let name_ref: &String = &name;
    println!("{name_ref}");

    // Auto deref happens here to get to the String. See below
    let name_len = name_ref.len();
    println!("{name_len}");
    
    // You can think of this as syntactic sugar for the following line:
    let name_len2 = (*name_ref).len();
    println!("{name_len2}");

    // Because of how the deref rules work,
    // you can have an arbitrary number of references.
    // The . operator is clever enough to know what to do.
    let name_len3 = (&&&&&&&&&&&&name).len();
    assert_eq!(name_len3, name_len);
}
