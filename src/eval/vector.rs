use std::{
    alloc::{Layout, alloc, dealloc},
    ptr,
};

const INITIAL_CAPACITY: usize = 4;

#[derive(Clone)]
pub struct Vector<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vector<T> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        // i am just learning so if unwrap crashes its what it is
        // https://doc.rust-lang.org/std/alloc/struct.Layout.html
        let layout = Layout::array::<T>(INITIAL_CAPACITY).unwrap();
        let data = unsafe { alloc(layout) as *mut T }; // https://doc.rust-lang.org/std/alloc/fn.alloc.html
        Vector {
            data,
            len: 0,
            capacity: INITIAL_CAPACITY,
        }
    }
    pub fn new_with_capacity(capacity: usize) -> Self {
        let layout = Layout::array::<T>(capacity).unwrap();
        let data = unsafe { alloc(layout) as *mut T }; // https://doc.rust-lang.org/std/alloc/fn.alloc.html
        Vector {
            data,
            len: 0,
            capacity,
        }
    }
    pub fn get_elements_as_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut vec: Vec<T> = Vec::with_capacity(self.capacity);
        for i in 0..self.len {
            let elem = unsafe { &*(self.data.add(i)) };
            vec.push(elem.clone());
        }
        vec
    }

    pub fn push(&mut self, val: T) {
        if self.len == self.capacity {
            let new_capacity = self.capacity << 1;
            let new_layout = Layout::array::<T>(new_capacity).unwrap();
            let new_data = unsafe { alloc(new_layout) as *mut T };
            // copy elements
            for i in 0..self.len {
                unsafe {
                    ptr::write(new_data.add(i), ptr::read(self.data.add(i)));
                }
            }
            let old_layout = Layout::array::<T>(self.capacity).unwrap();
            unsafe {
                dealloc(self.data as *mut u8, old_layout);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }

        unsafe {
            ptr::write(self.data.add(self.len), val);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        Some(unsafe { std::ptr::read(self.data.add(self.len)) })
    }
    pub fn get(&self, idx: usize) -> Option<T> {
        if idx >= self.len {
            return None;
        }
        Some(unsafe { std::ptr::read(self.data.add(idx)) })
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

// impl<K: Display> Display for Vector<K> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }
