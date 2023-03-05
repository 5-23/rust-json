use std::collections::HashMap;
use super::types::{Type, ToJsonType};

#[derive(Clone)]
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

impl ToJsonType<String> for Json{
    fn to_json_type(&self) -> Type {
        Type::Json(self.hash.clone())
    }
}


impl std::fmt::Debug for Json{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate(){
            write!(f, "{:#?}", self.hash)
        }else{
            write!(f, "{:?}", self.hash)
        }
    }
}