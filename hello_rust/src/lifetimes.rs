// Lifetimes and references

use std::fmt::Display;

pub fn lifetimes() {
    test_longer();
    test_struct_lifetime();

    let a = "abc def ghi";
    let x = first_word(a);
    println!("The first word of `{a}` is {x}");

    test_static_lifetime();
}

//--------------

fn test_longer() {
    let string1 = String::from("abcdefg");
    let result;
    {
        let string2 = String::from("xyz");
        // result is valid as long as string2 (which has the shorter lifespan) is valid
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    // here it would NOT be ok - string2 would have shorter lifespan than result
    //println!("The longest string is {}", result);
}

// BAD (no lifetimes):
//fn longest(x: &str, y: &str) -> &str {
// =>
//     error[E0106]: missing lifetime specifier
//     --> src\lifetimes.rs:21:33
//      |
//   21 | fn longest(x: &str, y: &str) -> &str {
//      |               ----     ----     ^ expected named lifetime parameter
//      |
//      = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
//   help: consider introducing a named lifetime parameter
//      |
//   21 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//      |           ++++     ++          ++          ++

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


//---------------------
// Struct lifetimes

// Struct with lifetime annotation, because it contains references
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // 3rd elision rule applies -> returned value gets same lifetime annotation as self
    // (which is ok in this case, as we return a ref contained in self)
    // => no explicit lifetime annotations are needed here :)
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }

    // here, the 3rd elision rule DOES apply, but it is WRONG :(
    // (the returned ref does not have same lifetime annotation as self)
    // We can fix it by adding an explicit lifetime annotation,
    // to bind the parameter ref and the corresponding returned ref
    fn announce_and_return_arg<'b>(&self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        announcement
    }
}


fn test_struct_lifetime() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("Ref from struct: {}", i.part);
}

//---------------------
// Lifetime Elision
//---------------------
// i.e. no need to explicitly add lifetimes, because the compilert can auto-determine lifetimes
// Elision rules (used by the compiler):
//  (1) compiler assigns a lifetime param to each parameter that’s a reference:
//          fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
//  (2) if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters:
//          fn foo<'a>(x: &'a i32) -> &'a i32
//  (3) (for methods) if there are multiple input lifetime parameters, but one of them is &self or &mut self
//      => the lifetime of self is assigned to all output lifetime parameters.

// Here, ellision rules (1) + (2) apply
//  => same as: fn first_word<'a>(s: &'a str) -> &'a str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

//---------------------
// The Static Lifetime

fn test_static_lifetime() {
    let s: &'static str = "I have a static lifetime.";
    println!("Static: {s}");
}


//-------------------------------------------------------------------
// Generic Type Parameters, Trait Bounds, and Lifetimes Together

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

