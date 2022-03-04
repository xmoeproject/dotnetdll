use std::borrow::Cow;
use super::attribute::Attribute;

#[derive(Debug, Clone)]
pub struct Module<'a> {
    pub attributes: Vec<Attribute<'a>>,
    pub name: Cow<'a, str>,
    pub mvid: [u8; 16],
}
impl<'a> Module<'a> {
    pub const fn new(name: Cow<'a, str>) -> Self {
        Self {
            attributes: vec![],
            name,
            mvid: [0; 16],
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExternalModuleReference<'a> {
    pub attributes: Vec<Attribute<'a>>,
    pub name: Cow<'a, str>,
}

#[derive(Debug, Clone)]
pub struct File<'a> {
    pub attributes: Vec<Attribute<'a>>,
    pub has_metadata: bool,
    pub name: Cow<'a, str>,
    pub hash_value: Cow<'a, [u8]>,
}
