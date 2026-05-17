use std::{
    cell::RefCell,
    collections::{HashMap, hash_map},
    rc::Rc,
};

use crate::object::obj::Object;

// NOTE: another way do do below thing
// struct Environment {
//     parent: Option<usize>,
//     store: HashMap<String, Object>,
// }
// Vec<Environment>
#[derive(Default, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Rc<RefCell<Environment>>>,
}
impl Environment {
    pub fn set_outer(&mut self, env: Rc<RefCell<Environment>>) {
        self.outer = Some(env);
    }
    pub fn set(&mut self, key: String, val: Object) {
        self.store.insert(key, val);
    }
    pub fn get(&self, key: &str) -> Option<Object> {
        let obj = self.store.get(key);
        if let Some(o) = obj {
            return Some(o.clone());
        }
        match &self.outer {
            Some(env) => Some(env.borrow().get(key)?.clone()), // recursively till above
            None => None,
        }
    }
    // pub fn get_ref(&self, key: &str) -> Option<&Object> {
    //     self.store.get(key)
    // }
    pub fn iter(&self) -> hash_map::Iter<'_, String, Object> {
        self.store.iter()
    }
    pub fn iter_mut(&mut self) -> hash_map::IterMut<'_, String, Object> {
        self.store.iter_mut()
    }
}
