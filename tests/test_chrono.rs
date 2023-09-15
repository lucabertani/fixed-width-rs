use fixed_width::FixedWidth;

#[test]
fn simple_test() {
    #[derive(Debug, FixedWidth)]
    struct Test {
        #[fixed_width(size = 10, date_format = "%d%m%Y")]
        date: chrono::NaiveDate,
    }

    let t = Test {
        date: chrono::NaiveDate::from_ymd_opt(2023, 09, 14).unwrap(),
    };
    let s: String = t.to_fixed_width_string().unwrap();

    assert_eq!("  14092023".to_string(), s);
}
