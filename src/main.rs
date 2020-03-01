mod chapters;

use chapters::{c01, c02::chapter_main as chapter_02};

fn main() {
    c01::format_string();
    chapter_02();
}
