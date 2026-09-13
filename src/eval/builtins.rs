use crate::{
    consts::{LEN_FN_NAME, LOWER_FN_NAME, TYPE_FN_NAME, UPPER_FN_NAME},
    object::obj::{
        BOOLEAN_OBJ, BUILTIN_OBJ, Builtin, DOUBLE_OBJ, DoubleObject, ERROR_OBJ, ErrorObject,
        FUNCTION_OBJ, NULL_OBJ, Object, ObjectTrait, RETURN_VALUE_OBJ, STRING_OBJ, StringObject,
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

fn builtin_len(args: &[Object]) -> Object {
    let s = match check_single_str_arg(args, "len") {
        Ok(s) => s,
        Err(o) => return Object::Error(o),
    };

    Object::Double(DoubleObject {
        value: s.value.len() as f64,
    })
}

fn builtin_type(args: &[Object]) -> Object {
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
    };

    Object::StringObj(StringObject { value: obj_type })
}

fn builtin_lower(args: &[Object]) -> Object {
    let s = match check_single_str_arg(args, LOWER_FN_NAME) {
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
fn builtin_upper(args: &[Object]) -> Object {
    let s = match check_single_str_arg(args, UPPER_FN_NAME) {
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
//     append()
//     pop()
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
