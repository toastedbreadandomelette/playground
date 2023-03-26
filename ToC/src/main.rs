mod common;
mod json_parser;

fn main() {
    let mut pr = json_parser::parser::parse_str("[1,2,true,false,\"some_\\\"string\",{'some':'object', 'and': {'some': 'nested objects', 'as':['well',{'here':'too'},null,12345354]}}]");
    let object = pr.unwrap();
    println!("{}", object);
    println!("here1 {}\n", object[2].get_bool().unwrap());
    println!("here2 {}\n", object[5]["and"]["as"]);
}
