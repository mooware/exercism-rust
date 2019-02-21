use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // for grapheme clusters, there's apparently a crate "unicode-segmentation"
    // with an iterator "input.graphemes(true)"
    let mut s = String::with_capacity(input.len());
    for g in UnicodeSegmentation::graphemes(input, true).rev() {
        s += g;
    }
    s
}
