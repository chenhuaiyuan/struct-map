use std::collections::HashMap;
use struct_map::{FieldValue, ToHashMap};

#[derive(ToHashMap, Debug)]
struct Foo {
    a: i32,
    b: i32,
}

fn main() {
    let foo = Foo { a: 1, b: 2 };

    let b = foo.to_map();
    println!("{:?}", b); // output {"b": Int(2), "a": Int(1)}

    let mut map: HashMap<String, FieldValue> = HashMap::new();
    map.insert("a".to_owned(), FieldValue::Int(3));
    map.insert("b".to_owned(), FieldValue::Int(4));
    let test = Foo::from_map(map).unwrap();
    println!("{:?}", test); // output: Foo { a: 3, b: 4 }
}
