use std::{
    error::Error,
    fs::{self, File},
    io::{self, BufReader, BufWriter, ErrorKind},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

pub struct Bucket {
    pub rdns: String,
    pub shared: bool,
}

pub enum BucketChild {
    Resource(String),
    SubBucket(String),
}

impl Bucket {
    pub fn path(&self) -> PathBuf {
        let rdns_path = self.rdns.replace('.', "/");
        let mut path = dirs::data_dir().unwrap();
        path.push("bucket");
        path.push(rdns_path);
        path
    }

    pub fn new<S>(rdns: S, shared: bool) -> Self
    where
        S : Into<String>
    {
        let result = Self { rdns: rdns.into(), shared };
        let path = result.path();

        if let Err(e) = fs::create_dir_all(&path) {
            if e.kind() == ErrorKind::AlreadyExists {
                assert!(fs::metadata(&path).unwrap().file_type().is_dir());
            }
        }

        result
    }

    pub fn get_resource_path(&self, resource: String) -> PathBuf {
        self.path().join(resource)
    }

    pub fn read<R>(&self, resource: String) -> Result<R, Box<dyn Error>>
    where
        R: for<'a> Deserialize<'a>,
    {
        Ok(serde_json::from_reader(BufReader::new(File::open(
            self.get_resource_path(resource),
        )?))?)
    }

    pub fn write<R>(&self, resource: String, data: R) -> Result<(), Box<dyn Error>>
    where
        R: Serialize,
    {
        Ok(serde_json::to_writer(
            BufWriter::new(File::open(self.get_resource_path(resource))?),
            &data,
        )?)
    }

    pub fn list_contents(&self) -> Result<Vec<BucketChild>, io::Error> {
        let dir = fs::read_dir(self.path())?;

        let entries = dir.map(|entry| {
            entry.and_then(|entry| {
                if entry.file_type()?.is_dir() {
                    Ok(BucketChild::SubBucket(
                        entry.file_name().to_string_lossy().to_string(),
                    ))
                } else {
                    Ok(BucketChild::Resource(
                        entry.file_name().to_string_lossy().to_string(),
                    ))
                }
            })
        });

        let mut result = Vec::new();
        for entry in entries {
            result.push(entry?);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bucket_path() {
        let bucket = Bucket::new("com.example.Example", false);
        assert_eq!(bucket.path(), dirs::data_dir().unwrap().join("bucket/com/example/Example"));
    }
}
