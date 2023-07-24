use mixvec::{MixVec, MixVecElement};

#[test]
fn one_element() {
    let mut v = MixVec::new();
    v.push(1);
    assert_eq!(v.len(), 1);
    assert_eq!(v[0], MixVecElement::Integer(1))
}

#[test]
fn multiple_elements() {
    let mut v = MixVec::new();
    v.push(1);
    v.push(200);
    v.push(300);
    assert_eq!(v.len(), 3);
    assert_eq!(v[0], MixVecElement::Integer(1));
    assert_eq!(v[1], MixVecElement::Integer(200));
    assert_eq!(v[2], MixVecElement::Integer(300));
}

#[test]
fn different_type_elements() {
    let mut v = MixVec::new();
    v.push(1);
    v.push("Hello world");
    v.push(3.14);
    v.push(false);
    v.push('A');
    // v.push(MyCustomStruct::new());
    assert_eq!(v.len(), 6);
    assert_eq!(v[0], MixVecElement::Integer(1));
    assert_eq!(v[1], MixVecElement::String("Hello world".to_string()));
    assert_eq!(v[2], MixVecElement::Float(3.14));
    assert_eq!(v[3], MixVecElement::Boolean(false));
    assert_eq!(v[4], MixVecElement::Character('A'));
}