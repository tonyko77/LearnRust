//! RUST test bench

#![allow(dead_code)]
#![allow(unused_variables)]

use std::f64::consts::PI;

//----------------------------------------------------
//  Deg to rad + clamp between 0 .. 2*pi

fn main() {
    deg_to_rad(0);
    deg_to_rad(1);
    deg_to_rad(90);
    deg_to_rad(180);
    deg_to_rad(359);
    deg_to_rad(360);
    deg_to_rad(362);
    deg_to_rad(720);
    deg_to_rad(-1);
    deg_to_rad(-362);
    deg_to_rad(-723);
}

fn deg_to_rad(deg: i32) -> f64 {
    let xdeg = ((deg % 360) + if deg < 0 { 360 } else { 0 }) as f64;
    // let xdeg = deg - (((deg / 360.0) as i32) * 360) as f64;
    // let xdeg = if xdeg < 0.0 { xdeg + 360.0 } else { xdeg };
    let rad = xdeg * PI / 180.0;
    println!("Deg(in) = {deg} => Deg(out) = {xdeg} ; Rad = {rad}");
    rad
}

//----------------------------------------------------
// Test how to use a struct to borrow a variable
struct XStruct<'a> {
    x: &'a mut i32,
}

fn _borrow_main() {
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

//----------------------------------------------------
// Test how signed<-> unsigned casting works
fn _signed_unsigned_main() {
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
