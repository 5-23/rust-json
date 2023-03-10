use std::collections::HashMap;
use super::types::{Type, ToJsonType};

#[derive(Clone)]
#[allow(unused)]
pub struct Json{
    hash: HashMap<String, Type>
}

#[allow(unused)]
impl Json{
    pub fn new() -> Self{
        Self { hash: HashMap::new() }
    }

    pub fn init(hash: HashMap<String, Type>) -> Self{
        Self { hash }
    }

    pub fn set(&mut self, key: &'static str, value: Type){
        self.hash.insert(key.to_string(), value);
    }

    pub fn get(&self, key: &'static str) -> Option<&Type>{
        self.hash.get(&key.to_string())
    }
}
#[allow(unused)]
impl ToJsonType<String> for Json{
    fn to_json_type(&self) -> Type {
        Type::Json(Json::init(self.hash.clone()))
    }
}

#[allow(unused)]
impl std::fmt::Debug for Json{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate(){
            write!(f, "{:#?}", self.hash)
        }else{
            write!(f, "{:?}", self.hash)
        }
    }
}

#[allow(unused)]
impl std::ops::Index<&str> for Json{
    type Output = Type;
    fn index(&self, index: &str) -> &Self::Output {
        self.hash.get(index).unwrap()
    }
}

/// TODO: unwrap_or make

// impl std::ops::Index<(&str, Type)> for Json{
//     type Output = Type;
//     fn index(&self, index: (&str, Type)) -> &Self::Output {
//         let index_clone = index.clone();
//         self.hash.get(index_clone.0).unwrap_or(index.1)
//     }
// }
#[allow(unused)]
impl std::ops::IndexMut<&str> for Json{
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        &mut *self.hash.get_mut(index).unwrap()
    }
}