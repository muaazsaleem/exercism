// - 9 is an Armstrong number, because `9 = 9^1 = 9`
// - 10 is _not_ an Armstrong number, because `10 != 1^2 + 0^2 = 1`
// - 153 is an Armstrong number, because: `153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153`
// - 154 is _not_ an Armstrong number, because: `154 != 1^3 + 5^3 + 4^3 = 1 + 125 + 64 = 190`

// An [Armstrong number][armstrong-number] is a number that is the sum of its own digits each raised to the power of the number of digits.

// raise each number to the power of the number of digits
// add the numbers
// compare the original number to the sum
pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let count = num_str.len() as u32;
    num_str
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        // u32.pow(u32) can technically overflow, so can u32 + ... + u32
        .fold(Some(0), | acc, i |
        // if the last sum produced a None, we overflowed, return None as well
        acc.and_then(|acc|
            i.checked_pow(count).and_then(|i| i.checked_add(acc) )
        )
    ).is_some_and(|sum| sum == num)
}

#[cfg(test)]
mod tests {
    use crate::is_armstrong_number;

    #[test]
    fn u32_overflow() {
       assert_eq!(is_armstrong_number(u32::MAX), false);
    }
}
