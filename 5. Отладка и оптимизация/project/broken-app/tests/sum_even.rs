use broken_app::sum_even;

#[test]
fn sums_even_pos_numbers() {
    let nums = [1, 2, 3, 4];
    assert_eq!(sum_even(&nums), 6);
}

#[test]
fn sums_even_neg_numbers() {
    let nums = [-1, -2, -3, -4];
    assert_eq!(sum_even(&nums), -6);
}

#[test]
fn sums_empty() {
    let nums = [];
    assert_eq!(sum_even(&nums), 0);
}

#[test]
fn sums_one() {
    let nums = [1];
    assert_eq!(sum_even(&nums), 0);
}
