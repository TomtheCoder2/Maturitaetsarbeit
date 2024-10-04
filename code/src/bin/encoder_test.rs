/// Truth table for the encoder
/// check(false, false, false, true, -1);
/// check(false, false, true, false, 1);
/// check(false, true, false, false, 1);
/// check(false, true, true, true, -1);
/// check(true, false, false, false, -1);
/// check(true, false, true, true, 1);
/// check(true, true, false, true, 1);
/// check(true, true, true, false, -1);
fn test(last_a: bool, last_b: bool, a: bool, b: bool) -> i32 {
    if last_a == a && last_b == b {
        panic!("No change: test({}, {}, {}, {})", last_a, last_b, a, b);
    }
    if last_a != a && last_b != b {
        panic!(
            "Both a and b changed: test({}, {}, {}, {})",
            last_a, last_b, a, b
        );
    }
    let mut position = 0;
    // deltas for a and b
    // println!("delta_a: {}, delta_b: {}", delta_a, delta_b);
    // the four a cases
    if (a as i32 - last_a as i32) == 0 {
        position += 1
            * if a { 1 } else { -1 }
            * if (b as i32 - last_b as i32) == 1 {
                1
            } else {
                -1
            };
    } else {
        position -= 1 * if b { 1 } else { -1 } * (a as i32 - last_a as i32);
    }

    // println!("a: {} -> {}\nb: {} -> {}\nposition: {}\n", last_a, a, last_b, b, position);
    println!("check({}, {}, {}, {}, {});", last_a, last_b, a, b, position);
    // println!("last_a: {}, last_b: {}, a: {}, b: {}, position: {}", last_a, last_b, a, b, position);
    return position;
}

fn check(last_a: bool, last_b: bool, a: bool, b: bool, target: i32) {
    println!(
        "assert_eq!(test({}, {}, {}, {}), {});",
        last_a, last_b, a, b, target
    );
    if test(last_a, last_b, a, b) == target {
        println!("Test passed");
    } else {
        panic!(
            "Test failed: check({}, {}, {}, {}, {}). got {} instead of {}",
            last_a,
            last_b,
            a,
            b,
            target,
            test(last_a, last_b, a, b),
            target
        );
    }
}

fn main() {
    // all cases
    // test(false, false, false, false);
    test(false, false, false, true);
    test(false, false, true, false);
    // test(false, false, true, true);
    test(false, true, false, false);
    // test(false, true, false, true);
    // test(false, true, true, false);
    test(false, true, true, true);
    test(true, false, false, false);
    // test(true, false, false, true);
    // test(true, false, true, false);
    test(true, false, true, true);
    // test(true, true, false, false);
    test(true, true, false, true);
    test(true, true, true, false);
    // test(true, true, true, true);
    check(false, false, false, true, -1);
    check(false, false, true, false, 1);
    check(false, true, false, false, 1);
    check(false, true, true, true, -1);
    check(true, false, false, false, -1);
    check(true, false, true, true, 1);
    check(true, true, false, true, 1);
    check(true, true, true, false, -1);
}
