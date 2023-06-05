use std::{
    error::Error,
    fs::{self, File},
    io::{BufReader, BufWriter, ErrorKind},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

pub struct Bucket {
    pub rdns: String,
    pub shared: bool,
}

pub enum BucketChild {
    Resource(String),
    SubBucket(String)
}

impl Bucket {
    pub fn path(&self) -> PathBuf {
        let rdns_path = self.rdns.replace('.', "/");
        let mut path = PathBuf::from(dirs::data_dir().unwrap());
        path.push("bucket");
        path.push(rdns_path);
        path
    }

    pub fn new(rdns: String, shared: bool) -> Self {
        let result = Self { rdns, shared };
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

    pub fn list_contents(&self) -> Result<Vec<BucketChild>, Box<dyn Error>> {
        todo!()
    }

}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
