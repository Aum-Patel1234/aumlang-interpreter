use std::collections::HashMap;

use crate::object::obj::Object;

// impl Default for Environment {
//     fn default() -> Self {
//         Self {
//             store: Default::default(),
//         }
//     }
// }
#[derive(Default)]
pub struct Environment {
    store: HashMap<String, Object>,
}
impl Environment {
    pub fn set(&mut self, key: String, val: Object) {
        self.store.insert(key, val);
    }
    pub fn get(&mut self, key: &str) -> Option<Object> {
        let obj = self.store.get(key)?;
        Some(obj.clone())
    }
    pub fn get_ref(&mut self, key: &str) -> Option<&Object> {
        self.store.get(key)
    }
}
