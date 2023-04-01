//----------------------------------------------------------------
// (3) Common Programming Concepts

pub(crate) fn test_basic_rust() {
    println!("\n=============================\n--> Just some basic Rust stuff ...");

    // shadowing immutable variables
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("Some chars: {c} {z} {heart_eyed_cat}");

    // some tuples
    let tup = (500, 6.4, 1);
    let (_, y, _) = tup;
    println!("The value of y is: {y}");

    // some more tuples
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // some arrays
    let _months = [
        "January", "February", "March", "April",
        "May", "June", "July", "August", "September",
        "October", "November", "December"];
    let a = [3; 5];
    _print_and_sum_array(&a);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let len = a.len();

    let func_ptr = _print_and_sum_array;
    let z = if a.len() > 5 { 9999 } else { func_ptr(&a) };

    println!("Using stuff: {five_hundred} {six_point_four} {one} {z} {len}");

    _control_flow();
}


fn _print_and_sum_array(arr: &[i32]) -> i32 {
    print!("Array has {} elements: ", arr.len());
    let mut sep = "[";
    let mut sum = 0;
    for x in arr {
        print!("{sep}{x}");
        sum += x;
        sep = ", ";
    }
    println!(" ] => sum is {sum}");
    sum
}


fn _abc() -> () {
    ()
}


fn _control_flow() {
    let number = 4;
    if number < 5 {
        println!("{number} < 5");
    } else if number < 50 {
        println!("{number} in range [5, 50)");
    } else {
        println!("{number} >= 50");
    }   
}
