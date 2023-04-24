// RUST test bench

fn main() {
    // test casting signed <-> unsigned
    let sgn: i8 = -1;
    println!("i8={sgn} => i8->u8={} ,", sgn as u8);

    let usgn: u8 = 128;
    println!("u8={usgn} => u8->i8={} ,", usgn as i8);
}


