use bigdecimal::BigDecimal;
use fixed_width::FixedWidth;

#[test]
fn test_bigdecimal_simple() {
    #[derive(Debug, FixedWidth)]
    struct Test {
        #[fixed_width(size = 10)]
        field1: Option<String>,
        #[fixed_width(size = 10, decimals = 3)]
        number1: Option<BigDecimal>,
        #[fixed_width(size = 10)]
        field2: Option<String>,
    }

    //let bd: BigDecimal = BigDecimal::from_f64(24.25).unwrap();

    let test = Test {
        field1: Some("a".to_string()),
        number1: None,
        field2: Some("b".to_string()),
    };

    let s = test.to_fixed_width_string().unwrap();

    println!("{}", s);
}
