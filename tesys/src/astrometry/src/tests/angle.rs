use crate::Angle;

#[test]
fn angle_default_wrap() {
    let test_prec = 1.0E9; // Round to nano degrees to avoid rounding problems. 
    let ang = Angle::new(390.0).wrap();
    let rnd = (ang.deg() * test_prec).round() / test_prec;
    assert_eq!(rnd, 30.0);
}

#[test]
fn angle_custom_wrap_neg() {
    let test_prec = 1.0E9; // Round to nano degrees to avoid rounding problems. 
    let ang = Angle::new(-100.0).wrap_with_bounds(- std::f64::consts::PI / 2.0, std::f64::consts::PI);
    let rnd = (ang.deg() * test_prec).round() / test_prec;
    assert_eq!(rnd, 170.0);
}


#[test]
fn angle_custom_wrap_pos() {
    let test_prec = 1.0E9; // Round to nano degrees to avoid rounding problems. 
    let ang = Angle::new(210.0).wrap_with_bounds(- std::f64::consts::PI / 2.0, std::f64::consts::PI);
    let rnd = (ang.deg() * test_prec).round() / test_prec;
    assert_eq!(rnd, -60.0);
}

#[test]
fn angle_custom_wrap_inside() {
    let test_prec = 1.0E9; // Round to nano degrees to avoid rounding problems. 
    let ang = Angle::new(-60.0).wrap_with_bounds(- std::f64::consts::PI / 2.0, std::f64::consts::PI);
    let rnd = (ang.deg() * test_prec).round() / test_prec;
    assert_eq!(rnd, -60.0);
}