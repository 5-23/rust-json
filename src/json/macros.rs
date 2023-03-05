use super::types::*;
use std::collections::HashMap;

/// this json! macro
/// 
/// ```
/// json!{
///     "one": 1,
///     "two": "two"
///     "three?": false
/// }
#[macro_export]
macro_rules! json {
    () => {};
    ( null ) => { Type::Null };
    ( {$($k: tt: $v: tt), *} ) => {
        {
            let mut j: Json = Json::new();
            $(
                j.set($k, json!($v));
            )*
            Type::transform(j)
        }
    };
    ( [$($e: tt), *] ) => {
        {
            let mut arr = vec![];
            $(arr.push(Type::transform($e));)*
            Type::transform(arr)
        }
    };
    ( $($k: tt: $v: tt), * ) => {
        {
            let mut j = Json::new();
            $(
                j.set($k, json!($v));
            )*
            j
        }
    };
    ( $t: tt ) => { Type::transform($t) };    
}