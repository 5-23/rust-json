use std::collections::HashMap;

use super::implement::Json;
#[derive(Clone)]
#[allow(unused)]
pub enum Type{
    Null,
    Int(i128),
    Float(f64),
    String(String),
    Bool(bool),
    Json(HashMap<&'static str, Type>),
    Array(Vec<Type>)
}

#[allow(unused)]
impl Type {
    pub fn parse_int(&self) -> Result<&i128, &'static str>{
        if let Self::Int(i) = self{
            Result::Ok(i)
        }else{
            Err("is not Integer")
        }
    }
    
    pub fn parse_str(&self) -> Result<&String, &'static str>{
        if let Self::String(s) = self{
            Result::Ok(s)
        }else{
            Err("is not String")
        }
    }
    
    pub fn parse_bool(&self) -> Result<&bool, &'static str>{
        if let Self::Bool(b) = self{
            Result::Ok(b)
        }else{
            Err("is not Boolean")
        }
    }

    pub fn parse_float(&self) -> Result<&f64, &'static str>{
        if let Self::Float(f) = self{
            Result::Ok(f)
        }else{
            Err("is not Float")
        }
    }

    pub fn parse_array(&self) -> Result<&Vec<Type>, &'static str>{
        if let Self::Array(v) = self{
            Result::Ok(v)
        }else{
            Err("is not Array")
        }
    }
    
    pub fn parse_json(&self) -> Result<&HashMap<&'static str, Type>, &'static str>{
        if let Self::Json(j) = self{
            Result::Ok(j)
        }else{
            Err("is not Json")
        }
    }
}


impl std::fmt::Debug for Type{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.alternate(){
            match self {
                Self::Bool(v) => write!(f, "{v:?}"),
                Self::Int(v) => write!(f, "{v:?}"),
                Self::Float(v) => write!(f, "{v:?}"),
                Self::String(v) => write!(f, "{v:?}"),
                Self::Json(v) => write!(f, "{:#?}", v),
                Self::Array(v) => {
                    write!(f, "{:#?}", v)
                },
                Self::Null => write!(f, "null"),
            }
        }else{
            match self {
                Self::Bool(v) => write!(f, "{v:?}"),
                Self::Int(v) => write!(f, "{v:?}"),
                Self::Float(v) => write!(f, "{v:?}"),
                Self::String(v) => write!(f, "{v:?}"),
                Self::Json(v) => write!(f, "{:?}", v),
                Self::Array(v) => {
                    write!(f, "{:?}", v)
                },
                Self::Null => write!(f, "null"),
            }
        }
    }
}

// impl std::fmt::Arguments for Type{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::Bool(v) => write!(f, "{v:?}"),
//             Self::Int(v) => write!(f, "{v:?}"),
//             Self::String(v) => write!(f, "{v:?}"),
//             Self::Json(v) => write!(f, "{:?}", v),
//             Self::Array(v) => {
//                 write!(f, "{:?}", v)
//             },
//             Self::Null => write!(f, "null"),
//         }
//     }
// }

pub trait Transform<T>{
    fn transform(value: T) -> Self;
}

pub trait ToJsonType<T>{
    fn to_json_type(&self) -> Type;
}

impl ToJsonType<String> for String{
    fn to_json_type(&self) -> Type {
        Type::String(self.to_string())
    }
}

impl ToJsonType<&str> for &str{
    fn to_json_type(&self) -> Type {
        Type::String(self.to_string())
    }
}

impl ToJsonType<bool> for bool{
    fn to_json_type(&self) -> Type {
        Type::Bool(*self)
    }
}

impl ToJsonType<Vec<Type>> for Vec<Type>{
    fn to_json_type(&self) -> Type {
        Type::Array(self.clone())
    }
}

impl ToJsonType<f32> for f32{
    fn to_json_type(&self) -> Type {
        Type::Float(*self as f64)
    }
}
impl ToJsonType<f64> for f64{
    fn to_json_type(&self) -> Type {
        Type::Float(*self)
    }
}



impl Transform<String> for Type{
    fn transform(value: String) -> Self {
        Self::String(value)
    }
}

impl Transform<&str> for Type{
    fn transform(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl Transform<bool> for Type{
    fn transform(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl Transform<f32> for Type{
    fn transform(value: f32) -> Self {
        Self::Float(value as f64)
    }
}

impl Transform<f64> for Type{
    fn transform(value: f64) -> Self {
        Self::Float(value)
    }
}


impl Transform<HashMap<&'static str, Type>> for Type{
    fn transform(value: HashMap<&'static str, Type>) -> Self {
        Self::Json(value)
    }
}

impl Transform<Json> for Type{
    fn transform(value: Json) -> Self {
        value.to_json_type()
    }
}

impl Transform<Vec<Type>> for Type{
    fn transform(value: Vec<Type>) -> Self {
        Self::Array(value)
    }
}

/// integer.to_json_type()
/// Type::transform(integer)
macro_rules! integers {
    ($($i: tt) *) => {
        $(
            impl Transform<$i> for Type{
                fn transform(value: $i) -> Self {
                    Self::Int(value as i128)
                }
            }

            impl ToJsonType<$i> for $i{
                fn to_json_type(&self) -> Type {
                    Type::Int(*self as i128)
                }
            }
        )*
    };
}
integers![isize i8 i16 i32 i64 i128 usize u8 u16 u32 u64 u128];