mod json;

use std::collections::HashMap;

use json::types::*;
use json::implement::*;

fn main(){
    let mut a = json! {
        "a": 1,
        "b": true,
        "c": {
            "d": 1,
            "f": 2
        },
        "arr": [1, 2],
        "null": null
    };
    a["a"] = 1000.to_json_type();
    println!("{a:#?}");
    println!("a: {:?}", a["a"]);
}