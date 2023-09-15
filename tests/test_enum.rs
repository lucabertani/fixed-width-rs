use fixed_width::{FixedWidth, FixedWidthEnum};

// cargo expand --test test_enum

#[test]
fn enum_test_simple() {
    #[derive(FixedWidth)]
    struct Test {
        #[fixed_width(size = 3)]
        persona_m: Persona,
        #[fixed_width(size = 3)]
        persona_f: Persona,
    }

    #[derive(FixedWidthEnum)]
    enum Persona {
        M,
        F,
    }

    let t = Test {
        persona_m: Persona::M,
        persona_f: Persona::F,
    };
    let s: String = t.to_fixed_width_string().unwrap();
    assert_eq!("  M  F".to_string(), s);
}

#[test]
fn enum_test_impl() {
    #[derive(FixedWidth)]
    struct Test {
        #[fixed_width(size = 10)]
        persona_m: Persona,
        #[fixed_width(size = 10)]
        persona_f: Persona,
    }

    enum Persona {
        M,
        F,
    }
    impl FixedWidthEnum for Persona {
        fn key(&self) -> String {
            match self {
                Persona::M => String::from("Maschio"),
                Persona::F => String::from("Femmina"),
            }
        }
    }

    let t = Test {
        persona_m: Persona::M,
        persona_f: Persona::F,
    };
    let s: String = t.to_fixed_width_string().unwrap();
    assert_eq!("   Maschio   Femmina".to_string(), s);
}
