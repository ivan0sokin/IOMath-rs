use iomath::vectors::{ Vector2, UVector2, Vector3, Vector4 };
use iomath::quaternions::Quaternion;

#[test]
fn vector_2_empty() {
    let vector = Vector2::empty();

    assert_eq!(vector, Vector2 { x: 0.0, y: 0.0 });
}

#[test]
fn vector_2_new() {
    let vector = Vector2::new(-0.5, 1.333);

    assert_eq!(vector, Vector2 { x: -0.5, y: 1.333 });
}

#[test]
fn vector_2_from_scalar() {
    let vector = Vector2::from_scalar(1.7);

    assert_eq!(vector, Vector2 { x: 1.7, y: 1.7 });
}

#[test]
fn vector_2_from_vector_3() {
    let vector = Vector2::from_vector_3(Vector3::new(1.9, 5.6, 0.7));

    assert_eq!(vector, Vector2::new(1.9, 5.6));
}

#[test]
fn vector_2_from_vector_4() {
    let vector = Vector2::from_vector_4(Vector4::new(1.9, 5.6, 0.7, 8.7));

    assert_eq!(vector, Vector2::new(1.9, 5.6));
}

#[test]
fn vector_2_from_quaternion() {
    let vector = Vector2::from_quaternion(Quaternion::identity());

    assert_eq!(vector, Vector2::new(0.0, 0.0));
}

#[test]
fn vector_2_copy() {
    let vector_from = Vector2::new(0.0, 1.1);
    let vector_to = vector_from;

    assert_eq!(vector_from, Vector2::new(0.0, 1.1));
    assert_eq!(vector_to, Vector2::new(0.0, 1.1));
}

#[test]
fn vector_2_index() {
    let vector = Vector2::new(1.3, 2.7);

    assert_eq!(vector, Vector2::new(vector[0], vector[1]));
}

#[test]
fn vector_2_index_out_of_bounds() {
    let vector = Vector2::new(7.3, 2.4);
    
    assert_eq!(vector, Vector2::new(vector[0], vector[256]))
}

#[test]
fn vector_2_index_mut() {
    let mut vector = Vector2::new(5.6, 3.8);
    vector[0] *= 0.5;
    vector[1] *= 2.0;

    assert_eq!(vector, Vector2::new(2.8, 7.6));
}

#[test]
fn vector_2_index_mut_out_of_bounds() {
    let mut vector = Vector2::new(4.5, -7.6);
    vector[1024] = -1.0;

    assert_eq!(vector, Vector2::new(4.5, -1.0));
}

#[test]
fn vector_2_add_scalar() {
    let vector = Vector2::new(1.0, -2.0);
    let result = vector + 3.0;

    assert_eq!(result, Vector2::new(4.0, 1.0));
}

#[test]
fn vector_2_add_vector() {
    let first_add = Vector2::new(5.0, 6.0);
    let second_add = Vector2::new(-1.0, 4.5);
    let result = first_add + second_add;
    
    assert_eq!(result, Vector2::new(4.0, 10.5));
}

#[test]
fn vector_2_add_assign_scalar() {
    let mut vector = Vector2::new(2.5, -1.0);
    vector += 0.5;

    assert_eq!(vector, Vector2::new(3.0, -0.5));
}

#[test]
fn vector_2_add_assign_vector() {
    let mut vector = Vector2::new(2.5, -1.0);
    vector += Vector2::new(0.5, 1.0);

    assert_eq!(vector, Vector2::new(3.0, 0.0));
}

#[test]
fn vector_2_sub_scalar() {
    let vector = Vector2::new(1.0, -2.0);
    let result = vector - 3.0;

    assert_eq!(result, Vector2::new(-2.0, -5.0));
}

#[test]
fn vector_2_sub_vector() {
    let minuend = Vector2::new(5.0, 6.0);
    let subtrahend = Vector2::new(-1.0, 4.5);
    let result = minuend - subtrahend;
    
    assert_eq!(result, Vector2::new(6.0, 1.5));
}

#[test]
fn vector_2_sub_assign_scalar() {
    let mut vector = Vector2::new(2.5, -1.0);
    vector -= 0.5;

    assert_eq!(vector, Vector2::new(2.0, -1.5));
}

#[test]
fn vector_2_sub_assign_vector() {
    let mut vector = Vector2::new(2.5, -1.0);
    vector -= Vector2::new(0.5, 1.0);

    assert_eq!(vector, Vector2::new(2.0, -2.0));
}

#[test]
fn vector_2_mul_scalar() {
    let vector = Vector2::new(1.0, -2.0);
    let result = vector * 3.0;

    assert_eq!(result, Vector2::new(3.0, -6.0));
}

#[test]
fn vector_2_mul_vector() {
    let first_mul = Vector2::new(5.0, 6.0);
    let second_mul = Vector2::new(-1.0, 4.5);
    let result = first_mul * second_mul;
    
    assert_eq!(result, Vector2::new(-5.0, 27.0));
}

#[test]
fn vector_2_mul_assign_scalar() {
    let mut vector = Vector2::new(2.5, -1.0);
    vector *= 0.5;

    assert_eq!(vector, Vector2::new(1.25, -0.5));
}

#[test]
fn vector_2_mul_assign_vector() {
    let mut vector = Vector2::new(2.5, -1.0);
    vector *= Vector2::new(4.0, -11.0);

    assert_eq!(vector, Vector2::new(10.0, 11.0));
}

#[test]
fn vector_2_div_scalar() {
    let vector = Vector2::new(1.0, -2.0);
    let result = vector / 3.0;

    assert_eq!(result, Vector2::new(0.33333334, -0.6666667));
}

#[test]
fn vector_2_div_vector() {
    let dividend = Vector2::new(5.0, 6.0);
    let divider = Vector2::new(-1.0, 4.5);
    let result = dividend / divider;
    
    assert_eq!(result, Vector2::new(-5.0, 1.3333334));
}

#[test]
fn vector_2_div_assign_scalar() {
    let mut vector = Vector2::new(2.5, -1.0);
    vector /= 0.5;

    assert_eq!(vector, Vector2::new(5.0, -2.0));
}

#[test]
fn vector_2_div_assign_vector() {
    let mut vector = Vector2::new(2.5, -1.0);
    vector /= Vector2::new(4.0, -11.0);

    assert_eq!(vector, Vector2::new(0.625, 0.09090909));
}

#[test]
fn vector_2_neg() {
    let vector = -Vector2::new(-1.0, 5.3);

    assert_eq!(vector, Vector2::new(1.0, -5.3));
}

#[test]
fn bit_vector_2_and_scalar() {
    let vector = UVector2::new(2, 4);
    let result = vector & 2;

    assert_eq!(result, UVector2::new(2, 0));
}

#[test]
fn bit_vector_2_and_vector() {
    let vector = UVector2::new(4, 1);
    let result = vector & UVector2::new(1, 1);

    assert_eq!(result, UVector2::new(0, 1));
}

#[test]
fn bit_vector_2_and_assign_scalar() {
    let mut vector = UVector2::new(3, 7);
    vector &= 3;

    assert_eq!(vector, UVector2::new(3, 3));
}

#[test]
fn bit_vector_2_and_assign_vector() {
    let mut vector = UVector2::new(9, 3);
    vector &= UVector2::new(3, 9);

    assert_eq!(vector, UVector2::new(1, 1));
}

#[test]
fn bit_vector_2_or_scalar() {
    let vector = UVector2::new(2, 4);
    let result = vector | 2;

    assert_eq!(result, UVector2::new(2, 6));
}

#[test]
fn bit_vector_2_or_vector() {
    let vector = UVector2::new(4, 1);
    let result = vector | UVector2::new(1, 1);

    assert_eq!(result, UVector2::new(5, 1));
}

#[test]
fn bit_vector_2_or_assign_scalar() {
    let mut vector = UVector2::new(3, 7);
    vector |= 3;

    assert_eq!(vector, UVector2::new(3, 7));
}

#[test]
fn bit_vector_2_or_assign_vector() {
    let mut vector = UVector2::new(9, 3);
    vector |= UVector2::new(3, 9);

    assert_eq!(vector, UVector2::new(11, 11));
}

#[test]
fn bit_vector_2_xor_scalar() {
    let vector = UVector2::new(2, 4);
    let result = vector ^ 2;

    assert_eq!(result, UVector2::new(0, 6));
}

#[test]
fn bit_vector_2_xor_vector() {
    let vector = UVector2::new(4, 1);
    let result = vector ^ UVector2::new(1, 1);

    assert_eq!(result, UVector2::new(5, 0));
}

#[test]
fn bit_vector_2_xor_assign_scalar() {
    let mut vector = UVector2::new(3, 7);
    vector ^= 3;

    assert_eq!(vector, UVector2::new(0, 4));
}

#[test]
fn bit_vector_2_xor_assign_vector() {
    let mut vector = UVector2::new(9, 3);
    vector ^= UVector2::new(3, 9);

    assert_eq!(vector, UVector2::new(10, 10));
}

#[test]
fn vector_2_rem_scalar() {
    let vector = Vector2::new(15.6, 11.0);
    let result = vector % 10.0;

    assert_eq!(result, Vector2::new(5.6000004, 1.0));
}

#[test]
fn vector_2_rem_vector() {
    let vector = Vector2::new(5.3, 7.0);
    let result = vector % Vector2::new(5.3, 3.5);

    assert_eq!(result, Vector2::new(0.0, 0.0));
}

#[test]
fn vector_2_rem_assign_scalar() {
    let mut vector = Vector2::new(15.6, 11.0);
    vector %= 10.0;

    assert_eq!(vector, Vector2::new(5.6000004, 1.0));
}

#[test]
fn vector_2_rem_assign_vector() {
    let mut vector = Vector2::new(5.3, 7.0);
    vector %= Vector2::new(5.3, 3.5);

    assert_eq!(vector, Vector2::new(0.0, 0.0));
}

#[test]
fn vector_2_shl_scalar() {
    let vector = UVector2::new(2, 3);
    let result = vector << 4;

    assert_eq!(result, UVector2::new(32, 48));
}

#[test]
fn vector_2_shl_vector() {
    let vector = UVector2::new(4, 5);
    let result = vector << UVector2::new(1, 2);

    assert_eq!(result, UVector2::new(8, 20));
}

#[test]
fn vector_2_shl_assign_scalar() {
    let mut vector = UVector2::new(2, 3);
    vector <<= 4;

    assert_eq!(vector, UVector2::new(32, 48));
}

#[test]
fn vector_2_shl_assign_vector() {
    let mut vector = UVector2::new(4, 5);
    vector <<= UVector2::new(1, 2);

    assert_eq!(vector, UVector2::new(8, 20));
}

#[test]
fn vector_2_shr_scalar() {
    let vector = UVector2::new(2, 10);
    let result = vector >> 3;

    assert_eq!(result, UVector2::new(0, 1));
}

#[test]
fn vector_2_shr_vector() {
    let vector = UVector2::new(17, 26);
    let result = vector >> UVector2::new(4, 2);

    assert_eq!(result, UVector2::new(1, 6));
}

#[test]
fn vector_2_shr_assign_scalar() {
    let mut vector = UVector2::new(2, 10);
    vector >>= 3;

    assert_eq!(vector, UVector2::new(0, 1));
}

#[test]
fn vector_2_shr_assign_vector() {
    let mut vector = UVector2::new(17, 26);
    vector >>= UVector2::new(4, 2);

    assert_eq!(vector, UVector2::new(1, 6));
}

#[test]
fn vector_2_debug_struct() {
    let vector = Vector2::new(11.0, 4.5);

    assert_eq!(format!("{:?}", vector), "TVector2<f32> { x: 11.0, y: 4.5 }");
}