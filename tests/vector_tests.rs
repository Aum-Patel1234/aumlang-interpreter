use aumlang::eval::vector::Vector;

#[test]
fn new_vector_is_empty() {
    let vector: Vector<i32> = Vector::new();
    assert_eq!(vector.get_elements_as_vec(), Vec::<i32>::new());
}

#[test]
fn push_one_element() {
    let mut vector = Vector::new();
    vector.push(10);
    assert_eq!(vector.get_elements_as_vec(), vec![10]);
}

#[test]
fn push_multiple_elements() {
    let mut vector = Vector::new();

    vector.push(10);
    vector.push(20);
    vector.push(30);

    assert_eq!(vector.get_elements_as_vec(), vec![10, 20, 30]);
}

#[test]
fn push_beyond_initial_capacity() {
    let mut vector = Vector::new();

    for i in 0..10 {
        vector.push(i);
    }

    assert_eq!(
        vector.get_elements_as_vec(),
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    );
}

#[test]
fn pop_removes_last_element() {
    let mut vector = Vector::new();

    vector.push(10);
    vector.push(20);
    vector.push(30);

    vector.pop();

    assert_eq!(vector.get_elements_as_vec(), vec![10, 20]);
}

#[test]
fn pop_multiple_elements() {
    let mut vector = Vector::new();

    vector.push(10);
    vector.push(20);
    vector.push(30);

    vector.pop();
    vector.pop();

    assert_eq!(vector.get_elements_as_vec(), vec![10]);
}

#[test]
fn push_after_pop() {
    let mut vector = Vector::new();

    vector.push(10);
    vector.push(20);

    vector.pop();
    vector.push(30);

    assert_eq!(vector.get_elements_as_vec(), vec![10, 30]);
}

#[test]
fn vector_works_with_strings() {
    let mut vector = Vector::new();

    vector.push(String::from("hello"));
    vector.push(String::from("world"));

    assert_eq!(
        vector.get_elements_as_vec(),
        vec![String::from("hello"), String::from("world")]
    );
}
#[test]
fn pop_returns_last_element() {
    let mut vector = Vector::new();

    vector.push(10);
    vector.push(20);

    assert_eq!(vector.pop(), Some(20));
    assert_eq!(vector.pop(), Some(10));
    assert_eq!(vector.pop(), None);
}
#[test]
fn get_existing_element() {
    let mut vector = Vector::new();

    vector.push(10);
    vector.push(20);
    vector.push(30);

    assert_eq!(vector.get(0), Some(10));
    assert_eq!(vector.get(1), Some(20));
    assert_eq!(vector.get(2), Some(30));
}

#[test]
fn get_out_of_bounds() {
    let mut vector = Vector::new();

    vector.push(10);
    vector.push(20);

    assert_eq!(vector.get(2), None);
    assert_eq!(vector.get(100), None);
}

#[test]
fn get_does_not_remove_element() {
    let mut vector = Vector::new();

    vector.push(10);
    vector.push(20);

    assert_eq!(vector.get(0), Some(10));
    assert_eq!(vector.get(0), Some(10));

    assert_eq!(vector.get_elements_as_vec(), vec![10, 20]);
}
