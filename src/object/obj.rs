use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    environment::Environment,
    parser::{
        Node,
        ast::{BlockStatement, Identifier},
    },
};

pub type ObjectType = String;

pub const DOUBLE_OBJ: &str = "DOUBLE";
pub const STRING_OBJ: &str = "STRING";
pub const BOOLEAN_OBJ: &str = "BOOLEAN";
pub const NULL_OBJ: &str = "NULL";
pub const RETURN_VALUE_OBJ: &str = "RETURN_VALUE";
pub const ERROR_OBJ: &str = "ERROR";
pub const FUNCTION_OBJ: &str = "FUNCTION";
pub const BUILTIN_OBJ: &str = "BUILTIN";
pub const TRUE: BooleanObject = BooleanObject { value: true };
pub const FALSE: BooleanObject = BooleanObject { value: false };
pub const NULL: NullObject = NullObject {};

pub trait ObjectTrait {
    fn object_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
}

#[derive(Clone)]
pub enum Object {
    Double(DoubleObject),
    StringObj(StringObject),
    Boolean(&'static BooleanObject),
    Null(&'static NullObject),
    RetrunValue(ReturnObject),
    Error(ErrorObject),
    Function(FunctionObject),
    Builtin(&'static Builtin),
}
impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Object::Double(double_object) => double_object.inspect(),
            Object::Boolean(boolean_object) => boolean_object.inspect(),
            Object::Null(null_object) => null_object.inspect(),
            Object::RetrunValue(return_object) => return_object.inspect(),
            Object::Error(error) => error.inspect(),
            Object::Function(function_object) => function_object.inspect(),
            Object::StringObj(string_object) => string_object.inspect(),
            Object::Builtin(builtin) => builtin.inspect(),
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
            Object::RetrunValue(return_object) => return_object.object_type(),
            Object::Error(error) => error.object_type(),
            Object::Function(function_object) => function_object.object_type(),
            Object::StringObj(string_object) => string_object.object_type(),
            Object::Builtin(builtin) => builtin.object_type(),
        }
    }

    fn inspect(&self) -> String {
        match self {
            Object::Double(d) => d.inspect(),
            Object::Boolean(boolean_object) => boolean_object.inspect(),
            Object::Null(null_object) => null_object.inspect(),
            Object::RetrunValue(return_object) => return_object.inspect(),
            Object::Error(error) => error.inspect(),
            Object::Function(function_object) => function_object.inspect(),
            Object::StringObj(string_object) => string_object.inspect(),
            Object::Builtin(builtin) => builtin.inspect(),
        }
    }
}

// double
#[derive(Clone)]
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

// strings
#[derive(Clone)]
pub struct StringObject {
    pub value: String,
}

impl ObjectTrait for StringObject {
    fn object_type(&self) -> ObjectType {
        STRING_OBJ.to_string()
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

// boolean
#[derive(PartialEq)]
pub struct BooleanObject {
    pub value: bool,
}
impl BooleanObject {
    pub fn get(value: bool) -> &'static BooleanObject {
        if value { &TRUE } else { &FALSE }
    }
    pub fn get_from_num(value: f64) -> &'static BooleanObject {
        if value == 0f64 { &FALSE } else { &TRUE }
    }
    pub fn not_get(&self) -> &'static BooleanObject {
        if *self == TRUE { &FALSE } else { &TRUE }
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

// return object
#[derive(Clone)]
pub struct ReturnObject {
    pub value: Box<Object>,
}
impl ReturnObject {
    pub fn new(retrun_obj: Object) -> ReturnObject {
        ReturnObject {
            value: Box::new(retrun_obj),
        }
    }
}
impl ObjectTrait for ReturnObject {
    fn object_type(&self) -> ObjectType {
        RETURN_VALUE_OBJ.to_string()
    }

    fn inspect(&self) -> String {
        self.value.inspect()
    }
}

// error
// NOTE: In a production-ready interpreter we’d want to attach a stack trace to such error
// objects, add the line and column numbers of its origin and provide more than just a message.
// Line and column numbers are attached to the tokens by the lexer
#[derive(Clone)]
pub struct ErrorObject {
    pub msg: String,
}
impl ErrorObject {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}
impl ObjectTrait for ErrorObject {
    fn object_type(&self) -> ObjectType {
        ERROR_OBJ.to_string()
    }

    fn inspect(&self) -> String {
        format!("ERROR: {}", self.msg)
    }
}

// function obj
#[derive(Clone)]
pub struct FunctionObject {
    pub args: Vec<Identifier>,
    pub body: BlockStatement,
    pub env: Rc<RefCell<Environment>>,
}
impl FunctionObject {
    pub fn new(args: Vec<Identifier>, body: BlockStatement, env: Rc<RefCell<Environment>>) -> Self {
        FunctionObject { args, body, env }
    }
}
impl ObjectTrait for FunctionObject {
    fn object_type(&self) -> ObjectType {
        FUNCTION_OBJ.to_string()
    }

    fn inspect(&self) -> String {
        let mut out = String::new();
        out.push_str("fn(");
        out.push_str(
            &self
                .args
                .iter()
                .map(|i| i.value.as_str())
                .collect::<Vec<_>>()
                .join(", "),
        );
        out.push_str(") {\n");
        out.push_str(&self.body.string());
        out.push_str("\n}");
        out
    }
}

pub type BuiltinFunction = fn(args: &[Object]) -> Object;

#[derive(Clone)]
pub struct Builtin {
    pub func: BuiltinFunction,
}
impl ObjectTrait for Builtin {
    fn object_type(&self) -> ObjectType {
        FUNCTION_OBJ.to_string()
    }

    fn inspect(&self) -> String {
        "builtin function".to_string()
    }
}
