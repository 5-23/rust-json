use std::collections::HashMap;
#[derive(Clone)]
pub enum Type{
    Null,
    Int(i128),
    String(String),
    Bool(bool),
    Json(HashMap<&'static str, Type>),
}


impl std::fmt::Debug for Type{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(v) => write!(f, "{v:?}"),
            Self::Int(v) => write!(f, "{v:?}"),
            Self::String(v) => write!(f, "{v:?}"),
            Self::Json(v) => write!(f, "{:?}", v),
            Self::Null => write!(f, "null"),
        }
    }
}

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