use iomath::vectors::{ Vector3, UVector3, Vector4, Vector2 };
use iomath::quaternions::Quaternion;

#[test]
fn vector_3_empty() {
    let vector = Vector3::empty();

    assert_eq!(vector, Vector3 { x: 0.0, y: 0.0, z: 0.0 });
}

#[test]
fn vector_3_new() {
    let vector = Vector3::new(-0.5, 1.333, 3.7);

    assert_eq!(vector, Vector3 { x: -0.5, y: 1.333, z: 3.7 });
}

#[test]
fn vector_3_copy() {
    let vector_from = Vector3::new(0.0, 1.1, 2.2);
    let vector_to = vector_from;

    assert_eq!(vector_from, Vector3::new(0.0, 1.1, 2.2));
    assert_eq!(vector_to, Vector3::new(0.0, 1.1, 2.2));
}

#[test]
fn vector_3_from_scalar() {
    let vector = Vector3::from_scalar(1.7);

    assert_eq!(vector, Vector3 { x: 1.7, y: 1.7, z: 1.7 });
}

#[test]
fn vector_3_from_value_vector() {
    let vector = Vector3::from_value_vector(4.5, Vector2::new(9.8, 3.8));

    assert_eq!(vector, Vector3::new(4.5, 9.8, 3.8));
}

#[test]
fn vector_3_from_vector_value() {
    let vector = Vector3::from_vector_value(Vector2::new(9.8, 3.8), 4.5);

    assert_eq!(vector, Vector3::new(9.8, 3.8, 4.5));
}

#[test]
fn vector_3_from_vector_2() {
    let vector = Vector3::from(Vector2::new(5.5, -3.3));

    assert_eq!(vector, Vector3::new(5.5, -3.3, 0.0));
}

#[test]
fn vector_3_from_vector_4() {
    let vector = Vector3::from(Vector4::new(0.5, 4.3, 1.1, 5.6));

    assert_eq!(vector, Vector3::new(0.5, 4.3, 1.1));
}

#[test]
fn vector_3_from_quaternion() {
    let vector = Vector3::from(Quaternion::identity());

    assert_eq!(vector, Vector3::new(0.0, 0.0, 0.0));
}

#[test]
fn vector_3_index() {
    let vector = Vector3::new(1.3, 2.7, 0.4);

    assert_eq!(vector, Vector3::new(vector[0], vector[1], vector[2]));
}

#[test]
fn vector_3_index_out_of_bounds() {
    let vector = Vector3::new(7.3, 2.4, -3.8);
    
    assert_eq!(vector, Vector3::new(vector[0], vector[1], vector[256]))
}

#[test]
fn vector_3_index_mut() {
    let mut vector = Vector3::new(5.6, 3.8, 27.0);
    vector[0] *= 0.5;
    vector[1] *= 2.0;
    vector[2] /= 3.0;

    assert_eq!(vector, Vector3::new(2.8, 7.6, 9.0));
}

#[test]
fn vector_3_index_mut_out_of_bounds() {
    let mut vector = Vector3::new(4.5, -7.6, 7.2);
    vector[1024] = -1.0;

    assert_eq!(vector, Vector3::new(4.5, -7.6, -1.0));
}

#[test]
fn vector_3_add_scalar() {
    let vector = Vector3::new(1.0, -2.0, 3.0);
    let result = vector + 3.0;

    assert_eq!(result, Vector3::new(4.0, 1.0, 6.0));
}

#[test]
fn vector_3_add_vector() {
    let first_add = Vector3::new(5.0, 6.0, 4.0);
    let second_add = Vector3::new(-1.0, 4.5, 3.2);
    let result = first_add + second_add;
    
    assert_eq!(result, Vector3::new(4.0, 10.5, 7.2));
}

#[test]
fn vector_3_add_assign_scalar() {
    let mut vector = Vector3::new(2.5, -1.0, 9.9);
    vector += 0.5;

    assert_eq!(vector, Vector3::new(3.0, -0.5, 10.4));
}

#[test]
fn vector_3_add_assign_vector() {
    let mut vector = Vector3::new(2.5, -1.0, 3.2);
    vector += Vector3::new(0.5, 1.0, -3.0);

    assert_eq!(vector, Vector3::new(3.0, 0.0, 0.20000005));
}

#[test]
fn vector_3_sub_scalar() {
    let vector = Vector3::new(1.0, -2.0, 3.0);
    let result = vector - 3.0;

    assert_eq!(result, Vector3::new(-2.0, -5.0, 0.0));
}

#[test]
fn vector_3_sub_vector() {
    let minuend = Vector3::new(5.0, 6.0, 4.0);
    let subtrahend = Vector3::new(-1.0, 4.5, 10.0);
    let result = minuend - subtrahend;
    
    assert_eq!(result, Vector3::new(6.0, 1.5, -6.0));
}

#[test]
fn vector_3_sub_assign_scalar() {
    let mut vector = Vector3::new(2.5, -1.0, 4.4);
    vector -= 0.5;

    assert_eq!(vector, Vector3::new(2.0, -1.5, 3.9));
}

#[test]
fn vector_3_sub_assign_vector() {
    let mut vector = Vector3::new(2.5, -1.0, 2.2);
    vector -= Vector3::new(0.5, 1.0, 0.7);

    assert_eq!(vector, Vector3::new(2.0, -2.0, 1.5));
}

#[test]
fn vector_3_mul_scalar() {
    let vector = Vector3::new(1.0, -2.0, 3.0);
    let result = vector * 3.0;

    assert_eq!(result, Vector3::new(3.0, -6.0, 9.0));
}

#[test]
fn vector_3_mul_vector() {
    let first_mul = Vector3::new(5.0, 6.0, 4.0);
    let second_mul = Vector3::new(-1.0, 4.5, 2.5);
    let result = first_mul * second_mul;
    
    assert_eq!(result, Vector3::new(-5.0, 27.0, 10.0));
}

#[test]
fn vector_3_mul_assign_scalar() {
    let mut vector = Vector3::new(2.5, -1.0, 1.1);
    vector *= 0.5;

    assert_eq!(vector, Vector3::new(1.25, -0.5, 0.55));
}

#[test]
fn vector_3_mul_assign_vector() {
    let mut vector = Vector3::new(2.5, -1.0, 7.5);
    vector *= Vector3::new(4.0, -11.0, 2.0);

    assert_eq!(vector, Vector3::new(10.0, 11.0, 15.0));
}

#[test]
fn vector_3_div_scalar() {
    let vector = Vector3::new(1.0, -2.0, -3.0);
    let result = vector / 3.0;

    assert_eq!(result, Vector3::new(0.33333334, -0.6666667, -1.0));
}

#[test]
fn vector_3_div_vector() {
    let dividend = Vector3::new(5.0, 6.0, 4.0);
    let divider = Vector3::new(-1.0, 4.5, 2.0);
    let result = dividend / divider;
    
    assert_eq!(result, Vector3::new(-5.0, 1.3333334, 2.0));
}

#[test]
fn vector_3_div_assign_scalar() {
    let mut vector = Vector3::new(2.5, -1.0, 1.5);
    vector /= 0.5;

    assert_eq!(vector, Vector3::new(5.0, -2.0, 3.0));
}

#[test]
fn vector_3_div_assign_vector() {
    let mut vector = Vector3::new(2.5, -1.0, -1.1);
    vector /= Vector3::new(4.0, -11.0, 10.0);

    assert_eq!(vector, Vector3::new(0.625, 0.09090909, -0.11));
}

#[test]
fn vector_3_neg() {
    let vector = -Vector3::new(-1.0, 5.3, 4.4);

    assert_eq!(vector, Vector3::new(1.0, -5.3, -4.4));
}

#[test]
fn bit_vector_3_and_scalar() {
    let vector = UVector3::new(2, 4, 6);
    let result = vector & 2;

    assert_eq!(result, UVector3::new(2, 0, 2));
}

#[test]
fn bit_vector_3_and_vector() {
    let vector = UVector3::new(4, 1, 7);
    let result = vector & UVector3::new(1, 1, 1);

    assert_eq!(result, UVector3::new(0, 1, 1));
}

#[test]
fn bit_vector_3_and_assign_scalar() {
    let mut vector = UVector3::new(3, 7, 4);
    vector &= 3;

    assert_eq!(vector, UVector3::new(3, 3, 0));
}

#[test]
fn bit_vector_3_and_assign_vector() {
    let mut vector = UVector3::new(9, 3, 4);
    vector &= UVector3::new(3, 9, 4);

    assert_eq!(vector, UVector3::new(1, 1, 4));
}

#[test]
fn bit_vector_3_or_scalar() {
    let vector = UVector3::new(2, 4, 6);
    let result = vector | 2;

    assert_eq!(result, UVector3::new(2, 6, 6));
}

#[test]
fn bit_vector_3_or_vector() {
    let vector = UVector3::new(4, 1, 3);
    let result = vector | UVector3::new(1, 1, 1);

    assert_eq!(result, UVector3::new(5, 1, 3));
}

#[test]
fn bit_vector_3_or_assign_scalar() {
    let mut vector = UVector3::new(3, 7, 6);
    vector |= 3;

    assert_eq!(vector, UVector3::new(3, 7, 7));
}

#[test]
fn bit_vector_3_or_assign_vector() {
    let mut vector = UVector3::new(9, 3, 6);
    vector |= UVector3::new(3, 9, 6);

    assert_eq!(vector, UVector3::new(11, 11, 6));
}

#[test]
fn bit_vector_3_xor_scalar() {
    let vector = UVector3::new(2, 4, 6);
    let result = vector ^ 2;

    assert_eq!(result, UVector3::new(0, 6, 4));
}

#[test]
fn bit_vector_3_xor_vector() {
    let vector = UVector3::new(4, 1, 2);
    let result = vector ^ UVector3::new(1, 1, 2);

    assert_eq!(result, UVector3::new(5, 0, 0));
}

#[test]
fn bit_vector_3_xor_assign_scalar() {
    let mut vector = UVector3::new(3, 7, 9);
    vector ^= 3;

    assert_eq!(vector, UVector3::new(0, 4, 10));
}

#[test]
fn bit_vector_3_xor_assign_vector() {
    let mut vector = UVector3::new(9, 3, 5);
    vector ^= UVector3::new(3, 9, 7);

    assert_eq!(vector, UVector3::new(10, 10, 2));
}

#[test]
fn vector_3_rem_scalar() {
    let vector = Vector3::new(15.6, 11.0, 5.0);
    let result = vector % 10.0;

    assert_eq!(result, Vector3::new(5.6000004, 1.0, 5.0));
}

#[test]
fn vector_3_rem_vector() {
    let vector = Vector3::new(5.3, 7.0, 3.0);
    let result = vector % Vector3::new(5.3, 3.5, 2.5);

    assert_eq!(result, Vector3::new(0.0, 0.0, 0.5));
}

#[test]
fn vector_3_rem_assign_scalar() {
    let mut vector = Vector3::new(15.6, 11.0, 12.0);
    vector %= 10.0;

    assert_eq!(vector, Vector3::new(5.6000004, 1.0, 2.0));
}

#[test]
fn vector_3_rem_assign_vector() {
    let mut vector = Vector3::new(5.3, 7.0, 8.4);
    vector %= Vector3::new(5.3, 3.5, 4.3);

    assert_eq!(vector, Vector3::new(0.0, 0.0, 4.0999994));
}

#[test]
fn vector_3_shl_scalar() {
    let vector = UVector3::new(2, 3, 4);
    let result = vector << 4;

    assert_eq!(result, UVector3::new(32, 48, 64));
}

#[test]
fn vector_3_shl_vector() {
    let vector = UVector3::new(4, 5, 6);
    let result = vector << UVector3::new(1, 2, 3);

    assert_eq!(result, UVector3::new(8, 20, 48));
}

#[test]
fn vector_3_shl_assign_scalar() {
    let mut vector = UVector3::new(2, 3, 4);
    vector <<= 4;

    assert_eq!(vector, UVector3::new(32, 48, 64));
}

#[test]
fn vector_3_shl_assign_vector() {
    let mut vector = UVector3::new(4, 5, 6);
    vector <<= UVector3::new(1, 2, 3);

    assert_eq!(vector, UVector3::new(8, 20, 48));
}

#[test]
fn vector_3_shr_scalar() {
    let vector = UVector3::new(2, 10, 20);
    let result = vector >> 3;

    assert_eq!(result, UVector3::new(0, 1, 2));
}

#[test]
fn vector_3_shr_vector() {
    let vector = UVector3::new(17, 26, 14);
    let result = vector >> UVector3::new(4, 2, 3);

    assert_eq!(result, UVector3::new(1, 6, 1));
}

#[test]
fn vector_3_shr_assign_scalar() {
    let mut vector = UVector3::new(2, 10, 20);
    vector >>= 3;

    assert_eq!(vector, UVector3::new(0, 1, 2));
}

#[test]
fn vector_3_shr_assign_vector() {
    let mut vector = UVector3::new(17, 26, 14);
    vector >>= UVector3::new(4, 2, 3);

    assert_eq!(vector, UVector3::new(1, 6, 1));
}

#[test]
fn vector_3_debug_struct() {
    let vector = Vector3::new(11.0, 4.5, -9.0);

    assert_eq!(format!("{:?}", vector), "TVector3<f32> { x: 11.0, y: 4.5, z: -9.0 }");
}