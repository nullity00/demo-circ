fn main(a : Field, b : Field) -> pub Field {
    let c = a + b;
    assert(c == 77);
    c
}

#[test]
fn test_main() {
    let result = main(2, 75);
    assert(result == 77);
}
