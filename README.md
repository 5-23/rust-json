# rust-json

rust in json

## example
```rs
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
    a["a"] += 2.to_json_type();
    println!("{a:#?}");
    println!("a: {:?}", a["a"]);
}
```
