/// Remove duplicate characters in a string
pub fn undup_chars(input: &str, chars: Vec<char>) -> String {
    let mut output = String::new();
    let mut previous_char = '\0';

    for c in input.chars() {
        if !(c == previous_char && chars.contains(&c)) {
            output.push(c);
        }
 
        previous_char = c;
    }

    output
}

#[test]
fn test_undup_chars() {
    let input = "The   Quick     Brown Fox       Jumps  Over The    Lazy Dog.........";
    let output = undup_chars(&input, vec![' ', '.']);
    assert_eq!("The Quick Brown Fox Jumps Over The Lazy Dog.", output);
}
