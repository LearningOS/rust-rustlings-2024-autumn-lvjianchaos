// variables2.rs
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.
fn main() {
    let x = 10;
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}

// hint:
// The compiler message is saying that Rust cannot infer the type that the
// variable binding `x` has with what is given here.
// What happens if you annotate line 7 with a type annotation?
// What if you give x a value?
// What if you do both?
// What type should x be, anyway?
// What if x is the same type as 10? What if it's a different type?
