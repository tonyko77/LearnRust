//----------------------------------------------------------------
// (8) Common Collections

pub(crate) fn test_common_collections() {
    println!("\n=============================\n--> Collections:");

    _test_vectors();
    _test_strings();
    _test_hash_maps();
}


fn _test_vectors() {
    let mut v = Vec::<i32>::new(); // turbofish :)
    // same as:  let v: Vec<i32> = Vec::new();

    // initialize vector using the `vec!` macro
    let v2 = vec!['a', 'b', 'c'];

    // add elements using `push()`
    v.push(1);
    v.push(2);
    v.push(3);

    // access elements via indexing or `get()`
    let e1: i32 = v[0];
    let e2: &i32 = &v[1];
    let e3: Option<&i32> = v.get(2);
    println!("First={e1}, second={e2}, third={}", e3.unwrap_or(&999));
    // no problem if not using refs -> i32 has Copy trait :)
    println!("Again: First={}, second={}, third={}", v[0], v[1], v[2]);

    // accessing index OUT OF RANGE
    //let this_will_panic = &v[100];
    let this_will_return_none = v.get(100);
    if let Option::Some(x) = this_will_return_none {
        println!("THIS WILL NEVER HAPPEN: {x}");
    }
    else {
        println!("As expected, v.get(100) returned None: {this_will_return_none:?}");
    }

    // cannot have mutable and immutable refs to same vector
    let first = &v[0];
    //v.push(42); // <-- This is a mutable borrow => CANNOT do this while the `first` ref is alive
    println!("First is {first}");
    v.push(9); // Here we can push (mutable borrow of `v`), as long as the `first` ref is NO LONGER USED from here on

    // use enums to store items of different types
    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    // NEAT TRICK: get both index and element, in a `for` loop
    for (idx, elem) in row.iter().enumerate() {
        match elem {
            SpreadsheetCell::Int(i) => {
                println!("{idx} -> Int: {i}");
            },
            SpreadsheetCell::Float(f) => {
                println!("{idx} -> Float: {f}");
            },
            SpreadsheetCell::Text(t) => {
                println!("{idx} -> Text: {t}");
            },
        }
    }

    // setting values into a vector
    println!("Before: {}", v[2]);
    v[2] = 42;
    println!("After: {}", v[2]);

    // removing values from a vector (=> it can be used as a stack)
    println!("Len before: {}", v.len());
    let popped = v.pop();
    // if let Option::Some(i) = popped {
    //     println!("Popped: {i}");
    // }
    println!("Popped: {popped:?}");
    println!("Len after: {}", v.len());

    // now let's play with a vector of structs
    let mut v = Vec::<SomeStruct>::new();
    let a = SomeStruct {
        name: String::from("Toni"),
        age: 42,
        data: SpreadsheetCell::Float(4.2)
    };
    println!("From a: {:?}", a);
    v.push(a);
    println!("From vect: {:?}", v[0]);
    //println!("From a: {:?}", a); // cannot do this here - a was MOVED

    // modify vector item
    v[0].name.push('k');
    v[0].age += 4;
    v[0].data = SpreadsheetCell::Text(String::from("qwe"));
    println!("Modif in vect: {:?}", v[0]);
}


#[derive(Debug)]
struct SomeStruct {
    name: String,
    age: u32,
    data: SpreadsheetCell
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn _test_strings() {
    // create an empty String
    let s = String::new();

    // create a str slice ...
    let data = "initial contents";
    // ... and copy it to a new String
    let mut s = data.to_string();
    // proof that it is a copy
    s.push_str(" no more :)");
    println!("Original: {data}");
    println!("Copy String: {s}");

    // the to_string() method also works on a literal directly:
    let s = "initial contents".to_string();

    // or we can use String::from()
    let s = String::from("abc");

    // Strings and str slices are UTF-8 encoded !!
    let hello1 = String::from("السلام عليكم");
    let hello2 = String::from("Dobrý den");
    let hello3 = String::from("שָׁלוֹם");
    let hello4 = String::from("नमस्ते");
    let hello5 = String::from("こんにちは");
    let hello6 = String::from("안녕하세요");
    let hello7 = String::from("你好");
    let hello8 = String::from("Здравствуйте");

    // concat strings
    let s1 = String::from(&hello5);
    let s2 = String::from(&hello8);
    let s3 = String::from(&hello6);
    // when adding, the first string will be moved, the rest can be borrowed
    // Also, the compiler can coerce directly &String into &str :)
    let sconcat = s1 + &s2 + &s3;
    println!("Concat: {sconcat}");

    // For more complicated string concatenations, use the `format!` macro:
    // Also, `format!` does not take ownership of any of its arguments
    let s1 = String::from(&hello5);
    let sformat = format!("{s1} == {s2} == {s3}");
    println!("Format: {sformat}");

    // IMPORTANT: Rust Strings DO NOT support indexing !!
    //let wont_compile = s1[0];

    // They support slicing, using byte indexes :/ - WEIRD
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // even though this is 4 bytes, it only encompasses 2 characters :(
    println!("A slice of {hello} is {s}");
    // Also, the length is given in BYTES, not characters :(
    println!("{hello} has 12 characters, but its len() is {}", hello.len());
    // Slicing "inside" a multi-byte character leads to runtime errors
    //let this_will_panic = &hello[0..1]; // this panics at runtime
    // Slicing should be done at valid char indices, which can be obtained via `.char_indices()`

    // With that crazyness in mind => How to iterate over strings
    // We must be explicit that if we want characters ...
    print!("The chars of {hello} are:");
    for c in hello.chars() {
        print!(" {c}");
    }
    println!(" => chars count is: {}", hello.chars().count());
    // ... or bytes:
    print!("The bytes of {hello} are:");
    for b in hello.bytes() {
        print!(" {b}");
    }
    println!(" => byte count is: {}", hello.bytes().count());
    // ... or even char indices:
    println!("The char indices of {hello} are:");
    let indices = hello.char_indices();
    for c in indices {
        println!("  => [{}] = \'{}\'", c.0, c.1);
    }

}


use std::collections::HashMap;

fn _test_hash_maps() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Initial HashMap: {scores:?}");

    let blue_score_ref: Option<&i32> = scores.get("Blue"); // HashMap::get() returns a ref Option ...
    let blue_score = blue_score_ref.copied(); // ... so we turn it into a non-ref option
    println!("Blue score: {blue_score:?}");
    // using a missing key just returns None
    let this_is_none = scores.get("abc");
    println!("This is None: {this_is_none:?}");

    // iterate using key/value pairs, via a `for` loop
    for (key, value) in &scores {
        println!("Score: {key} = {value}");
    }

    // overwriting a value - same as adding, using `insert()`
    scores.insert("Blue".to_string(), 33);
    println!("Updated Blue score: {:?}", scores.get("Blue"));

    // adding a key/value only if key is not present in the map => `entry()`
    scores.entry(String::from("Blue")).or_insert(110);
    scores.entry(String::from("Green")).or_insert(120);
    println!("Second HashMap: {scores:?}");

    // updating the value based on the old value
    // example: count words from a text
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);    

}
