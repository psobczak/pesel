use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("Incorrect input string length: `{0}` (length must be 11).")]
    InvalidLength(usize),
    #[error("Input contains invalid character: `{0:?}`. Only numeric chars are allowed.")]
    NonNumericCharacter(Vec<char>),
}

#[derive(Debug, PartialEq)]
pub enum Sex {
    Male,
    Female,
}

impl From<char> for Sex {
    fn from(value: char) -> Self {
        match value {
            '0' | '2' | '4' | '6' | '8' => Self::Female,
            _ => Self::Male,
        }
    }
}

#[derive(Debug)]
pub struct Pesel {
    date_of_birth: String,
    serial_number: String,
    sex: Sex,
    control_number: String,
}

impl Pesel {
    pub fn new<T: AsRef<str>>(string: T) -> Result<Self, Error> {
        let string = string.as_ref();

        if string.len() != 11 {
            return Err(Error::InvalidLength(string.len()));
        }

        let wrong_chars: Vec<char> = string.chars().filter(|c| !char::is_numeric(*c)).collect();
        if !wrong_chars.is_empty() {
            return Err(Error::NonNumericCharacter(wrong_chars));
        }

        Ok(Self {
            date_of_birth: string[..6].to_string(),
            serial_number: string[6..10].to_string(),
            sex: Sex::from(string[9..10].chars().next().unwrap()),
            control_number: string[10..].to_string(),
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn if_input_length_ne_11() {
        assert_eq!(
            Pesel::new("1111111111").err(),
            Some(Error::InvalidLength(10))
        );
        assert_eq!(
            Pesel::new("111111111111").err(),
            Some(Error::InvalidLength(12))
        );
    }

    #[test]
    fn create_from_valid_input() {
        let input = "44051401458";
        let pesel = Pesel::new(input).unwrap();
        assert_eq!(pesel.date_of_birth, "440514");
        assert_eq!(pesel.serial_number, "0145");
        assert_eq!(pesel.sex, Sex::Male);
        assert_eq!(pesel.control_number, "8");
    }

    #[test]
    fn input_contains_non_numeric_chars() {
        let input = "11111111abc";
        assert_eq!(
            Pesel::new(input).err(),
            Some(Error::NonNumericCharacter(vec!['a', 'b', 'c']))
        )
    }

    #[test]
    fn even_number_is_female() {
        let input = "95071505325";
        let pesel = Pesel::new(input).unwrap();
        assert_eq!(pesel.sex, Sex::Female);
    }

    #[test]
    fn odd_number_is_male() {
        let input = "95071505335";
        let pesel = Pesel::new(input).unwrap();
        assert_eq!(pesel.sex, Sex::Male);
    }
}
