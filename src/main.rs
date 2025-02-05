mod conditions;
mod operators;
mod scope;
#[cfg(test)]
mod tests;
mod value;

use value::*;

fn main() {
    let list = list!["Hello", "world", "!"];
    println!("{list}")
}
