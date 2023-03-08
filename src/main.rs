mod json;

use json::types::*;
use json::implement::*;

fn main(){
    let mut a = json! {
        "a": 1,
        "b": true,
        "f": 1.,
        "c": {
            "d": 1,
            "f": 2
        },
        "arr": [1, "aa"],
        "null": null
    };
    // let b = a;
    a["a"] += 2.to_json_type();
    println!("{a:#?}");
    println!("a: {:?}", a["a"]);
}