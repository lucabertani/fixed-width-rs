use fixed_width::FixedWidth;

#[test]
fn list_test_simple() {
    #[derive(Debug, FixedWidth)]
    struct Master {
        #[fixed_width(size = 10)]
        name: String,

        #[fixed_width(size = 100)]
        details: Vec<Detail>,
    }

    #[derive(Debug, FixedWidth)]
    struct Detail {
        #[fixed_width(size = 10)]
        detail: String,
    }

    let t = Master {
        name: "pippo".to_string(),
        details: vec![
            Detail {
                detail: "details1".to_string(),
            },
            Detail {
                detail: "details2".to_string(),
            },
        ],
    };
    let s: String = t.to_fixed_width_string().unwrap();

    assert_eq!("     pippo                                                                                  details1  details2".to_string(), s);
}

#[test]
fn list_test_complex() {
    #[derive(Debug, FixedWidth)]
    struct Master {
        #[fixed_width(size = 10)]
        name: String,

        #[fixed_width(size = 100)]
        details: Vec<Detail>,
    }

    #[derive(Debug, FixedWidth)]
    struct Detail {
        #[fixed_width(size = 10)]
        detail: String,

        #[fixed_width(size = 30)]
        sub_details: Vec<SubDetail>,
    }

    #[derive(Debug, FixedWidth)]
    struct SubDetail {
        #[fixed_width(size = 15)]
        sub_detail: String,
    }

    let t = Master {
        name: "pippo".to_string(),
        details: vec![
            Detail {
                detail: "details1".to_string(),
                sub_details: vec![
                    SubDetail {
                        sub_detail: "sub_details1".to_string(),
                    },
                    SubDetail {
                        sub_detail: "sub_details2".to_string(),
                    },
                ],
            },
            Detail {
                detail: "details2".to_string(),
                sub_details: vec![
                    SubDetail {
                        sub_detail: "sub_details3".to_string(),
                    },
                    SubDetail {
                        sub_detail: "sub_details4".to_string(),
                    },
                ],
            },
        ],
    };
    let s: String = t.to_fixed_width_string().unwrap();

    assert_eq!("     pippo                      details1   sub_details1   sub_details2  details2   sub_details3   sub_details4".to_string(), s);
}
