use vaasut_math::Vec2;

#[test]
fn test_vec2_length() {
    let v = Vec2::new(3.0, 4.0);
    assert_eq!(v.length(), 5.0);
}

#[test]
fn test_vec2_zero() {
    let v = Vec2::ZERO;
    assert_eq!(v.x, 0.0);
    assert_eq!(v.y, 0.0);
}
