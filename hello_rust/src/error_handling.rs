//----------------------------------------------------------------
// (9) Error Handling

use std::num::ParseIntError;

pub(crate) fn test_error_handling() {
    println!("\n=============================\n--> Some error handling:");

    // just a straight panic :(
    //panic!("This is a panic");

    let parsed = "23".parse::<i32>();
    println!("Parsed ok: {}", parsed.unwrap());

    let badparse = std::fs::read_dir("/bad/path");
    // unwrap() will always panic if the result is an error
    // => unwrap() is crash-prone and SHOULD NOT be used, unless you are SURE that the result is not an error !!!
    //println!("Parsed bad: {}", badparse.unwrap());

    // expect() also panics, but can customize the panic out message
    // badparse.expect("An error occurred");

    match badparse {
        Ok(read_dir) => {
            println!("Read Ok: {read_dir:?}");
        },
        Err(err) => {
            println!("Read Err: {err:?}");
        },
    }

    let qmo = question_mark_operator("23", "45").unwrap();
    println!("qmo: {qmo}");

    // convert one error type to another
    // (using explicit types, to make it clear)
    let errx: Result<i32, ParseIntError> = question_mark_operator("x", "23");
    let mapped_err: Result<i32, String> = errx.map_err(|x| {
        format!("Got an error: {x:?}")
    });
    println!("Mapped error: {mapped_err:?}");
}


// using the ? operator to unpack-or-immediately-return-failure
fn question_mark_operator(x: &str, y: &str) -> Result<i32, ParseIntError> {
    let parsed1 = x.parse::<i32>()?; // either return, if the parse() was ok ...
    let parsed2 = y.parse::<i32>()?; // ... or early return the error
    Ok(parsed1 + parsed2)
}
