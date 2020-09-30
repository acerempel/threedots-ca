use relative_path::{RelativePath, RelativePathBuf};
use std::path::Path;
use anyhow::Result;

pub fn strip_input_dir(input_dir: &str, input_path_real: &Path) -> Result<RelativePathBuf> {
    let input_path_really = RelativePath::from_path(input_path_real)?;
    Ok(RelativePath::new(input_dir).relative(input_path_really))
}

use std::fs;

pub fn create_parent_directories<T: AsRef<Path>>(output: &T) -> Result<()> {
    if let Some(parent) = output.as_ref().parent() {
        fs::DirBuilder::new().recursive(true).create(parent)?;
    }; Ok(())
}