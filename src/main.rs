mod json;

use std::collections::HashMap;

use json::types::*;
use json::implement::*;

fn main(){
    let a = json! {
        "a": 1,
        "b": true,
        "c": {
            "d": 1,
            "f": 2
        },
        "arr": [1, 2],
        "null": null
    };

    println!("{a:#?}");
}