mod my_funcs;
use crate::my_funcs::add_five;
fn main() {
    let mut x: u32 = 50;
    println!("{}", x);

    let y = add_five(x);

    x = 70;

    println!("{} + 5 = {}", x, y);
}
