mod chapters;

use chapters::{c01, c02::chapter_main as chapter_02};

fn main() {
    // c01::format_string();
    // c01::display();
    // c01::testcase_list();
    c01::formatting();


    chapter_02();
}
