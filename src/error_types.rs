use std::error::Error;
use std::convert::From;
use std::num::ParseIntError;
use std::fmt;

#[derive(Debug)]
pub enum FromStrError {
    InvalidInputLength(&'static str, usize, usize),
    MalformedInput(&'static str),
}

impl Error for FromStrError {
    fn description(&self) -> &str {
        match *self {
            FromStrError::InvalidInputLength(_, _, _) => "Str input was the wrong length",
            FromStrError::MalformedInput(_) => "Str input was malformed",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for FromStrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FromStrError::InvalidInputLength(field_name, expected, found) => {
                write!(f,
                       "\"{}\" field was wrong length: expected {} found {}",
                       field_name,
                       expected,
                       found)
            }
            FromStrError::MalformedInput(field_name) => {
                write!(f, "Input was malformed at \"{}\" field", field_name)
            }
        }
    }
}

#[derive(Debug)]
pub enum FromFenError {
    IncorrectNumberOfFields(usize),
    MalformedStringField(FromStrError),
    IntFieldParseError(ParseIntError),
}

impl From<ParseIntError> for FromFenError {
    fn from(err: ParseIntError) -> FromFenError {
        FromFenError::IntFieldParseError(err)
    }
}

impl From<FromStrError> for FromFenError {
    fn from(err: FromStrError) -> FromFenError {
        FromFenError::MalformedStringField(err)
    }
}

impl Error for FromFenError {
    fn description(&self) -> &str {
        match *self {
            FromFenError::IncorrectNumberOfFields(_) => "fen str had an incorrect numer of fields",
            FromFenError::MalformedStringField(ref err) => err.description(),
            FromFenError::IntFieldParseError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            FromFenError::IncorrectNumberOfFields(_) => None,
            FromFenError::MalformedStringField(ref err) => Some(err as &Error),
            FromFenError::IntFieldParseError(ref err) => Some(err as &Error),
        }
    }
}

impl fmt::Display for FromFenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FromFenError::IncorrectNumberOfFields(num_fields) => {
                write!(f, "Incorrect number of fields, found {}", num_fields)
            }
            FromFenError::MalformedStringField(ref err) => write!(f, "{}", err),
            FromFenError::IntFieldParseError(ref err) => write!(f, "{}", err),
        }
    }
}
