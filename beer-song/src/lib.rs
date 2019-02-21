pub fn verse(n: i32) -> String {
    assert!(n >= 0);
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        _ => {
            let plural = if n > 2 { "s" } else { "" };
            format!("{i} bottles of beer on the wall, {i} bottles of beer.\nTake one down and pass it around, {j} bottle{s} of beer on the wall.\n",
                i = n, j = n - 1, s = plural)
        }
    }
}

pub fn sing(start: i32, end: i32) -> String {
    assert!(start >= end && end >= 0);
    let mut s = String::new();
    for i in (end..=start).rev() {
        if i != start {
            s += "\n";
        }
        s += &verse(i);
    }
    return s;
}
