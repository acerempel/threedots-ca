use crate::link::Link;

pub struct Nav<'a> {
    pub header: &'a [Link<'a>],
    pub footer: &'a [Link<'a>],
}