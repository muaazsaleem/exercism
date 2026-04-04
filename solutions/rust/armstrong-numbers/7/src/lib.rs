pub fn is_armstrong_number(num: u32) -> bool {
    let count = Digits(num).count() as u32;

    Digits(num)
        // if num == 0, the try_fold gets skipped
        .try_fold(0, |acc, digit| {
            // digit^count + acc
            // u32.pow(u32) can technically overflow, so can u32 + ... + u32
            digit.checked_pow(count).and_then(|digit| digit.checked_add(acc))
        })
        .is_some_and(|sum| sum == num)
}

struct Digits(u32);

impl Iterator for Digits {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 > 0 {
            let digit = self.0 % 10;
            self.0 /= 10;
            Some(digit)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::is_armstrong_number;
    #[test]
    fn u32_overflow() {
        assert_eq!(is_armstrong_number(u32::MAX), false);
    }
}
