use fixed_width::FixedWidth;

#[test]
fn parse_simple_string_and_number() {
    #[derive(Debug, PartialEq, FixedWidth)]
    struct Test {
        #[fixed_width(size = 10)]
        name: String,
        #[fixed_width(size = 20)]
        description: String,
        #[fixed_width(size = 5)]
        age: u32,
    }

    // first: serialize
    let original = Test {
        name: "pippo".to_string(),
        description: "pippo descrizione".to_string(),
        age: 25,
    };
    let s = original.to_fixed_width_string().unwrap();
    assert_eq!("     pippo   pippo descrizione   25", &s);

    // then: parse back
    let parsed = Test::from_fixed_width_string(&s).unwrap();
    assert_eq!(parsed.name, "pippo");
    assert_eq!(parsed.description, "pippo descrizione");
    assert_eq!(parsed.age, 25);
}

#[test]
fn parse_with_list() {
    #[derive(Debug, PartialEq, FixedWidth)]
    struct Master {
        #[fixed_width(size = 10)]
        name: String,
        #[fixed_width(size = 40)]
        details: Vec<Detail>,
    }

    #[derive(Debug, PartialEq, FixedWidth)]
    struct Detail {
        #[fixed_width(size = 10)]
        detail: String,
        #[fixed_width(size = 5)]
        value: i32,
    }

    // serialize
    let original = Master {
        name: "pippo".to_string(),
        details: vec![
            Detail {
                detail: "det1".to_string(),
                value: 100,
            },
            Detail {
                detail: "det2".to_string(),
                value: 200,
            },
        ],
    };
    let s = original.to_fixed_width_string().unwrap();

    // parse back
    let parsed = Master::from_fixed_width_string(&s).unwrap();
    assert_eq!(parsed.name, "pippo");
    assert_eq!(parsed.details.len(), 2);
    assert_eq!(parsed.details[0].detail, "det1");
    assert_eq!(parsed.details[0].value, 100);
    assert_eq!(parsed.details[1].detail, "det2");
    assert_eq!(parsed.details[1].value, 200);
}
