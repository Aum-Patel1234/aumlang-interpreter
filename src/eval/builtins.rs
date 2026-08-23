use crate::object::obj::{Builtin, DoubleObject, ErrorObject, Object, ObjectTrait};

fn builtin_len(args: &[Object]) -> Object {
    if args.len() != 1 {
        return Object::Error(ErrorObject::new(format!(
            "Expected one string argument in builtin len() func got {} args",
            args.len(),
        )));
    }
    let s = match &args[0] {
        Object::StringObj(string_object) => string_object,
        o => {
            return Object::Error(ErrorObject::new(format!(
                "Expected string got {}",
                o.object_type()
            )));
        }
    };

    Object::Double(DoubleObject {
        value: s.value.len() as f64,
    })
}
pub static BUILTINS: &[(&str, Builtin)] = &[("len", Builtin { func: builtin_len })];
