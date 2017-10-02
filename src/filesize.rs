use std::fmt;

#[derive(Debug)]
enum FileSize {
    B,
    KB,
    MB,
    GB,
    TB,
}

impl fmt::Display for FileSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FileSize {
    fn value(&self) -> u64 {
        match *self {
            FileSize::B => 1,
            FileSize::KB => 1_024,
            FileSize::MB => 1_048_576,
            FileSize::GB => 1_073_741_824,
            FileSize::TB => 1_099_511_627_776,
        }
    }
}

#[derive(Debug)]
pub struct StringFileSize(String);

impl fmt::Display for StringFileSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Get a reference to the content of the StringFileSize struct
        let &StringFileSize(ref value) = self; 
        write!(f, "{}", value)
    }
}

impl From<u64> for StringFileSize {
    fn from(size: u64) -> Self {
        let (file_size, unit) = match size {
        0...999 => (size as f64, FileSize::B),
        1_000...999_999 => (size as f64 / FileSize::KB.value() as f64, FileSize::KB),
        1_000_000...999_999_999 => (size as f64 / FileSize::MB.value() as f64, FileSize::MB),
        1_000_000_000...999_999_999_999 => {
            (size as f64 / FileSize::GB.value() as f64, FileSize::GB)
        }
        _ => (size as f64 / FileSize::TB.value() as f64, FileSize::TB),
    };

        StringFileSize(
            format!("{:.2} {}", file_size, unit)
        )
    }
}

