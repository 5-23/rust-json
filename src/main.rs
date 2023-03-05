use std::collections::HashMap;
mod json;
use json::types::*;
use json::implement::*;

fn main(){
    let a = json! {
        "a": 1
    };
    println!("{a:?}")
}