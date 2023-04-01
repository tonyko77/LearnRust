//----------------------------------------------------------------
// (4) Understanding Ownership, References, Borrowing ...

pub(crate) fn test_ownership_and_borrowing() {
    // this works, ...
    // (because: int, float, char, bool + tuples of these implement the Copy trait)
    let i1 = 4;
    let i2 = i1;
    println!("{i1} == {i2}");

    // ... this also works, ...
    let c1 = "abc";
    let c2 = c1;
    println!("{c1} == {c2}");

    // ... but this doesn't - s1 is MOVED
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{s1} == {s2}");

    // ... we can fix it by using cloning
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{s1} == {s2}");

    // ... or by using references
    let s1 = String::from("goodbye");
    let s2 = &s1;
    println!("{s1} == {s2}");

    // cannot borrow as mutable more than once
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // cannot borrow as mutable if already borrowed as immutable (and vice versa)
    // let mut s = String::from("qwe");
    // let r1 = &s;
    // let w2 = &mut s;
    // let r3 = &s;
    // println!("{s} == {r1} == {w2} == {r3}");
}
