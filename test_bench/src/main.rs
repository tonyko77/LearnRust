// RUST test bench

#[derive(Debug)]
struct XStruct<'a> {
    x: &'a mut i32,
}

fn main() {
    let mut xowner = 1; // Declare a mutable variable x
    let x_struct = XStruct { x: &mut xowner }; // Pass a mutable reference to x

    // mutate x via struct
    *x_struct.x += 1;

    // while x_struct is in scope, xowner cannot be used
    let x_struct = (); // Declare a new variable x_struct, so that the old one is out of scope

    xowner += 1; // Modify x...

    println!("The value of x_struct is {:?}.", x_struct);
    println!("The value of x is {:?}.", xowner);
}


fn _cast_main() {
    // test casting signed <-> unsigned
    let sgn: i8 = -1;
    println!("i8={sgn} => i8->u8={} ,", sgn as u8);

    let usgn: u8 = 128;
    println!("u8={usgn} => u8->i8={} ,", usgn as i8);

    // test casting chars to int
    for ch in 'A'..'F' {
        let cod = ch as u32;
        let d = (ch as u8) - ('A' as u8) + 10;
        println!("ch={ch} => code={cod} , d={d}");
    }
}


