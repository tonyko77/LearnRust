// Traits

use std::fmt;

// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// A rough one-sentence summary of each of the STANDARD TRAITS:
//
//   *      Clone: Items of this type can make a copy of themselves when asked.
//   *       Copy: If the compiler makes a bit-for-bit copy of this item's memory representation,
//                 the result is a valid new item.
//   *    Default: It's possible to make new instance of this type with sensible default values.
//   *  PartialEq: There's a partial equivalence relation for items of this type – any two items
//                 can be definitively compared, but it may not always be true that x==x.
//   *         Eq: There's an equivalence relation for items of this type: any two items
//                 can be definitively compared, and it is always true that x==x.
//   * PartialOrd: Some items of this type can be compared and ordered.
//   *        Ord: All items of this type can be compared and ordered.
//   *       Hash: Items of this type can produce a stable hash of their contents when asked.
//   *      Debug: Items of this type can be displayed to programmers.
//   *    Display: Items of this type can be displayed to users.
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!


//-----------------------
// How to impl Display

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
