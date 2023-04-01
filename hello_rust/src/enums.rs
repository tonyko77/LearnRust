//----------------------------------------------------------------
// (6) Enums and Pattern Matching

pub fn test_enums() {
    println!("\n=============================\n--> Enums:");

    // simple enum checking - match a variable against an enum value
    let simp = SimpleEnum::B;
    // `==` can be used only if the enum derives from `PartialEq`
    if simp == SimpleEnum::B {
        println!("simp == SimpleEnum::B");
    }
    // `matches!` always works (since Rust 1.42)
    if matches!(simp, SimpleEnum::B) {
        println!("matches!(simp, SimpleEnum::B)");
    }
    // you can even use `if let` - but it doesn't make sense in this situation
    // (and it looks really weird)
    if let SimpleEnum::B = simp {
        println!("let SimpleEnum::B = simp");
    }

    let b1 = Bogus::NoParams;
    let b2 = Bogus::OneParam(42);
    let b3 = Bogus::TupleParams(String::from("abc"), 12.34);
    let b4 = Bogus::NamedParams { i: 23, f: 17.89 };

    println!("b1 = {:?}", b1);
    println!("b2 = {:?}", b2);
    println!("b3 = {:?}", b3);
    println!("b4 = {:?}", b4);

    if b1 == Bogus::NoParams {
        println!("b1 is NoParams");
    }

    // using matches, if Eq/PartialEq is NOT implemented
    if matches!(b1, Bogus::NoParams) {
        println!("b1 matches NoParams");
    }

    if matches!(b2, Bogus::OneParam(_)) {
        println!("b2 matches OneParam");
    }

    // using if let
    if let Bogus::TupleParams(a, b) = b3 {
        // we can put the var names inside the println format string !!!
        println!("b3 is TupleParams({a}, {b})");
    }

    // using match
    let b4_match = match b4 {
        Bogus::NoParams => String::from("NoParams"),
        Bogus::OneParam(o) => format!("OneParam: {o}"),
        Bogus::TupleParams(a, b) => format!("TupleParams: ({a}, {b})"),
        Bogus::NamedParams { i, f } => format!("NamedParams: i={i}, f={f}"),
    };
    println!("Match for b4: {b4_match}");

    match b2 {
        Bogus::NoParams => {
            todo!();
        },
        Bogus::OneParam(_) => {
            println!("b2 matched OneParam :)");
        },
        Bogus::TupleParams(_, _) => {
            todo!();
        },
        Bogus::NamedParams {i: _, f: _} => {
            todo!();
        }
    }

    // using Option
    let option: Option<String> = Option::Some(String::from("abc"));
    check_option(&option);
    if option.is_some() {
        println!("Unwrapped option: {}", option.unwrap());
    }

    let option: Option<String> = Option::None;
    check_option(&option);

    // using Result
    let result: Result<i32, String> = Result::Ok(42);
    check_result(&result);
    let result: Result<i32, String> = Result::Err(String::from("some error"));
    check_result(&result);
}


#[derive(Debug, PartialEq)]
enum Bogus {
    NoParams,
    OneParam(i32),
    TupleParams(String, f64),
    NamedParams {i:i32, f:f64}
}


#[derive(PartialEq)]
enum SimpleEnum {
    A, B, C
}


fn check_option(option: &Option<String>) {
    if let Option::Some(text) = option {
        println!("We got some data: {}", text);
    }
    else {
        println!("We got NO data!");
    }
}


fn check_result(result: &Result<i32, String>) {
    match result {
        Result::Ok(ok) => {
            println!("Result is OK: {}", ok);
        },
        Result::Err(err) => {
            println!("Result is FAILURE: {}", err);
        }
    }
}
