// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.


macro_rules! my_macro {
    ($x:expr) => {
        println!("{}", $x);
    };
}

fn main() {
    my_macro!("Check out my macro!");
}