#[cfg(test)]
mod test;

use super::simple_obj;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRGZONEINDEX {
    pub zones: Vec<String>,
}
simple_obj!(SRGZONEINDEX, "SRGZONEINDEX", zones);
