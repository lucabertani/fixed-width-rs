use bigdecimal::{BigDecimal, FromPrimitive};
use fixed_width::{any_value::AnyValue, model::field_config::FieldConfig, FixedWidth};

#[test]
fn test_truncate_string() {
    #[derive(Debug, FixedWidth)]
    struct Test {
        #[fixed_width(size = 10)]
        first: String,
        #[fixed_width(size = 2)]
        middle: String,
        #[fixed_width(size = 10)]
        last: String,
    }

    let t = Test {
        first: "aaaa".to_string(),
        middle: "123".to_string(),
        last: "bbbb".to_string(),
    };
    let s: String = t.to_fixed_width_string().unwrap();
    println!("s: '{s}'");

    assert_eq!("      aaaa12      bbbb".to_string(), s);
}

#[test]
fn test_truncate_string_utf16() {
    #[derive(Debug, FixedWidth)]
    struct Test {
        #[fixed_width(size = 4)]
        first: String,
        #[fixed_width(size = 30)]
        middle: String,
        #[fixed_width(size = 4)]
        last: String,
    }

    let t = Test {
        first: "aaaa".to_string(),
        middle: "Rua das Mercês nº45  Fracção C".to_string(),
        //middle: "cccccccccccccccccccccccccccccc".to_string(),
        //middle: "ccccccccccccccccccccccccccccccc".to_string(),
        last: "bbbb".to_string(),
    };
    let s: String = t.to_fixed_width_string().unwrap();
    println!("s: '{s}'");
    assert_eq!(s.len(), 38);

    // è corretto che abbia lo spazio perché quella a strana occupa 2 spazi e mi andrebbe a 31 caratteri, ma ne voglio massimo 30
    // e quindi ho 29 caratteri e viene paddato con 1 spazio
    assert_eq!("aaaa Rua das Mercês nº45  Fracçbbbb".to_string(), s);
}

#[test]
fn test_truncate_string_utf16_len() {
    let s_orig = "Rua das Mercês nº45  Fracção C";

    let s = AnyValue::String(s_orig.to_string());
    let b = s
        .to_bytes(FieldConfig::new(
            "a",
            30,
            b' ',
            false,
            0,
            false,
            "[year][month][day]",
            "[hour padding:none][minute][second]",
            "[year][month][day] [hour padding:none][minute][second]",
        ))
        .unwrap();

    let s_trunc = String::from_utf8_lossy(&b);

    println!("Original string: {s_orig}");
    println!("Truncated string: {s_trunc}");

    println!("Size: {}", s_orig.len());
    println!("Size: {}", s_trunc.len());
}

#[test]
fn test_truncate_bigdecimal() {
    #[derive(Debug, FixedWidth)]
    struct Test {
        #[fixed_width(size = 4)]
        first: String,
        #[fixed_width(size = 6, decimals = 3, pad_left = true, add_sign = true)]
        middle: BigDecimal,
        #[fixed_width(size = 4)]
        last: String,
    }

    let t = Test {
        first: "aaaa".to_string(),
        middle: BigDecimal::from_f64(123.456).unwrap(),
        last: "bbbb".to_string(),
    };
    let s: String = t.to_fixed_width_string().unwrap();
    println!("s: '{s}'");

    assert_eq!("aaaa12346+bbbb".to_string(), s);
}
