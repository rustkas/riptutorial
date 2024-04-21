pub fn work_on_bytes(slice: &[u8]) {
    println!("{:?}", slice);
}

pub fn work_on_bytes2<T: AsRef<[u8]>>(input: T)->T {
    let slice = input.as_ref();

    println!("Builded slice: {:?}", slice);
    input
}

fn main() {
    let vec = Vec::new();
    work_on_bytes(&vec);
    let vec = work_on_bytes2(vec);
    work_on_bytes2(vec);

    let arr = [0; 10];
    work_on_bytes(&arr);
    work_on_bytes2(arr);
    work_on_bytes2(arr);

    let slice = &[1, 2, 3];
    work_on_bytes(slice); // Note lack of &, since it doesn't need coercing
    work_on_bytes2(slice);

    work_on_bytes("strings work too!".as_bytes());
    work_on_bytes2("strings work too!");

}
