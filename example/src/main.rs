use structmap::ToHashMap;

#[derive(ToHashMap, Debug)]
struct Foo {
    a: i32,
    b: i32,
}

fn main() {
    let foo = Foo { a: 1, b: 2 };

    let b = foo.to_map();
    println!("{:?}", b); // {"b": Int(2), "a": Int(1)}
}
