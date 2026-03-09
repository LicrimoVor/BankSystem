use broken_app::average_positive;

#[test]
fn averages_only_positive() {
    let nums = [5, 5, 15, 15];
    assert!((average_positive(&nums) - 10.0).abs() < f64::EPSILON);
}

#[test]
fn averages_no_positive() {
    let nums = [-5, -5, -15];
    assert!((average_positive(&nums) - 0.0).abs() < f64::EPSILON);
}

#[test]
fn averages_positive_empty() {
    let nums = [];
    assert!((average_positive(&nums) - 0.0).abs() < f64::EPSILON);
}

#[test]
fn averages_positive_zeros() {
    let nums = [4, 0, 0, 0];
    println!("{}", average_positive(&nums));
    assert!((average_positive(&nums) - 1.0).abs() < f64::EPSILON);
}
