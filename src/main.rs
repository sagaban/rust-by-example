mod chapters;

use chapters::{c01, c02, c03};

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
    separator();
    c02::arrays_and_slices();
    separator();
    c03::structures();
    separator();
    c03::enums();
    separator();
    c03::alias();
    separator();
    c03::_use();
    separator();
    c03::c_liKe_enums();
    separator();
    c03::testcase_linked_list();
    separator();
    c03::constants();
}
