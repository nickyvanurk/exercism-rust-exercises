extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let s = String::from(input);
    s.graphemes(true).rev().collect::<String>()
}
