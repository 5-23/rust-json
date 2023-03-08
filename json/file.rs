use std::{path::Path};

use super::implement::Json;

#[allow(unused)]
pub struct File<P: AsRef<Path>>{
    path: P
}


#[allow(unused)]
impl<P: AsRef<Path>> File<P>{
    pub fn new(path: P) -> Self{
        Self{ path }
    }
    fn read(&self) -> Result<Json, &'static str>{
        todo!()
    }
    fn write(&self) -> Result<bool, bool>{
        todo!()
    }
}