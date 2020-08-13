use iomath::vectors::*;

#[test]
fn test_vector_add_scalar() {
    let vector = Vector2::new(1.0, -2.0);
    let result = vector + 3.0;

    assert_eq!(result, Vector2::new(4.0, 1.0));
}

#[test]
fn test_vector_add_vector() {
    let first_add = Vector2::new(5.0, 6.0);
    let second_add = Vector2::new(-1.0, 4.5);
    let result = first_add + second_add;

    assert_eq!(result, Vector2::new(4.0, 10.5));
}