// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

#[rustfmt::skip]
macro_rules! my_macro1 {
    () => {
        println!("Check out my macro!");
    }
}

macro_rules! my_macro {
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro1!();
    my_macro!(7777);
}
