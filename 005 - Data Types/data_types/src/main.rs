fn main() {
    let x: i32 = 2_000_000_000;
    let y: f64 = 2_000.32;

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, char);
    let (x, y, z) = (2, 4.0, 'A');
    tup = (x, y, z);
    let value = tup.2;
    let value = x;
}
