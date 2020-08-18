use iomath::vectors::{ Vector4, UVector4, Vector2, Vector3 };
use iomath::quaternions::Quaternion;

#[test]
fn vector_4_empty() {
    let vector = Vector4::empty();

    assert_eq!(vector, Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 });
}

#[test]
fn vector_4_new() {
    let vector = Vector4::new(-0.5, 1.333, 3.7, 2.5);

    assert_eq!(vector, Vector4 { x: -0.5, y: 1.333, z: 3.7, w: 2.5 });
}

#[test]
fn vector_4_from_scalar() {
    let vector = Vector4::from_scalar(1.7);

    assert_eq!(vector, Vector4 { x: 1.7, y: 1.7, z: 1.7, w: 1.7 });
}

#[test]
fn vector_4_from_two_values_vector() {
    let vector = Vector4::from_two_values_vector(1.0, 2.0, Vector2::new(4.0, 3.0));

    assert_eq!(vector, Vector4::new(1.0, 2.0, 4.0, 3.0));
}

#[test]
fn vector_4_from_value_vector_value() {
    let vector = Vector4::from_value_vector_value(1.0, Vector2::new(4.0, 3.0), 2.0);

    assert_eq!(vector, Vector4::new(1.0, 4.0, 3.0, 2.0));
}

#[test]
fn vector_4_from_vector_two_values() {
    let vector = Vector4::from_vector_two_values(Vector2::new(4.0, 3.0), 2.0, 1.0);

    assert_eq!(vector, Vector4::new(4.0, 3.0, 2.0, 1.0));
}

#[test]
fn vector_4_from_two_vectors() {
    let vector = Vector4::from_two_vectors(Vector2::new(2.0, 1.0), Vector2::new(4.0, 3.0));

    assert_eq!(vector, Vector4::new(2.0, 1.0, 4.0, 3.0));
}

#[test]
fn vector_4_from_value_vector() {
    let vector = Vector4::from_value_vector(4.5, Vector3::new(9.8, 3.8, 4.4));

    assert_eq!(vector, Vector4::new(4.5, 9.8, 3.8, 4.4));
}

#[test]
fn vector_4_from_vector_value() {
    let vector = Vector4::from_vector_value(Vector3::new(9.8, 3.8, 4.4), 4.5);

    assert_eq!(vector, Vector4::new(9.8, 3.8, 4.4, 4.5));
}

#[test]
fn vector_4_from_quaternion() {
    let vector = Vector4::from_quaternion(Quaternion::identity());

    assert_eq!(vector, Vector4::new(0.0, 0.0, 0.0, 1.0));
}

#[test]
fn vector_4_copy() {
    let vector_from = Vector4::new(0.0, 1.1, 2.2, 3.3);
    let vector_to = vector_from;

    assert_eq!(vector_from, Vector4::new(0.0, 1.1, 2.2, 3.3));
    assert_eq!(vector_to, Vector4::new(0.0, 1.1, 2.2, 3.3));
}

#[test]
fn vector_4_from_vector_2() {
    let vector = Vector4::from(Vector2::new(1.5, -5.6));

    assert_eq!(vector, Vector4::new(1.5, -5.6, 0.0, 0.0));
}

#[test]
fn vector_4_from_vector_3() {
    let vector = Vector4::from(Vector3::new(1.5, -5.6, 2.4));

    assert_eq!(vector, Vector4::new(1.5, -5.6, 2.4, 0.0));
}

#[test]
fn vector_4_index() {
    let vector = Vector4::new(1.3, 2.7, 0.4, 3.5);

    assert_eq!(vector, Vector4::new(vector[0], vector[1], vector[2], vector[3]));
}

#[test]
fn vector_4_index_out_of_bounds() {
    let vector = Vector4::new(7.3, 2.4, -3.8, 4.2);
    
    assert_eq!(vector, Vector4::new(vector[0], vector[1], vector[2], vector[256]))
}

#[test]
fn vector_4_index_mut() {
    let mut vector = Vector4::new(5.6, 3.8, 27.0, 11.0);
    vector[0] *= 0.5;
    vector[1] *= 2.0;
    vector[2] /= 3.0;
    vector[3] -= 9.0;

    assert_eq!(vector, Vector4::new(2.8, 7.6, 9.0, 2.0));
}

#[test]
fn vector_4_index_mut_out_of_bounds() {
    let mut vector = Vector4::new(4.5, -7.6, 7.2, 15.0);
    vector[1024] = -1.0;

    assert_eq!(vector, Vector4::new(4.5, -7.6, 7.2, -1.0));
}

#[test]
fn vector_4_add_scalar() {
    let vector = Vector4::new(1.0, -2.0, 3.0, -4.0);
    let result = vector + 3.0;

    assert_eq!(result, Vector4::new(4.0, 1.0, 6.0, -1.0));
}

#[test]
fn vector_4_add_vector() {
    let first_add = Vector4::new(5.0, 6.0, 4.0, 3.0);
    let second_add = Vector4::new(-1.0, 4.5, 3.2, 1.3);
    let result = first_add + second_add;
    
    assert_eq!(result, Vector4::new(4.0, 10.5, 7.2, 4.3));
}

#[test]
fn vector_4_add_assign_scalar() {
    let mut vector = Vector4::new(2.5, -1.0, 9.9, -0.0001);
    vector += 0.5;

    assert_eq!(vector, Vector4::new(3.0, -0.5, 10.4, 0.4999));
}

#[test]
fn vector_4_add_assign_vector() {
    let mut vector = Vector4::new(2.5, -1.0, 3.2, 3.5);
    vector += Vector4::new(0.5, 1.0, -3.0, 3.2);

    assert_eq!(vector, Vector4::new(3.0, 0.0, 0.20000005, 6.7));
}

#[test]
fn vector_4_sub_scalar() {
    let vector = Vector4::new(1.0, -2.0, 3.0, -4.0);
    let result = vector - 3.0;

    assert_eq!(result, Vector4::new(-2.0, -5.0, 0.0, -7.0));
}

#[test]
fn vector_4_sub_vector() {
    let minuend = Vector4::new(5.0, 6.0, 4.0, 3.0);
    let subtrahend = Vector4::new(-1.0, 4.5, 10.0, 2.0);
    let result = minuend - subtrahend;
    
    assert_eq!(result, Vector4::new(6.0, 1.5, -6.0, 1.0));
}

#[test]
fn vector_4_sub_assign_scalar() {
    let mut vector = Vector4::new(2.5, -1.0, 4.4, 2.2);
    vector -= 0.5;

    assert_eq!(vector, Vector4::new(2.0, -1.5, 3.9, 1.7));
}

#[test]
fn vector_4_sub_assign_vector() {
    let mut vector = Vector4::new(2.5, -1.0, 2.2, 6.7);
    vector -= Vector4::new(0.5, 1.0, 0.7, 0.7);

    assert_eq!(vector, Vector4::new(2.0, -2.0, 1.5, 6.0));
}

#[test]
fn vector_4_mul_scalar() {
    let vector = Vector4::new(1.0, -2.0, 3.0, -4.0);
    let result = vector * 3.0;

    assert_eq!(result, Vector4::new(3.0, -6.0, 9.0, -12.0));
}

#[test]
fn vector_4_mul_vector() {
    let first_mul = Vector4::new(5.0, 6.0, 4.0, 3.0);
    let second_mul = Vector4::new(-1.0, 4.5, 2.5, 4.0);
    let result = first_mul * second_mul;
    
    assert_eq!(result, Vector4::new(-5.0, 27.0, 10.0, 12.0));
}

#[test]
fn vector_4_mul_assign_scalar() {
    let mut vector = Vector4::new(2.5, -1.0, 1.1, -3.33);
    vector *= 0.5;

    assert_eq!(vector, Vector4::new(1.25, -0.5, 0.55, -1.665));
}

#[test]
fn vector_4_mul_assign_vector() {
    let mut vector = Vector4::new(2.5, -1.0, 7.5, 5.3);
    vector *= Vector4::new(4.0, -11.0, 2.0, -1.5);

    assert_eq!(vector, Vector4::new(10.0, 11.0, 15.0, -7.9500003));
}

#[test]
fn vector_4_div_scalar() {
    let vector = Vector4::new(1.0, -2.0, -3.0, 4.0);
    let result = vector / 3.0;

    assert_eq!(result, Vector4::new(0.33333334, -0.6666667, -1.0, 1.3333334));
}

#[test]
fn vector_4_div_vector() {
    let dividend = Vector4::new(5.0, 6.0, 4.0, 3.0);
    let divider = Vector4::new(-1.0, 4.5, 2.0, 5.0);
    let result = dividend / divider;
    
    assert_eq!(result, Vector4::new(-5.0, 1.3333334, 2.0, 0.6));
}

#[test]
fn vector_4_div_assign_scalar() {
    let mut vector = Vector4::new(2.5, -1.0, 1.5, 4.0);
    vector /= 0.5;

    assert_eq!(vector, Vector4::new(5.0, -2.0, 3.0, 8.0));
}

#[test]
fn vector_4_div_assign_vector() {
    let mut vector = Vector4::new(2.5, -1.0, -1.1, 5.3);
    vector /= Vector4::new(4.0, -11.0, 10.0, 3.0);

    assert_eq!(vector, Vector4::new(0.625, 0.09090909, -0.11, 1.7666668));
}

#[test]
fn vector_4_neg() {
    let vector = -Vector4::new(-1.0, 5.3, 4.4, -7.2);

    assert_eq!(vector, Vector4::new(1.0, -5.3, -4.4, 7.2));
}

#[test]
fn bit_vector_4_and_scalar() {
    let vector = UVector4::new(2, 4, 6, 8);
    let result = vector & 2;

    assert_eq!(result, UVector4::new(2, 0, 2, 0));
}

#[test]
fn bit_vector_4_and_vector() {
    let vector = UVector4::new(4, 1, 7, 8);
    let result = vector & UVector4::new(1, 1, 1, 4);

    assert_eq!(result, UVector4::new(0, 1, 1, 0));
}

#[test]
fn bit_vector_4_and_assign_scalar() {
    let mut vector = UVector4::new(3, 7, 4, 5);
    vector &= 3;

    assert_eq!(vector, UVector4::new(3, 3, 0, 1));
}

#[test]
fn bit_vector_4_and_assign_vector() {
    let mut vector = UVector4::new(9, 3, 4, 6);
    vector &= UVector4::new(3, 9, 4, 7);

    assert_eq!(vector, UVector4::new(1, 1, 4, 6));
}

#[test]
fn bit_vector_4_or_scalar() {
    let vector = UVector4::new(2, 4, 6, 8);
    let result = vector | 2;
    
    assert_eq!(result, UVector4::new(2, 6, 6, 10));
}

#[test]
fn bit_vector_4_or_vector() {
    let vector = UVector4::new(4, 1, 3, 7);
    let result = vector | UVector4::new(1, 1, 1, 5);

    assert_eq!(result, UVector4::new(5, 1, 3, 7));
}

#[test]
fn bit_vector_4_or_assign_scalar() {
    let mut vector = UVector4::new(3, 7, 6, 9);
    vector |= 3;
    
    assert_eq!(vector, UVector4::new(3, 7, 7, 11));
}

#[test]
fn bit_vector_4_or_assign_vector() {
    let mut vector = UVector4::new(9, 3, 6, 5);
    vector |= UVector4::new(3, 9, 6, 6);

    assert_eq!(vector, UVector4::new(11, 11, 6, 7));
}

#[test]
fn bit_vector_4_xor_scalar() {
    let vector = UVector4::new(2, 4, 6, 8);
    let result = vector ^ 2;

    assert_eq!(result, UVector4::new(0, 6, 4, 10));
}

#[test]
fn bit_vector_4_xor_vector() {
    let vector = UVector4::new(4, 1, 2, 7);
    let result = vector ^ UVector4::new(1, 1, 2, 5);

    assert_eq!(result, UVector4::new(5, 0, 0, 2));
}

#[test]
fn bit_vector_4_xor_assign_scalar() {
    let mut vector = UVector4::new(3, 7, 9, 6);
    vector ^= 3;

    assert_eq!(vector, UVector4::new(0, 4, 10, 5));
}

#[test]
fn bit_vector_4_xor_assign_vector() {
    let mut vector = UVector4::new(9, 3, 5, 2);
    vector ^= UVector4::new(3, 9, 7, 15);

    assert_eq!(vector, UVector4::new(10, 10, 2, 13));
}

#[test]
fn vector_4_rem_scalar() {
    let vector = Vector4::new(15.6, 11.0, 5.0, 2.2);
    let result = vector % 10.0;

    assert_eq!(result, Vector4::new(5.6000004, 1.0, 5.0, 2.2));
}

#[test]
fn vector_4_rem_vector() {
    let vector = Vector4::new(5.3, 7.0, 3.0, 7.5);
    let result = vector % Vector4::new(5.3, 3.5, 2.5, 5.2);

    assert_eq!(result, Vector4::new(0.0, 0.0, 0.5, 2.3000002));
}

#[test]
fn vector_4_rem_assign_scalar() {
    let mut vector = Vector4::new(15.6, 11.0, 12.0, 15.0);
    vector %= 10.0;

    assert_eq!(vector, Vector4::new(5.6000004, 1.0, 2.0, 5.0));
}

#[test]
fn vector_4_rem_assign_vector() {
    let mut vector = Vector4::new(5.3, 7.0, 8.4, 14.0);
    vector %= Vector4::new(5.3, 3.5, 4.3, 8.7);

    assert_eq!(vector, Vector4::new(0.0, 0.0, 4.0999994, 5.3));
}

#[test]
fn vector_4_shl_scalar() {
    let vector = UVector4::new(2, 3, 4, 5);
    let result = vector << 4;

    assert_eq!(result, UVector4::new(32, 48, 64, 80));
}

#[test]
fn vector_4_shl_vector() {
    let vector = UVector4::new(4, 5, 6, 7);
    let result = vector << UVector4::new(1, 2, 3, 2);

    assert_eq!(result, UVector4::new(8, 20, 48, 28));
}

#[test]
fn vector_4_shl_assign_scalar() {
    let mut vector = UVector4::new(2, 3, 4, 5);
    vector <<= 4;

    assert_eq!(vector, UVector4::new(32, 48, 64, 80));
}

#[test]
fn vector_4_shl_assign_vector() {
    let mut vector = UVector4::new(4, 5, 6, 7);
    vector <<= UVector4::new(1, 2, 3, 2);

    assert_eq!(vector, UVector4::new(8, 20, 48, 28));
}

#[test]
fn vector_4_shr_scalar() {
    let vector = UVector4::new(2, 10, 20, 25);
    let result = vector >> 3;

    assert_eq!(result, UVector4::new(0, 1, 2, 3));
}

#[test]
fn vector_4_shr_vector() {
    let vector = UVector4::new(17, 26, 14, 23);
    let result = vector >> UVector4::new(4, 2, 3, 2);

    assert_eq!(result, UVector4::new(1, 6, 1, 5));
}

#[test]
fn vector_4_shr_assign_scalar() {
    let mut vector = UVector4::new(2, 10, 20, 25);
    vector >>= 3;

    assert_eq!(vector, UVector4::new(0, 1, 2, 3));
}

#[test]
fn vector_4_shr_assign_vector() {
    let mut vector = UVector4::new(17, 26, 14, 23);
    vector >>= UVector4::new(4, 2, 3, 2);

    assert_eq!(vector, UVector4::new(1, 6, 1, 5));
}

#[test]
fn vector_4_debug_struct() {
    let vector = Vector4::new(11.0, 4.5, -9.0, 5.2);

    assert_eq!(format!("{:?}", vector), "TVector4<f32> { x: 11.0, y: 4.5, z: -9.0, w: 5.2 }");
}