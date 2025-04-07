use bigdecimal::{BigDecimal, FromPrimitive};
use fixed_width::FixedWidth;

#[test]
fn test_bigdecimal_simple_3() {
    //let bd: BigDecimal = BigDecimal::from_f64(24.25).unwrap();

    /*let test = Test {
        field1: Some("a".to_string()),
        number1: BigDecimal::from_f64(0.0),
        field2: Some("b".to_string()),
    };

    let s = test.to_fixed_width_string().unwrap();

    println!("{}", s);*/

    test_test(Test::new("mario", 0.1, "rossi"));

    /*assert_eq!(
        test_test(Test::new("mario", 0.0, "rossi")),
        "     mario      000+     rossi"
    );
    assert_eq!(
        test_test(Test::new("mario", 1.0, "rossi")),
        "     mario      100+     rossi"
    );
    assert_eq!(
        test_test(Test::new("mario", 10.0, "rossi")),
        "     mario     1000+     rossi"
    );
    assert_eq!(
        test_test(Test::new("mario", 0.1, "rossi")),
        "     mario      010+     rossi"
    );
    assert_eq!(
        test_test(Test::new("mario", 0.10, "rossi")),
        "     mario      010+     rossi"
    );
    assert_eq!(
        test_test(Test::new("mario", 0.01, "rossi")),
        "     mario      001+     rossi"
    );
    assert_eq!(
        test_test(Test::new("mario", 1.01, "rossi")),
        "     mario      101+     rossi"
    );
    assert_eq!(
        test_test(Test::new("mario", 1.1, "rossi")),
        "     mario      110+     rossi"
    );
    assert_eq!(
        test_test(Test::new("mario", -0.0, "rossi")),
        "     mario      000+     rossi"
    );
    assert_eq!(
        test_test(Test::new("mario", -0.01, "rossi")),
        "     mario      001-     rossi"
    );
    assert_eq!(
        test_test(Test::new("mario", -0.1, "rossi")),
        "     mario      010-     rossi"
    );
    assert_eq!(
        test_test(Test::new("mario", -1.1, "rossi")),
        "     mario      110-     rossi"
    );
    assert_eq!(
        test_test(Test::new("mario", -1.11, "rossi")),
        "     mario      111-     rossi"
    );*/
}

fn test_test(t: Test) -> String {
    let s = t.to_fixed_width_string().unwrap();
    println!("{}", s);
    s
}

#[derive(Debug, FixedWidth)]
struct Test {
    #[fixed_width(size = 10)]
    field1: Option<String>,
    #[fixed_width(size = 10, decimals = 3, add_sign = true)]
    number1: Option<BigDecimal>,
    #[fixed_width(size = 10)]
    field2: Option<String>,
}

impl Test {
    fn new(field1: &str, number1: f64, field2: &str) -> Self {
        Self {
            field1: Some(field1.to_string()),
            number1: BigDecimal::from_f64(number1),
            field2: Some(field2.to_string()),
        }
    }
}
