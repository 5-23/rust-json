use super::types::*;

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
    ( null ) => { Type::Null };
    ( $($k: tt: $v: tt), * ) => {
        {
            let mut j = Json::new();
            $(
                j.set($k, json!{$v});
            )*
            j
        }
    };
    ( {$($k: tt: $v: tt), *} ) => {
        {
            let mut j = Json::new();
            $(
                j.set($k, json!{$v});
            )*
            j
        }
    };
    ( $t: tt ) => { Type::transform($t) };    
}