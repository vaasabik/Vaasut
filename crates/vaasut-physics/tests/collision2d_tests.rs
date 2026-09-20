use vaasut_physics::Aabb2D;
use vaasut_math::Vec2;

#[test]
fn test_aabb2d_intersection() {
    let a = Aabb2D::new(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0));
    let b = Aabb2D::new(Vec2::new(1.0, 1.0), Vec2::new(3.0, 3.0));
    assert!(a.intersects(&b));
}
