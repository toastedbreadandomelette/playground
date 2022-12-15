mod container;
mod parser;
mod error;
mod common;

fn main () {
    let mut pr = parser::byte_parsing::parse_str("[1,2,true,false,\"some_string\",{'some':'object', 'and': {'some': 'nested objects', 'as':['well',{'here':'too'},null,12345354]}}]");
    let object = pr.unwrap();
    println!("{}", object);
    println!("{}", object[2]);
    println!("{}", object[5]["and"]["as"]);
}
