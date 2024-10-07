#[derive(Debug)]
struct Cat<'a> {
    name: &'a str,
    age: u8,
    weight: i8
}

fn main() {
    let name = "Romad";
    let age = 13;
    let weight = 13;
    let romad = Cat { name, age, weight };

    // Pretty print
    println!("{:#?}", romad);
}