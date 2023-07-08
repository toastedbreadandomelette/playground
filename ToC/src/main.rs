mod common;
mod json_parser;

fn main() {
    let mut pr = json_parser::parser::parse_str(r#"{
        "tell": "me",
        "where": 123.98,
        "you": 1.9e28,
        "are": [
            1,2,3,4,5,6,7,8,9,10000.000987
        ],
        "i'll": {
            "come":  "for",
            "you": [
                "and",
                { "parse": "json" },
                true,
                false,
                { "eof": null }
            ]
        }
    }"#);
    let object: &mut crate::common::container::Container = pr.unwrap_mut();
    println!("{}", object);
    println!("{}", object["tell"]);
    object["tell"] = crate::common::container::Container::Decimal(12233.2);
    println!("{}", object["tell"]);

    let mut v = json_parser::parser::parse_str(r#"1.2344"#);
    let c = v.unwrap();
    println!("{}", c);
}
