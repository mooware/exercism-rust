pub fn build_proverb(list: &[&str]) -> String {
    let mut result = String::new();
    for i in 1..list.len() {
        result += &format!("For want of a {} the {} was lost.\n", list[i-1], list[i]);
    }
    if !list.is_empty() {
        result += &format!("And all for the want of a {}.", list[0]);
    }
    result
}
