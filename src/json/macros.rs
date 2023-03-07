use super::types::*;
use std::collections::HashMap;

/// this json! macro
/// 
/// ```
/// json!{
///     "int": 1,
///     "str": "1",
///     "flaot": 10.,
///     "bool": false,
///     "arr": [1, "one"],
///     "arr2": [1; 10],
///     "null": null,
///     "object": {
///         "int": 1,
///         "str": "1",
///         "flaot": 10.,
///         "bool": false,
///         "arr": [1, "one"],
///         "arr2": [1; 10],
///         "null": null,
///     }
/// }
/// ```
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
    ( [$e: tt; $len: tt] ) => {
        Type::transform(vec![Type::transform($e); $len])
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