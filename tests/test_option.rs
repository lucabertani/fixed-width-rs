use fixed_width::FixedWidth;
use fixed_width::FixedWidthEnum;

// cargo test --test test_option

#[test]
fn option_test_simple() {
    #[derive(Debug, FixedWidth)]
    struct Test {
        #[fixed_width(size = 10)]
        name: Option<String>,
    }

    let t = Test {
        name: Some("pippo".to_string()),
    };
    let s: String = t.to_fixed_width_string().unwrap();

    assert_eq!("     pippo".to_string(), s);
}

#[test]
fn option_test_complex() {
    #[derive(Debug, FixedWidth)]
    struct Test {
        #[fixed_width(size = 10)]
        name: Option<String>,
        #[fixed_width(size = 10)]
        description: Option<String>,
        #[fixed_width(size = 10)]
        date: Option<time::PrimitiveDateTime>,
        #[fixed_width(size = 3, pad = "0", pad_left = "true")]
        age: Option<u32>,
    }

    let t = Test {
        name: Some("pippo".to_string()),
        description: None,
        date: None,
        age: Some(24),
    };
    let s: String = t.to_fixed_width_string().unwrap();

    assert_eq!("     pippo                    024".to_string(), s);
}

#[test]
fn option_enum_test() {
    #[derive(Debug, FixedWidth)]
    struct Test {
        #[fixed_width(size = 3)]
        persona1: Option<Persona>,

        #[fixed_width(size = 3)]
        persona2: Option<Persona>,
    }

    #[derive(Debug, FixedWidthEnum)]
    enum Persona {
        M,
        F,
    }

    let t = Test {
        persona1: Some(Persona::M),
        persona2: Some(Persona::F),
    };
    let s: String = t.to_fixed_width_string().unwrap();

    assert_eq!("  M  F".to_string(), s);
}

#[test]
fn option_test_pad_if_none_serialize() {
    #[derive(Debug, FixedWidth)]
    struct Test {
        #[fixed_width(size = 10, pad_if_none = "*")]
        name: Option<String>,
        #[fixed_width(size = 5, pad = "0", pad_if_none = "9")]
        age: Option<u32>,
    }

    let t = Test {
        name: None,
        age: None,
    };

    let s: String = t.to_fixed_width_string().unwrap();
    assert_eq!("**********99999".to_string(), s);
}

#[test]
fn option_test_pad_if_none_deserialize() {
    #[derive(Debug, FixedWidth)]
    struct Test {
        #[fixed_width(size = 10, pad_if_none = "*")]
        name: Option<String>,
    }

    let t = Test::from_fixed_width_string("**********").unwrap();
    assert_eq!(true, t.name.is_none());
}

#[test]
fn option_test_pad_if_none_roundtrip() {
    #[derive(Debug, FixedWidth)]
    struct Test {
        #[fixed_width(size = 8, pad_if_none = "#")]
        code: Option<String>,
        #[fixed_width(size = 5, pad = "0", pad_if_none = "9")]
        qty: Option<u32>,
    }

    let input = Test {
        code: None,
        qty: Some(42),
    };

    let s = input.to_fixed_width_string().unwrap();
    assert_eq!("########00042".to_string(), s);

    let parsed = Test::from_fixed_width_string(s.as_str()).unwrap();
    assert_eq!(true, parsed.code.is_none());
    assert_eq!(Some(42), parsed.qty);
}
