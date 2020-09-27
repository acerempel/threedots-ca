use std::path::{Path, PathBuf};
use anyhow::Result;

pub struct NominalPath<T: PathOrientation>{ pub path: String, phantom: PhantomData<T> }
pub struct RealPath<T: PathOrientation>{ pub path: PathBuf, phantom: PhantomData<T> }

impl<T: PathOrientation> AsRef<str> for NominalPath<T> { fn as_ref(&self) -> &str { self.path.as_ref() } }
impl<T: PathOrientation> AsRef<Path> for NominalPath<T> { fn as_ref(&self) -> &Path { self.path.as_ref() } }
impl<T: PathOrientation> AsRef<Path> for RealPath<T> { fn as_ref(&self) -> &Path { &self.path } }

use std::fmt;
impl<T: PathOrientation> fmt::Display for NominalPath<T> { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.path.fmt(f) } }

impl<T: PathOrientation> From<String> for NominalPath<T> {
    fn from(s: String) -> Self {
        NominalPath { path: s, phantom: PhantomData }
    }
}

impl<T: PathOrientation> From<PathBuf> for RealPath<T> {
    fn from(s: PathBuf) -> Self {
        RealPath { path: s, phantom: PhantomData }
    }
}

pub trait PathOrientation {}

pub struct Input;
pub struct Output;
impl PathOrientation for Input {}
impl PathOrientation for Output {}

use std::marker::PhantomData;

pub fn real_input_path(input_path: &Path) -> RealPath<Input> {
    RealPath { path: input_path.to_path_buf(), phantom: PhantomData }
}

pub fn prepend_output_dir(output_dir: &Path, path: NominalPath<Output>) -> RealPath<Output> {
    RealPath{ path: output_dir.join(path.path), phantom: path.phantom }
}

pub fn strip_input_dir(input_dir: &str, input_path_real: &RealPath<Input>) -> Result<NominalPath<Input>> {
    // I don't think `strip_prefix` is quite this smart.
    let stripped =
        if input_dir == "." { &input_path_real.path }
        else { input_path_real.path.strip_prefix(input_dir)? };
    stripped.to_str()
        .map(|s| Ok(NominalPath { path: s.to_owned(), phantom: PhantomData }))
        .unwrap_or_else(|| Err(anyhow!("not unicode path! {:?}", stripped)))
}

use std::fs;

pub fn create_parent_directories<T: AsRef<Path>>(output: &T) -> Result<()> {
    for parent in output.as_ref().parent().iter() { fs::DirBuilder::new().recursive(true).create(parent)?; }; Ok(())
}