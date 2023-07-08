use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Index, IndexMut};
use std::option::Option::Some;

/// A Container that has ability to store different kind
/// of data at a time. This includes basic data types like
/// - Null
/// - Integer
/// - Unsigned Integer
/// - Real Number
/// - Truth/Fallacy (Boolean)
/// - String values (These are displayed in double inverted quotes)
///
/// ## Examples for basic types
/// ```
/// let storage: Container = Container::Decimal(2e9+76);
/// let mut str: Container = Container::Str("here".to_string());
/// println!("{}, and {}", str);          // prints "here" and 2000000076
/// ```
///
/// And combination of such like:
/// - Array (An expandable, randomly accessible list)
/// - Set (A HashSet that stores unique values)
/// - Object (A HashMap, that associates a string key with a value)
///
/// ```
/// let array_container: Container = Container::new_array();
/// array_container.push(Container::Boolean(true));
/// array_container.push(Container::Number(1<<32));
/// array_container.push(Container::Decimal(2.34));
///
/// let object_container: Container = Container::new_object();
/// object_container.insert("key1".to_string(), Container::Str("hello".to_string()));
///
/// array_container.push(object_container);
/// println!("{}", array_container); /// dumps [true,4294967296,2.34,{"key1":"hello"}] in pretty fashion
/// ```
/// Todo:
/// - [ ] Support Date and raw binary data type
///
#[derive(Debug)]
pub enum Container {
    /// Representing an object of null type
    Null,
    /// A 16 byte signed integer
    Number(i64),
    /// A 16 byte unsigned integer
    Unsigned(u64),
    /// An 8 byte real number
    Decimal(f64),
    /// boolean value
    Boolean(bool),
    /// String
    Str(String),
    /// Dynamic allocated that can store
    /// these containers in consecutive fashion
    /// of their insertion.
    Array(Vec<Container>),
    /// Set containing unique container elements
    /// identified by either
    ///
    /// - Value or
    /// - Values inside these elements
    Set(HashSet<Container>),
    /// Key value pair, where key is string
    /// and value can be any of these types
    Object(HashMap<String, Container>),
}

impl Clone for Container {
    /// Creates an exact clone of self.
    fn clone(&self) -> Self {
        match *self {
            Self::Number(ref element) => Self::Number(*element),
            Self::Unsigned(ref element) => Self::Unsigned(*element),
            Self::Decimal(ref element) => Self::Decimal(*element),
            Self::Boolean(ref element) => Self::Boolean(*element),
            Self::Str(ref element) => Self::Str(String::from(element)),
            Self::Array(ref array) => Self::Array(array.clone()),
            Self::Object(ref object) => Self::Object(object.clone()),
            Self::Set(ref set) => Self::Set(set.clone()),
            Self::Null => Self::Null,
        }
    }
}

impl Hash for Container {
    fn hash<H: Hasher>(&self, s: &mut H) {
        match *self {
            Self::Number(ref v) => v.hash(s),
            Self::Unsigned(ref v) => v.hash(s),
            Self::Boolean(ref v) => v.hash(s),
            Self::Str(ref v) => v.hash(s),
            _ => (),
        }
    }
}

macro_rules! define_type_checks {
    ($gen_type:ident, $func:ident) => {
        pub fn $func(&self) -> bool {
            match *self {
                Self::$gen_type(_) => true,
                _ => false,
            }
        }
    };
}

impl Eq for Container {}

// == operator declaration
impl PartialEq for Container {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(this), Self::Number(other)) => this == other,
            (Self::Unsigned(this), Self::Unsigned(other)) => this == other,
            (Self::Decimal(this), Self::Decimal(other)) => this == other,
            (Self::Boolean(this), Self::Boolean(other)) => this == other,
            (Self::Str(ref this), Self::Str(ref other)) => this == other,
            (Self::Array(ref array), Self::Array(ref other_array)) => {
                array.len() == other_array.len()
                    && array.iter().zip(other_array.iter()).all(|(a, b)| a == b)
            }
            (Self::Set(ref set), Self::Set(ref other_set)) => {
                (set.len() == other_set.len())
                    && set.iter().all(|value| other_set.get(value) == Some(value))
            }
            (Self::Object(ref map_object), Self::Object(ref other_map_object)) => {
                (map_object.len() == other_map_object.len())
                    && map_object
                        .iter()
                        .all(|(key, value)| other_map_object.get(key) == Some(value))
            }
            (Self::Null, Self::Null) => true,
            _ => false,
        }
    }
}

// Display Object
impl fmt::Display for Container {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.dump_object(true, 4, ""))
    }
}

// To do: Implement index
impl Container {
    // Returned New Object
    #[inline(always)]
    pub fn new_object() -> Self {
        Self::Object(HashMap::new())
    }

    // Returns New Array Object
    #[inline(always)]
    pub fn new_array() -> Self {
        Self::Array(Vec::new())
    }

    // Returns new set
    #[inline(always)]
    pub fn new_set() -> Self {
        Self::Set(HashSet::new())
    }
    // Array: Push an item into array or an element into set:
    // Returns false if not inserted
    // Permissible for array type only
    pub fn push(&mut self, val: Self) -> bool {
        match *self {
            // Array push
            Self::Array(ref mut value) => {
                value.push(val);
                true
            }
            // Set push
            // To do: Define hash for HashSet
            Self::Set(ref mut value) => value.insert(val),
            _ => {
                println!("Error: The storage type should be of array or a set for pushing values.");
                false
            }
        }
    }

    // Insert/Replaces key value pair into Object
    // Returns true if success, else false.
    pub fn insert(&mut self, key: String, val: Self) -> bool {
        match *self {
            Self::Object(ref mut object) => {
                object.insert(key.to_owned(), val) != None
            }
            _ => {
                println!("Error: The storage should be of type Object");
                false
            }
        }
    }

    // Insert/Replaces key value pair into Object
    // Returns true if success, else false.
    pub fn insert_str(&mut self, key: &str, val: Self) -> bool {
        match *self {
            Self::Object(ref mut object) => {
                object.insert(key.to_owned(), val) != None
            }
            _ => {
                println!("Error: The storage should be of type Object");
                false
            }
        }
    }

    // Print the Stored value
    pub fn dump_object(&self, indent: bool, indent_size: u8, white_space: &str) -> String {
        match *self {
            Self::Array(ref value) => {
                if !indent {
                    format!(
                        "[{}]",
                        value
                            .iter()
                            .map(|element| element.dump_object(indent, indent_size, white_space))
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                } else {
                    let mut space: String = (0..indent_size).map(|_| ' ').collect();
                    if value.len() == 0 {
                        "[]".to_string()
                    } else {
                        space += white_space;
                        format!(
                            "[\n{}\n{}]",
                            value
                                .iter()
                                .map(|element| format!(
                                    "{}{}",
                                    space,
                                    element.dump_object(indent, indent_size, &space)
                                ))
                                .collect::<Vec<String>>()
                                .join(",\n"),
                            white_space
                        )
                    }
                }
            }
            Self::Object(ref value) => {
                if indent == false {
                    let (mut object_str, mut index) = ("{".to_string(), 0);
                    for (key, val) in value {
                        object_str.push_str(&format!(
                            "\"{}\":{}",
                            key,
                            val.dump_object(indent, indent_size, white_space)
                        ));
                        index += 1;
                        if index != value.len() {
                            object_str.push(',');
                        }
                    }
                    object_str.push('}');
                    object_str
                } else {
                    if value.len() == 0 {
                        "{}".to_string()
                    } else {
                        let (mut object_str, mut space, mut index) =
                            ("{\n".to_string(), white_space.to_string(), 0);
                        space += std::str::from_utf8(
                            &(0..indent_size).map(|_| b' ').collect::<Vec<u8>>()[..],
                        )
                        .unwrap();
                        // space += c;

                        for (key, val) in value {
                            object_str.push_str(&space);
                            object_str.push_str(&format!(
                                "\"{}\": {}",
                                key,
                                val.dump_object(indent, indent_size, &space)
                            ));
                            index += 1;
                            if index != value.len() {
                                object_str.push_str(",\n");
                            }
                        }
                        object_str.push_str(&format!("\n{}}}", white_space));
                        object_str
                    }
                }
            }
            Self::Set(ref value) => {
                if indent == false {
                    format!(
                        "({})",
                        value
                            .iter()
                            .map(|element| element.dump_object(indent, indent_size, white_space))
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                } else {
                    let mut space: String = (0..indent_size)
                        .map(|_| ' ')
                        .collect::<Vec<char>>()
                        .into_iter()
                        .collect();
                    if value.len() == 0 {
                        "()".to_string()
                    } else {
                        space += white_space;
                        format!(
                            "(\n{}\n{})",
                            value
                                .iter()
                                .map(|element| format!(
                                    "{}{}",
                                    space,
                                    element.dump_object(indent, indent_size, &space)
                                ))
                                .collect::<Vec<String>>()
                                .join(",\n"),
                            white_space
                        )
                    }
                }
            }
            Self::Number(value) => value.to_string(),
            Self::Unsigned(value) => value.to_string(),
            Self::Boolean(value) => value.to_string(),
            Self::Decimal(value) => value.to_string(),
            Self::Str(ref value) => format!("\"{}\"", value.to_string()),
            Self::Null => "null".to_string(),
        }
    }

    pub fn as_string(self) -> Option<String> {
        match self {
            Self::Str(value) => Some(value),
            _ => None,
        }
    }

    pub fn get_string(&self) -> Option<String> {
        match self {
            Self::Str(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_uint(&self) -> Option<u64> {
        match self {
            Self::Unsigned(value) => Some(*value),
            _ => None,
        }
    }

    pub fn get_int(&self) -> Option<i64> {
        match self {
            Self::Number(value) => Some(*value),
            _ => None,
        }
    }

    pub fn get_real(&self) -> Option<f64> {
        match self {
            Self::Decimal(value) => Some(*value),
            _ => None,
        }
    }

    pub fn get_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(*value),
            _ => None,
        }
    }

    define_type_checks!(Number, is_number);

    define_type_checks!(Unsigned, is_unsigned);

    define_type_checks!(Decimal, is_decimal);

    define_type_checks!(Boolean, is_bool);

    define_type_checks!(Str, is_str);

    define_type_checks!(Object, is_object);

    define_type_checks!(Set, is_set);

    pub fn is_null(&self) -> bool {
        match *self {
            Self::Null => true,
            _ => false,
        }
    }

    /// Returns the length of an object
    pub fn len(&self) -> usize {
        match *self {
            Self::Array(ref value) => value.len(),
            Self::Object(ref value) => value.len(),
            Self::Set(ref value) => value.len(),
            Self::Str(ref value) => value.len(),
            _ => 1usize,
        }
    }
}

impl Index<usize> for Container {
    type Output = Self;
    // Returns the value given the index (usize).
    fn index(&self, idx: usize) -> &Self::Output {
        match *self {
            Self::Array(ref value) => {
                if value.len() > idx {
                    &value.get(idx).unwrap()
                } else {
                    &Self::Null
                }
            }
            _ => &Self::Null,
        }
    }
}

impl Index<String> for Container {
    type Output = Self;
    // Returns the value given the string index
    fn index(&self, idx: String) -> &Self::Output {
        match *self {
            Self::Object(ref value) => {
                if let Some(value) = value.get(&idx) {
                    value
                } else {
                    &Self::Null
                }
            }
            _ => &Self::Null,
        }
    }
}

impl Index<&str> for Container {
    type Output = Self;
    // Returns the value given the string index
    fn index(&self, idx: &str) -> &Self::Output {
        match *self {
            Self::Object(ref value) => {
                if let Some(value) = value.get(&idx.to_owned()) {
                    value
                } else {
                    &Self::Null
                }
            }
            _ => &Self::Null,
        }
    }
}

impl IndexMut<usize> for Container {
    // Returns the value given the index (usize).
    fn index_mut(&mut self, index: usize) -> &mut Self {
        match *self {
            Self::Array(ref mut value) => {
                if value.len() > index {
                    &mut value[index]
                } else {
                    value.push(Self::Null);
                    value.last_mut().unwrap()
                }
            }
            _ => {
                // Log: Change into array warning
                *self = Self::new_array();
                self.push(Self::Null);
                &mut self[0]
            }
        }
    }
}

impl IndexMut<String> for Container {
    // Returns the value given the index (usize).
    fn index_mut(&mut self, idx: String) -> &mut Self {
        match *self {
            Self::Object(ref mut value) => {
                let exists = value.contains_key(&idx);
                if !exists {
                    value.insert(idx.to_string(), Self::Null);
                }
                value.get_mut(&idx).unwrap()
            }
            _ => {
                // Log: Change into array warning
                *self = Self::new_object();
                self.insert(idx.clone(), Self::Null);
                &mut self[idx]
            }
        }
    }
}

impl IndexMut<&str> for Container {
    // Returns the value given the index (usize).
    fn index_mut<'a>(&mut self, idx: &'a str) -> &mut Self {
        match *self {
            Self::Object(ref mut value) => {
                let key = idx.to_owned();
                if !value.contains_key(&key) {
                    value.insert(key.to_owned(), Self::Null);
                }
                value.get_mut(&key).unwrap()
            }
            _ => {
                // Log: Change into array
                *self = Self::new_object();
                self.insert(idx.to_owned(), Self::Null);
                &mut self[idx]
            }
        }
    }
}
