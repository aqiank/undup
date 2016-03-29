extern crate undup;

use undup::undup_chars;

fn main() {
    let input = "The   Quick     Brown Fox       Jumps  Over The    Lazy Dog.........";
    let output = undup_chars(&input, vec![' ', '.']);
    println!("{}", output);
}
