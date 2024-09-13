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

pub struct DigitParser<'a> {
    initial: &'a str,
}

impl<'a> DigitParser<'a> {
    pub fn new(initial: &'a str) -> Self {
        Self { initial }
    }

    pub fn first_digit(&self) -> Option<char> {
        let mut unparsed;
        for current in 0..self.initial.len() {
            unparsed = &self.initial[current..];
            if unparsed.starts_with(&['1', '2', '3', '4', '5', '6', '7', '8', '9']) {
                return unparsed.chars().next();
            }

            if unparsed.starts_with("one") {
                return Some('1');
            }

            if unparsed.starts_with("two") {
                return Some('2');
            }

            if unparsed.starts_with("three") {
                return Some('3');
            }

            if unparsed.starts_with("four") {
                return Some('4');
            }

            if unparsed.starts_with("five") {
                return Some('5');
            }

            if unparsed.starts_with("six") {
                return Some('6');
            }

            if unparsed.starts_with("seven") {
                return Some('7');
            }

            if unparsed.starts_with("eight") {
                return Some('8');
            }

            if unparsed.starts_with("nine") {
                return Some('9');
            }
        }

        return None;
    }

    pub fn last_digit(&self) -> Option<char> {
        let mut unparsed;
        for current in (0..self.initial.len()).rev() {
            unparsed = &self.initial[current..];
            if unparsed.starts_with(&['1', '2', '3', '4', '5', '6', '7', '8', '9']) {
                return unparsed.chars().next();
            }

            if unparsed.starts_with("one") {
                return Some('1');
            }

            if unparsed.starts_with("two") {
                return Some('2');
            }

            if unparsed.starts_with("three") {
                return Some('3');
            }

            if unparsed.starts_with("four") {
                return Some('4');
            }

            if unparsed.starts_with("five") {
                return Some('5');
            }

            if unparsed.starts_with("six") {
                return Some('6');
            }

            if unparsed.starts_with("seven") {
                return Some('7');
            }

            if unparsed.starts_with("eight") {
                return Some('8');
            }

            if unparsed.starts_with("nine") {
                return Some('9');
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
        assert_eq!(first_digit, Some('1'));
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
        assert_eq!(first_digit, Some('9'));
    }

    #[test]
    pub fn first_digit_return_valid_resut_for_numeric_digit() {
        // given
        let sample = [
            ("pqrs7tsixteen", '7', "in the middle of string"),
            ("2pqrstsixteen", '2', "in the start of string"),
            ("pqrstsiixteen4", '4', "in the end of string"),
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
        assert_eq!(last_digit, Some('3'));
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
        assert_eq!(first_digit, Some('7'));
    }

    #[test]
    pub fn last_digit_return_valid_resut_for_numeric_digit() {
        // given
        let sample = [
            ("pqrs7tsixteen", '6', "in the middle of string"),
            ("4pqrstsiixteen", '4', "in the start of string"),
            ("pqrstsixteen2", '2', "in the end of string"),
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

impl<'a, T> Iterator for CalibrationParser<T>
where
    T: Iterator<Item = &'a str>,
{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(initial) = self.inner.next() {
            let digit_parser = DigitParser::new(initial);
            let first = digit_parser.first_digit().unwrap();
            let last = digit_parser.last_digit().unwrap();
            return format!("{}{}", first, last).parse().ok();
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
    let calibration: u32 = std::fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .parse_digits()
        .sum();

    println!("Calibration value is {calibration}");
}
