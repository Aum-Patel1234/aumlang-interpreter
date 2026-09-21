use crate::{
    consts::{APPEND_FN_NAME, LEN_FN_NAME, LOWER_FN_NAME, TYPE_FN_NAME, UPPER_FN_NAME},
    object::obj::{
        ARRAY_OBJ, BOOLEAN_OBJ, BUILTIN_OBJ, Builtin, DOUBLE_OBJ, DoubleObject, ERROR_OBJ,
        ErrorObject, FUNCTION_OBJ, NULL_OBJ, Object, ObjectTrait, RETURN_VALUE_OBJ, STRING_OBJ,
        StringObject,
    },
};

#[inline]
pub fn check_single_str_arg<'a>(
    args: &'a [Object],
    fn_name: &'a str,
) -> Result<&'a StringObject, ErrorObject> {
    if args.len() != 1 {
        return Err(ErrorObject::new(format!(
            "Expected one string argument in builtin {}() func got {} args",
            fn_name,
            args.len(),
        )));
    }
    match &args[0] {
        Object::StringObj(string_object) => Ok(string_object),
        o => Err(ErrorObject::new(format!(
            "Expected string got {}",
            o.object_type()
        ))),
    }
}

fn builtin_len(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(ErrorObject::new(format!(
            "Expected one string argument in builtin {}() func got {} args",
            LEN_FN_NAME,
            args.len(),
        )));
    }
    match &args[0] {
        Object::StringObj(so) => Object::Double(DoubleObject {
            value: so.value.len() as f64,
        }),
        Object::Array(array_object) => Object::Double(DoubleObject {
            value: array_object.len() as f64,
        }),
        o => Object::Error(ErrorObject::new(format!(
            "Expected string got {}",
            o.object_type()
        ))),
    }
}

fn builtin_type(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(ErrorObject::new(format!(
            "Expected one string argument in builtin len() func got {} args",
            args.len(),
        )));
    }
    let obj_type = match args[0] {
        Object::Double(_) => DOUBLE_OBJ.to_lowercase(),
        Object::StringObj(_) => STRING_OBJ.to_lowercase(),
        Object::Boolean(_) => BOOLEAN_OBJ.to_lowercase(),
        Object::Null(_) => NULL_OBJ.to_lowercase(),
        Object::RetrunValue(_) => RETURN_VALUE_OBJ.to_lowercase(),
        Object::Error(_) => ERROR_OBJ.to_lowercase(),
        Object::Function(_) => FUNCTION_OBJ.to_lowercase(),
        Object::Builtin(_) => BUILTIN_OBJ.to_lowercase(),
        Object::Array(_) => ARRAY_OBJ.to_lowercase(),
    };

    Object::StringObj(StringObject { value: obj_type })
}

fn builtin_lower(args: Vec<Object>) -> Object {
    let s = match check_single_str_arg(&args, LOWER_FN_NAME) {
        Ok(s) => s.value.as_str(),
        Err(o) => return Object::Error(o),
    };

    let mut lower_str: Vec<u8> = Vec::with_capacity(s.len());
    for ch in s.chars() {
        let ascii = ch as u8;
        if !(65..=90).contains(&ascii) {
            lower_str.push(ascii);
        } else {
            // 65('A') + 32 = 97 ('a')
            lower_str.push(ascii + 32u8);
        }
    }

    Object::StringObj(StringObject {
        value: String::from_utf8_lossy(&lower_str).into_owned(),
    })
}
fn builtin_upper(args: Vec<Object>) -> Object {
    let s = match check_single_str_arg(&args, UPPER_FN_NAME) {
        Ok(s) => s.value.as_str(),
        Err(o) => return Object::Error(o),
    };

    let mut lower_str: Vec<u8> = Vec::with_capacity(s.len());
    for ch in s.chars() {
        let ascii = ch as u8;
        if !(97..=122).contains(&ascii) {
            lower_str.push(ascii);
        } else {
            // 97('a') - 32 = 65('A')
            lower_str.push(ascii - 32u8);
        }
    }

    Object::StringObj(StringObject {
        value: String::from_utf8_lossy(&lower_str).into_owned(),
    })
}
fn builtin_append(mut args: Vec<Object>) -> Object {
    if args.len() < 2 {
        return Object::Error(ErrorObject {
            msg: format!("builtin function {}() takes 2 or more args", APPEND_FN_NAME),
        });
    }
    let array = match args.remove(0) {
        Object::Array(array_object) => array_object,
        _ => {
            return Object::Error(ErrorObject {
                msg: format!(
                    "builtin function {}() expects first argument to be an Array Object",
                    APPEND_FN_NAME
                ),
            });
        }
    };
    {
        let mut arr = array.arr.borrow_mut();
        // NOTE: small bug here if ther is let say append(arr, 2,append)
        // as append in builtin func it will throw err but will add 2 as it was previously added
        for obj in args {
            match &obj {
                Object::Double(_) => {}
                Object::StringObj(_) => {}
                Object::Boolean(_) => {}
                Object::Null(_) => {}
                Object::Function(_) => {}
                Object::Array(_) => {}
                o => {
                    //not allowed remaining tyeps
                    return Object::Error(ErrorObject {
                        msg: format!(
                            "builtin function {}() does not allow value of type {} to be added to the array.",
                            APPEND_FN_NAME, o
                        ),
                    });
                }
            };
            arr.push(obj);
        }
    }
    Object::Array(array)
}

// fn builtin_pop(mut args: Vec<Object>) -> Object {
//     todo!()
// }

pub static BUILTINS: &[(&str, Builtin)] = &[
    (LEN_FN_NAME, Builtin { func: builtin_len }),
    (TYPE_FN_NAME, Builtin { func: builtin_type }),
    (
        LOWER_FN_NAME,
        Builtin {
            func: builtin_lower,
        },
    ),
    (
        UPPER_FN_NAME,
        Builtin {
            func: builtin_upper,
        },
    ),
    (
        APPEND_FN_NAME,
        Builtin {
            func: builtin_append,
        },
    ),
];

// Phase 2
// ───────
// trim()
// contains()
// substring()
// replace()
//
// Phase 3
// ───────
// int()
// float()
// str()
// bool()
//
// Phase 4
// ───────
// split()
// join()
// startsWith()
// endsWith()
//
// Phase 5
// ───────
// arrays/lists
//     sort()
//     reverse()
//
// Phase 6
// ───────
// math
//     abs()
//     min()
//     max()
//     pow()
//     sqrt()
//
// Phase 7
// ───────
// files
//     readFile()
//     writeFile()
