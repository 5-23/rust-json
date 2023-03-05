use std::collections::HashMap;
use super::types::Type;

pub struct Json{
    hash: HashMap<&'static str, Type>
}

impl Json{
    pub fn new() -> Self{
        Self { hash: HashMap::new() }
    }

    pub fn set(&mut self, key: &'static str, value: Type){
        self.hash.insert(key, value);
    }

    pub fn get(&self, key: &'static str) -> Option<&Type>{
        self.hash.get(&key)
    }
}

impl std::fmt::Debug for Json{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}