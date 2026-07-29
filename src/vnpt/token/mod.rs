use std::ffi::CStr;

pub struct Token<'a> {
    reader: Option<&'a CStr>,
}
impl<'a> Token<'a> {}
