#[cfg(test)]
mod test;

use super::simple_obj;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SRSEI {
    pub hashes: Vec<String>,
}
simple_obj!(SRSEI, "SRSEI", hashes);
