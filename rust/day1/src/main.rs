const DIGITS: [&str; 18] = [
    "1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six", "7", "seven", "8",
    "eight", "9", "nine",
];

pub struct DigitParser<'a> {
    initial: &'a str,
}

impl<'a> DigitParser<'a> {
    pub fn new(initial: &'a str) -> Self {
        Self { initial }
    }

    pub fn first_digit(&self) -> Option<u64> {
        let mut unparsed;
        for current in 0..self.initial.len() {
            unparsed = &self.initial[current..];
            for (index, digit) in DIGITS.into_iter().enumerate() {
                if unparsed.starts_with(digit) {
                    match index {
                        0 | 1 => return Some(1),
                        2 | 3 => return Some(2),
                        4 | 5 => return Some(3),
                        6 | 7 => return Some(4),
                        8 | 9 => return Some(5),
                        10 | 11 => return Some(6),
                        12 | 13 => return Some(7),
                        14 | 15 => return Some(8),
                        16 | 17 => return Some(9),
                        _ => continue,
                    };
                }
            }
        }

        return None;
    }

    pub fn last_digit(&self) -> Option<u64> {
        let mut unparsed;
        for current in (0..self.initial.len()).rev() {
            unparsed = &self.initial[current..];
            for (index, digit) in DIGITS.into_iter().enumerate() {
                if unparsed.starts_with(digit) {
                    match index {
                        0 | 1 => return Some(1),
                        2 | 3 => return Some(2),
                        4 | 5 => return Some(3),
                        6 | 7 => return Some(4),
                        8 | 9 => return Some(5),
                        10 | 11 => return Some(6),
                        12 | 13 => return Some(7),
                        14 | 15 => return Some(8),
                        16 | 17 => return Some(9),
                        _ => continue,
                    };
                }
            }
        }

        return None;
    }
}

#[cfg(test)]
mod test {
    use crate::DigitParser;

    #[test]
    pub fn first_digit_return_some_for_valid_input() {
        // given
        let initial = "abcone2threexyz";
        let digit_parser = DigitParser::new(initial);

        // when
        let first_digit = digit_parser.first_digit();

        // then
        assert_eq!(first_digit, Some(1));
    }

    #[test]
    pub fn first_digit_return_none_for_invalid_input() {
        // given
        let initial = "lajferuwojsfldkfw";
        let digit_parser = DigitParser::new(initial);

        // when
        let first_digit = digit_parser.first_digit();

        // then
        assert_eq!(first_digit, None);
    }

    #[test]
    pub fn first_digit_return_valid_resut_for_spelled_digit() {
        // given
        let initial = "nineeightseven2";
        let digit_parser = DigitParser::new(initial);

        // when
        let first_digit = digit_parser.first_digit();

        // then
        assert_eq!(first_digit, Some(9));
    }

    #[test]
    pub fn first_digit_return_valid_resut_for_numeric_digit() {
        // given
        let sample = [
            ("pqrs7tsixteen", 7, "in the middle of string"),
            ("2pqrstsixteen", 2, "in the start of string"),
            ("pqrstsiixteen4", 4, "in the end of string"),
        ];
        for (initial, expected, position) in sample {
            let digit_parser = DigitParser::new(initial);

            // when
            let first_digit = digit_parser.first_digit();

            // then
            assert_eq!(
                first_digit,
                Some(expected),
                "could not parse input {:?} when digit is {:}",
                initial,
                position
            );
        }
    }

    #[test]
    pub fn last_digit_return_some_for_valid_input() {
        // given
        let initial = "abcone2threexyz";
        let digit_parser = DigitParser::new(initial);

        // when
        let last_digit = digit_parser.last_digit();

        // then
        assert_eq!(last_digit, Some(3));
    }

    #[test]
    pub fn last_digit_return_none_for_invalid_input() {
        // given
        let initial = "lajferuwojsfldkfw";
        let digit_parser = DigitParser::new(initial);

        // when
        let last_digit = digit_parser.last_digit();

        // then
        assert_eq!(last_digit, None);
    }

    #[test]
    pub fn last_digit_return_valid_resut_for_spelled_digit() {
        // given
        let initial = "nineeightsevensxkdjf";
        let digit_parser = DigitParser::new(initial);

        // when
        let first_digit = digit_parser.last_digit();

        // then
        assert_eq!(first_digit, Some(7));
    }

    #[test]
    pub fn last_digit_return_valid_resut_for_numeric_digit() {
        // given
        let sample = [
            ("pqrs7tsixteen", 6, "in the middle of string"),
            ("4pqrstsiixteen", 4, "in the start of string"),
            ("pqrstsixteen2", 2, "in the end of string"),
        ];
        for (initial, expected, position) in sample {
            let digit_parser = DigitParser::new(initial);

            // when
            let last_digit = digit_parser.last_digit();

            // then
            assert_eq!(
                last_digit,
                Some(expected),
                "could not parse input {:?} when digit is {:}",
                initial,
                position
            );
        }
    }
}

trait ICalibrationParser: Iterator {
    fn parse_digits(self) -> CalibrationParser<Self>
    where
        Self: Sized;
}

pub struct CalibrationParser<T>
where
    T: Iterator,
{
    inner: T,
}

impl<'a, T> Iterator for CalibrationParser<T>
where
    T: Iterator<Item = &'a str>,
{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(initial) = self.inner.next() {
            let digit_parser = DigitParser::new(initial);
            let first = digit_parser.first_digit().unwrap();
            let last = digit_parser.last_digit().unwrap();

            return Some(first * 10 + last);
        }
        None
    }
}

impl<'a, T> ICalibrationParser for T
where
    T: Iterator<Item = &'a str>,
    T: Sized,
{
    fn parse_digits(self) -> CalibrationParser<Self> {
        CalibrationParser { inner: self }
    }
}

fn main() {
    let file_name = "input.txt";
    let calibration: u64 = std::fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .parse_digits()
        .map(|d| {
            println!("{}", d);
            d
        })
        .sum();

    println!("Calibration value is {calibration}");
}
