pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let count = num_str.len() as u32;
    num_str
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        // u32.pow(u32) can technically overflow, so can u32 + ... + u32
        .try_fold(0, |acc, i| {
            i.checked_pow(count).and_then(|i| i.checked_add(acc))
        })
        .is_some_and(|sum| sum == num)
}

#[cfg(test)]
mod tests {
    use crate::is_armstrong_number;

    #[test]
    fn u32_overflow() {
        assert_eq!(is_armstrong_number(u32::MAX), false);
    }
}
