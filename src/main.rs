struct MyError {
    value: u32,
}

fn main() {
    let e: *const MyError = std::ptr::null();
    let e_ref: &MyError = unsafe { &*e };
    dbg!(std::mem::size_of_val(&e_ref));
    print_error(e_ref);
}

fn print_error(e: &MyError) {
    println!("MyError (value = {})", e.value);
}
