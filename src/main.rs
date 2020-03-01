mod chapters;

use chapters::{c01, c02};

fn separator() {
    println!("\n \n ---------------------------------------- \n \n")
}
fn main() {
    c01::format_string();
    separator();
    c01::display();
    separator();
    c01::testcase_list();
    separator();
    c01::formatting();
    separator();
    c02::primitives_and_operations();
    separator();
    c02::tuples();
}
