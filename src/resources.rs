use std::ffi::CString;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};
use crate::util;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "I/O Error")]
    Io(io::Error),
    #[fail(display = "File contains null character")]
    FileContainsNil,
    #[fail(display = "Failed to find executable path")]
    FailToGetPath,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

pub struct Resources {
   root_path: PathBuf,
}

impl Resources {
    pub fn from_rel_path(rel_path: &Path) -> Result<Resources, Error> {
        let root_path = std::env::current_exe()
            .map_err(|_| Error::FailToGetPath)?
            .parent()
            .ok_or(Error::FailToGetPath)?
            .join(rel_path);

        Ok(Resources {
            root_path,
        })
    }

    pub fn load(&self, resource_name: &str) -> Result<CString, Error> {
        let mut file = File::open(self.root_path.join(resource_name))?;
        
        let mut buffer = Vec::with_capacity(file.metadata()?.len() as usize + 1);
        file.read_to_end(&mut buffer)?;
        
        // check for nil
        if buffer.iter().find(|&&x| x == 0).is_some() {
            return Err(Error::FileContainsNil);
        }
        
        Ok(unsafe { CString::from_vec_unchecked(buffer) })
    }
}