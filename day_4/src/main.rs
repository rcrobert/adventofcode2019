use std::ops::Index;
use std::convert::TryInto;


fn main() {
    // Input Range: 197487-673251
    let lower = 197487;
    let upper = 673251;

    let mut silly_number = SillyNumber::new(lower);
    println!("{:?}", silly_number);

    let mut passwords = Vec::<u64>::new();
    loop {
        if silly_number.number > upper {
            break;
        }
        if silly_number.is_valid_password() {
            passwords.push(silly_number.number);
        }
        silly_number.increment();
    }

    println!("Found {} passwords", passwords.len());
}

trait Password {
    fn is_valid_password(&self) -> bool;
}

#[derive(Debug)]
struct SillyNumber { 
    number: u64,
    digits: Vec<u8>,
}

impl SillyNumber {
    fn new(number: u64) -> Self {
        let digits = Self::make_digits(number);

        SillyNumber {
            number,
            digits,
        }
    }

    fn make_digits(number: u64) -> Vec<u8> {
        let mut digits = Vec::<u8>::with_capacity(6);
        for pos in 0..6 {
            let digit = Self::get_digit_at(number, pos);
            digits.push(digit);
        }

        // This better only have six digits
        assert!(Self::get_digit_at(number, 7) == 0);
        digits
    }

    fn get_digit_at(number: u64, pos: usize) -> u8 {
        let pos: u32 = pos.try_into().unwrap();

        let digit = number / (10_u64.pow(pos)) % 10;
        let digit: u8 = digit as u8;
        digit
    }

    fn increment(&mut self) {
        self.number += 1;
        self.digits = Self::make_digits(self.number);
    }

    fn iter(&self) -> SillyNumberIter {
        SillyNumberIter {
            digits: &self.digits,
            pos: 0,
        }
    }
}

impl Index<usize> for SillyNumber {
    type Output = u8;

    fn index(&self, pos: usize) -> &Self::Output {
        &self.digits[pos]
    }
}

struct SillyNumberIter<'a> {
    digits: &'a Vec<u8>,
    pos: usize,
}

impl<'a> Iterator for SillyNumberIter<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.digits.len() {
            None
        } else {
            self.pos += 1;
            Some(self.digits[self.pos-1])
        }
    }
}

impl Password for SillyNumber {
    fn is_valid_password(&self) -> bool {
        let mut last_digit: u8 = 66;
        let mut pair_found = false;
        let mut length_of_run = 0;

        for digit in self.iter() {
            if digit > last_digit {
                return false;
            } else if digit == last_digit {
                length_of_run += 1;
            } else {
                // A valid, different digit
                if !pair_found {
                    pair_found = length_of_run == 2;
                }
                length_of_run = 1;
            }
            last_digit = digit;
        }

        // We may end on a pair
        return pair_found || length_of_run == 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_pairs() {
        assert!(SillyNumber::new(112233).is_valid_password());
        assert!(SillyNumber::new(125599).is_valid_password());
        assert!(SillyNumber::new(115699).is_valid_password());
    }

    #[test]
    fn test_run() {
        assert!(!SillyNumber::new(999999).is_valid_password());
        assert!(!SillyNumber::new(123444).is_valid_password());
    }

    #[test]
    fn test_run_with_pair() {
        assert!(SillyNumber::new(222559).is_valid_password());
        assert!(SillyNumber::new(111199).is_valid_password());
        assert!(SillyNumber::new(112222).is_valid_password());
    }

    #[test]
    fn test_decreasing_digit() {
        assert!(!SillyNumber::new(221555).is_valid_password());
    }
}
