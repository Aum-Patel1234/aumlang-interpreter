use std::fmt::Display;

pub type ObjectType = String;

pub const DOUBLE_OBJ: &str = "DOUBLE";
pub const BOOLEAN_OBJ: &str = "BOOLEAN";
pub const NULL_OBJ: &str = "NULL";
pub const TRUE: BooleanObject = BooleanObject { value: true };
pub const FALSE: BooleanObject = BooleanObject { value: false };
pub const NULL: NullObject = NullObject {};

pub trait ObjectTrait {
    fn object_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
}
pub enum Object {
    Double(DoubleObject),
    Boolean(&'static BooleanObject),
    Null(&'static NullObject),
}
impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Object::Double(double_object) => double_object.inspect(),
            Object::Boolean(boolean_object) => boolean_object.inspect(),
            Object::Null(null_object) => null_object.inspect(),
        };
        write!(f, "{}", s)
    }
}
impl ObjectTrait for Object {
    fn object_type(&self) -> ObjectType {
        match self {
            Object::Double(d) => d.object_type(),
            Object::Boolean(boolean_object) => boolean_object.object_type(),
            Object::Null(null_object) => null_object.object_type(),
        }
    }

    fn inspect(&self) -> String {
        match self {
            Object::Double(d) => d.inspect(),
            Object::Boolean(boolean_object) => boolean_object.inspect(),
            Object::Null(null_object) => null_object.inspect(),
        }
    }
}

// double
pub struct DoubleObject {
    pub value: f64,
}

impl ObjectTrait for DoubleObject {
    fn object_type(&self) -> ObjectType {
        DOUBLE_OBJ.to_string()
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

// boolean
pub struct BooleanObject {
    pub value: bool,
}
impl BooleanObject {
    pub fn get(value: bool) -> &'static BooleanObject {
        if value { &TRUE } else { &FALSE }
    }
}
impl ObjectTrait for BooleanObject {
    fn object_type(&self) -> ObjectType {
        BOOLEAN_OBJ.to_string()
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

// null
pub struct NullObject {}
impl NullObject {
    pub fn get() -> &'static NullObject {
        &NULL
    }
}
impl ObjectTrait for NullObject {
    fn object_type(&self) -> ObjectType {
        NULL_OBJ.to_string()
    }

    fn inspect(&self) -> String {
        "null".to_string()
    }
}
