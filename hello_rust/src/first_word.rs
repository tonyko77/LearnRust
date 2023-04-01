//----------------------------------------------------------------
// Exercise: extract the first word from a given string

pub(crate) fn test_first_word_corner_cases() {
    println!("\n=============================\n--> Exercise: extract the first word from a given string");

    _test_first_word("hello");
    _test_first_word("hello world of rust");
    _test_first_word("   hello   ");
    _test_first_word("   hello");
    _test_first_word("hello   ");
    _test_first_word("   ");
    _test_first_word("");
}

fn _test_first_word(text: &str) {
    let fw = _first_word(text);
    println!("split_str test: [{}] => [{}]", text, fw);
}

fn _first_word(text: &str) -> &str {
    let mut start = 0;
    let mut end = text.len();
    let mut found_start = false;

    for (i, c) in text.chars().enumerate() {
        if !found_start {
            if !c.is_whitespace() {
                found_start = true;
                start = i;
            }
        }
        else {
            if c.is_whitespace() {
                end = i;
                break;
            }
        }
    }
    if found_start {
        &text[start..end]
    }
    else {
        ""
    }
}
