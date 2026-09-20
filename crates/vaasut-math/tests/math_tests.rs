use vaasut_math::{Vec2, Vec3, vec2, vec3, Color, Transform2D, Transform3D, Rect};

#[test]
fn test_vec2_from_glam() {
    let v = vec2(3.0, 4.0);
    assert_eq!(v.length(), 5.0);
    assert_eq!(v + vec2(1.0, 1.0), vec2(4.0, 5.0));
}

#[test]
fn test_vec3_from_glam() {
    let v = vec3(1.0, 2.0, 3.0);
    assert_eq!(v + vec3(1.0, 1.0, 1.0), vec3(2.0, 3.0, 4.0));
}

#[test]
fn test_color_creation() {
    let c = Color::from_rgb8(255, 128, 64);
    assert!((c.r - 1.0).abs() < 0.01);
    assert!((c.g - 0.5).abs() < 0.01);
    assert!((c.b - 0.25).abs() < 0.01);
}

#[test]
fn test_transform2d_basic() {
    let mut t = Transform2D::default();
    t.translate(vec2(10.0, 20.0));
    assert_eq!(t.position, vec2(10.0, 20.0));
}

#[test]
fn test_transform3d_basic() {
    let mut t = Transform3D::default();
    t.translate(vec3(1.0, 2.0, 3.0));
    assert_eq!(t.position, vec3(1.0, 2.0, 3.0));
}

#[test]
fn test_rect_intersection() {
    let r1 = Rect::new(0.0, 0.0, 100.0, 100.0);
    let r2 = Rect::new(50.0, 50.0, 100.0, 100.0);
    assert!(r1.intersects(&r2));
}
