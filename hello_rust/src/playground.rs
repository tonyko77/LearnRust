// Just some Rust Playground :)
use std::{mem, fmt::Display, any::type_name};

pub fn playground() {
    let i = 42;
    use_i32(i);
    use_i64(i.into());
    use_usize(i as usize);
}


fn use_i32(i: i32) {
    print_val(i);
}

fn use_i64(i: i64) {
    print_val(i);
}

fn use_usize(i: usize) {
    print_val(i);
}

fn print_val<T: Display>(i: T) {
    let s = mem::size_of::<T>();
    let n = type_name::<T>();
    println!("Some {n}: {i} - uses {s} bytes ({} bits)", 8 * s);
}
