//! RUST test bench

#![allow(dead_code)]
#![allow(unused_variables)]

use std::f64::consts::PI;

//----------------------------------------------------
//  Deg to rad + clamp between 0 .. 2*pi

fn main() {
    const PI2: f64 = 2.0 * PI;
    let x = PI2 + 1.0;
    let y = x % PI2;
    println!("x={x} => y={y}");
    let x = 1.0 - PI2;
    let y = x % PI2;
    println!("x={x} => y={y}");

    deg_to_rad(0);
    deg_to_rad(1);
    deg_to_rad(180);

    deg_to_rad(359);
    deg_to_rad(360);
    deg_to_rad(361);

    deg_to_rad(-1);
    deg_to_rad(-719);
    deg_to_rad(-721);
}

fn deg_to_rad(deg: i32) -> f64 {
    const PI2: f64 = 2.0 * PI;
    let mut rad = ((deg as f64) * PI / 180.0) % PI2;
    if rad < 0.0 {
        rad += PI2;
    }

    let xdeg = (rad * 180.0 / PI + 0.03125) as i32;
    println!("Deg(in) = {deg} => Rad(out) = {rad} ; Deg(out) = {xdeg}");
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
