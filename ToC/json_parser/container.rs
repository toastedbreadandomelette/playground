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
    Number(i128),
    /// A 16 byte unsigned integer
    Unsigned(u128),
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
            Container::Null => Container::Null,
            // Object, Array and Set containers
            Container::Array(ref array) => {
                let mut clone_array: Container = Container::new_array();
                for element in array {
                    clone_array.push(element.clone());
                }
                clone_array
            }
            Container::Object(ref object) => {
                let mut clone_object: Container = Container::new_object();
                for (key, value) in object {
                    clone_object.insert(key.to_string(), value.clone());
                }
                clone_object
            }
            Container::Set(ref set) => {
                let mut clone_set: Container = Container::new_set();
                for value in set {
                    clone_set.push(value.clone());
                }
                clone_set
            }
        }
    }
}

/// Implementing hash for the Set type in Container(::Set)
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
            (Container::Number(ref this_value), Container::Number(ref other_value)) => {
                this_value == other_value
            }
            (Container::Unsigned(ref this_value), Container::Unsigned(ref other_value)) => {
                this_value == other_value
            }
            (Container::Decimal(ref this_value), Container::Decimal(ref other_value)) => {
                this_value == other_value
            }
            (Container::Boolean(ref this_value), Container::Boolean(ref other_value)) => {
                this_value == other_value
            }
            (Container::Str(ref this_value), Container::Str(ref other_value)) => {
                this_value == other_value
            }
            (Container::Array(ref this_value), Container::Array(ref other_value)) => {
                if this_value.len() != other_value.len() {
                    false
                } else {
                    for i in 0..this_value.len() {
                        if this_value[i] != other_value[i] {
                            return false;
                        }
                    }
                    true
                }
            }
            (Container::Set(ref this_value), Container::Set(ref other_value)) => {
                if this_value.len() != other_value.len() {
                    false
                } else {
                    for item in this_value {
                        if other_value.get(item) == None {
                            return false;
                        }
                    }
                    true
                }
            }
            (Container::Object(ref this_value), Container::Object(ref other_value)) => {
                if this_value.len() != other_value.len() {
                    return false;
                }
                for (key, item) in this_value {
                    if other_value.get(key) != Some(item) {
                        return false;
                    }
                }
                true
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
    pub fn new_object() -> Container {
        Container::Object(HashMap::new())
    }

    // Returns New Array Object
    #[inline(always)]
    pub fn new_array() -> Container {
        Container::Array(Vec::new())
    }

    // Returns new set
    #[inline(always)]
    pub fn new_set() -> Container {
        Container::Set(HashSet::new())
    }
    // Array: Push an item into array or an element into set:
    // Returns false if not inserted
    // Permissible for array type only
    pub fn push(&mut self, val: Container) -> bool {
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
    pub fn insert(&mut self, key: String, val: Container) -> bool {
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
                if indent == false {
                    let mut object_str = "[".to_string();
                    for (index, elem) in value.iter().enumerate() {
                        object_str.push_str(&elem.dump_object(indent, indent_size, white_space));
                        if index != value.len() - 1 {
                            object_str.push(',');
                        }
                    }
                    object_str.push(']');
                    object_str
                } else {
                    if value.len() == 0 {
                        return "[]".to_string();
                    }
                    let mut space = white_space.to_string();
                    for _i in 0..indent_size {
                        space.push(' ');
                    }

                    let mut object_str = "[\n".to_string();
                    for (index, elem) in value.iter().enumerate() {
                        object_str.push_str(&space.to_string());
                        object_str.push_str(&elem.dump_object(indent, indent_size, &space));
                        if index != value.len() - 1 {
                            object_str.push_str(",\n");
                        }
                    }

                    object_str.push_str(&format!("\n{}]", &white_space.to_string()));
                    object_str
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

                        for _i in 0..indent_size {
                            space.push(' ');
                        }
                        for (key, val) in value {
                            object_str.push_str(&space);
                            object_str.push_str(&format!("\"{}\": {}", key, val.dump_object(indent, indent_size, &space)));
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
                    let (mut object_str, mut index) = ("(".to_string(), 0);

                    for val in value {
                        object_str.push_str(&val.dump_object(indent, indent_size, white_space));
                        index += 1;
                        if index != value.len() {
                            object_str.push(',');
                        }
                    }
                    object_str.push(')');
                    object_str
                } else {
                    if value.len() == 0 {
                        "()".to_string()
                    } else {
                        let (mut object_str, mut space, mut index) =
                            ("(\n".to_string(), white_space.to_string(), 0);
                        for _i in 0..indent_size {
                            space.push(' ');
                        }
                        for val in value {
                            object_str.push_str(&space);
                            object_str.push_str(&val.dump_object(indent, indent_size, &space));
                            index += 1;
                            if index != value.len() {
                                object_str.push_str(",\n");
                            }
                        }
                        object_str.push_str(&format!("\n{})", white_space));
                        object_str
                    }
                }
            }
            Container::Number(ref value) => value.to_string(),
            Container::Unsigned(ref value) => value.to_string(),
            Container::Boolean(ref value) => value.to_string(),
            Container::Decimal(ref value) => value.to_string(),
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
            _ => 0usize,
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
    fn index_mut(&mut self, idx: usize) -> &mut Container {
        match *self {
            Container::Array(ref mut value) => {
                if value.len() > idx {
                    return &mut value[idx];
                } else {
                    let len = value.len();
                    value.push(Container::Null);
                    &mut value[len - 1]
                }
            }
            _ => {
                // Log: Change into array warning
                *self = Container::new_array();
                self.push(Container::Null);
                &mut self[0]
            }
        }
    }
}

impl IndexMut<String> for Container {
    // Returns the value given the index (usize).
    fn index_mut(&mut self, idx: String) -> &mut Container {
        match *self {
            Container::Object(ref mut value) => {
                let exists = value.contains_key(&idx);
                if !exists {
                    value.insert(idx.to_string(), Container::Null);
                }
                value.get_mut(&idx).unwrap()
            }
            _ => {
                // Log: Change into array warning
                *self = Container::new_object();
                self.insert(idx.clone(), Container::Null);
                &mut self[idx]
            }
        }
    }
}

impl IndexMut<&str> for Container {
    // Returns the value given the index (usize).
    fn index_mut<'a>(&mut self, idx: &'a str) -> &mut Container {
        match *self {
            Container::Object(ref mut value) => {
                let exists = value.contains_key(&idx.to_string());
                if !exists {
                    value.insert(idx.to_string(), Container::Null);
                }
                value.get_mut(&idx.to_string()).unwrap()
            }
            _ => {
                // Log: Change into arrayCRED warning
                *self = Container::new_object();
                self.insert(idx.to_string(), Container::Null);
                &mut self[idx]
            }
        }
    }
}
