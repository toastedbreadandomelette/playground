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
    fn clone(&self) -> Container {
        match *self {
            Container::Number(ref element) => Container::Number(*element),
            Container::Unsigned(ref element) => Container::Unsigned(*element),
            Container::Decimal(ref element) => Container::Decimal(*element),
            Container::Boolean(ref element) => Container::Boolean(*element),
            Container::Str(ref element) => Container::Str(String::from(element)),
            Container::Array(ref array) => Container::Array(array.clone()),
            Container::Object(ref object) => Container::Object(object.clone()),
            Container::Set(ref set) => Container::Set(set.clone()),
            Container::Null => Container::Null,
        }
    }
}

impl Hash for Container {
    fn hash<H: Hasher>(&self, s: &mut H) {
        match *self {
            Container::Number(ref v) => v.hash(s),
            Container::Unsigned(ref v) => v.hash(s),
            Container::Boolean(ref v) => v.hash(s),
            Container::Str(ref v) => v.hash(s),
            _ => (),
        }
    }
}

macro_rules! define_type_checks {
    ($gen_type:ident, $func:ident) => {
        pub fn $func(&self) -> bool {
            match *self {
                Container::$gen_type(_) => true,
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
            (Container::Number(this), Container::Number(other)) => this == other,
            (Container::Unsigned(this), Container::Unsigned(other)) => this == other,
            (Container::Decimal(this), Container::Decimal(other)) => this == other,
            (Container::Boolean(this), Container::Boolean(other)) => this == other,
            (Container::Str(ref this), Container::Str(ref other)) => this == other,
            (Container::Array(ref array), Container::Array(ref other_array)) => {
                array.len() == other_array.len()
                    && array.iter().zip(other_array.iter()).all(|(a, b)| a == b)
            }
            (Container::Set(ref set), Container::Set(ref other_set)) => {
                (set.len() == other_set.len())
                    && set.iter().all(|value| other_set.get(value) == Some(value))
            }
            (Container::Object(ref map_object), Container::Object(ref other_map_object)) => {
                (map_object.len() == other_map_object.len())
                    && map_object
                        .iter()
                        .all(|(key, value)| other_map_object.get(key) == Some(value))
            }
            (Container::Null, Container::Null) => true,
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
        Container::Object(HashMap::new())
    }

    // Returns New Array Object
    #[inline(always)]
    pub fn new_array() -> Self {
        Container::Array(Vec::new())
    }

    // Returns new set
    #[inline(always)]
    pub fn new_set() -> Self {
        Container::Set(HashSet::new())
    }
    // Array: Push an item into array or an element into set:
    // Returns false if not inserted
    // Permissible for array type only
    pub fn push(&mut self, val: Self) -> bool {
        match *self {
            // Array push
            Container::Array(ref mut value) => {
                value.push(val);
                true
            }
            // Set push
            // To do: Define hash for HashSet
            Container::Set(ref mut value) => value.insert(val),
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
            Container::Object(ref mut object) => {
                let exists = object.contains_key(&key);
                object.insert(key.to_string(), val);
                exists
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
            Container::Array(ref value) => {
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
                    let mut space: String = (0..indent_size).into_iter().map(|_| ' ').collect();
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
            Container::Object(ref value) => {
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
            Container::Set(ref value) => {
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
                        .into_iter()
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
            Container::Number(value) => value.to_string(),
            Container::Unsigned(value) => value.to_string(),
            Container::Boolean(value) => value.to_string(),
            Container::Decimal(value) => value.to_string(),
            Container::Str(ref value) => {
                format!("\"{}\"", value.to_string())
            }
            Container::Null => "null".to_string(),
        }
    }

    pub fn as_string(self) -> Option<String> {
        match self {
            Container::Str(value) => Some(value),
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
            Container::Null => true,
            _ => false,
        }
    }

    /// Returns the length of an object
    pub fn len(&self) -> usize {
        match *self {
            Container::Array(ref value) => value.len(),
            Container::Object(ref value) => value.len(),
            Container::Set(ref value) => value.len(),
            Container::Str(ref value) => value.len(),
            _ => 1usize,
        }
    }
}

impl Index<usize> for Container {
    type Output = Container;
    // Returns the value given the index (usize).
    fn index(&self, idx: usize) -> &Self::Output {
        match *self {
            Container::Array(ref value) => {
                if value.len() > idx {
                    &value.get(idx).unwrap()
                } else {
                    &Container::Null
                }
            }
            _ => &Container::Null,
        }
    }
}

impl Index<String> for Container {
    type Output = Container;
    // Returns the value given the string index
    fn index(&self, idx: String) -> &Self::Output {
        match *self {
            Container::Object(ref value) => {
                let ret = value.get(&idx);
                if ret != None {
                    ret.unwrap()
                } else {
                    &Container::Null
                }
            }
            _ => &Container::Null,
        }
    }
}

impl Index<&str> for Container {
    type Output = Container;
    // Returns the value given the string index
    fn index(&self, idx: &str) -> &Self::Output {
        match *self {
            Container::Object(ref value) => {
                let ret = value.get(&idx.to_string());
                if ret != None {
                    ret.unwrap()
                } else {
                    &Container::Null
                }
            }
            _ => &Container::Null,
        }
    }
}

impl IndexMut<usize> for Container {
    // Returns the value given the index (usize).
    fn index_mut(&mut self, index: usize) -> &mut Self {
        match *self {
            Self::Array(ref mut value) => {
                if value.len() > index {
                    return &mut value[index];
                } else {
                    let len = value.len();
                    value.push(Container::Null);
                    &mut value[len - 1]
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
            Container::Object(ref mut value) => {
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
                let exists = value.contains_key(&idx.to_string());
                if !exists {
                    value.insert(idx.to_string(), Container::Null);
                }
                value.get_mut(&idx.to_string()).unwrap()
            }
            _ => {
                // Log: Change into arrayCRED warning
                *self = Self::new_object();
                self.insert(idx.to_string(), Self::Null);
                &mut self[idx]
            }
        }
    }
}
