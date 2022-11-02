#[test]
fn trim_string() {
    //
    let untrimmed_str = " this is test with whitespace    \t";
    let trimmed_str = untrimmed_str.trim();
    println!("Trimmed str = \"{}\"", trimmed_str);
    //
    assert_eq!("this is test with whitespace", trimmed_str);
}

#[test]
fn string_length() {
    //
    let message = "All good things come to those who wait";
    println!("Length = {}", message.len());
    //
    assert_eq!(message.len(), 38);
}

#[test]
fn string_number_of_chars() {
    //
    let message = "文字列の長さ";
    assert_eq!(message.chars().count(), 6);
    //
}

#[test]
fn split_string() {
    //
    // Split a string on a token
    let input = "20,30,400,100,21,-1";
    let values : Vec<&str> = input.split(",").collect();
    for (i, s) in values.iter().enumerate() {
        println!("Value {} = {}", i, s);
    }
    //
    assert_eq!(values.len(), 6);
}

#[test]
fn split_whitespace() {
    for s in " All good   \n\n\tthings  to those who    wait".split_whitespace() {
        println!("Part - {}", s);
    }
}

#[test]
fn tokenize_string() {
    // TODO
}

#[test]
fn join_strings() {
    // TODO
}

#[test]
fn get_substring() {
    // TODO
}

#[test]
fn upper_to_lower() {
    // TODO
}

#[test]
fn case_insensitive_compare() {
    // TODO
}


#[test]
fn regular_expression_match() {
    // TODO
}

