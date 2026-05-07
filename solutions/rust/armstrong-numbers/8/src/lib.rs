pub fn is_armstrong_number(num: u32) -> bool {
    if num < 10 {
       return true;
    }
    let count = num.ilog10() + 1;

    Digits(num)
        // if num == 0, the try_fold gets skipped
        .try_fold(0, |acc, digit| {
	// The maximal u32 is 4_294_967_295. This number has 10 digits, so the maximal digit^count would be 
	// 9.pow(10) == 3_486_784_401, which doesn't overflow. 
            digit.pow(count).checked_add(acc)
        })
        .is_some_and(|sum| sum == num)
}

struct Digits(u32);

impl Iterator for Digits {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        (self.0 > 0).then(|| {
            let digit = self.0 % 10;
            self.0 /= 10;
            digit
        }) 
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
