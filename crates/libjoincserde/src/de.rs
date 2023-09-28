use crate::error::Result;
use serde::Deserialize;

pub struct Deserializer<'de> {
    _input: &'de str,
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        Deserializer { _input: input }
    }
}

pub fn from_str<'a, T>(_s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    unimplemented!()
}
