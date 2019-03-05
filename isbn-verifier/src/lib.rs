/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut digits = [0; 10];
    let mut i = 0;
    for c in isbn.chars() {
        if i >= digits.len() {
            return false; // too long
        } else if c == '-' {
            continue; // ignore char
        } else if c.is_ascii_digit() {
            digits[i] = c.to_digit(10).unwrap();
            i += 1;
        } else if c == 'X' && i == digits.len() - 1 {
            digits[i] = 10;
            i += 1;
        }
    }
    if i != digits.len() {
        return false; // too short
    }
    let mut sum = 0;
    for i in 0..digits.len() {
        sum += digits[i] as usize * (digits.len() - i);
    }
    return sum % 11 == 0;
}
