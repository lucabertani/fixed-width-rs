use std::io::{BufRead, BufReader};

use fixed_width::FixedWidth;

#[test]
fn test_from_fixed() {
    #[derive(Debug, FixedWidth)]
    struct Test {
        #[fixed_width(size = 10)]
        name: String,
        #[fixed_width(size = 10)]
        description: String,
        #[fixed_width(size = 2, constant = "\r\n")]
        pub filler: String,
    }

    let t = Test {
        name: "Mario".to_string(),
        description: "Rossi".to_string(),
        filler: "\r\n".to_string(),
    };

    let s = t.to_fixed_width_string().unwrap();
    println!("s: '{s}'");

    // scrivi s su un file
    std::fs::write("test.txt", &s).unwrap();

    // rileggi riga per riga il file e verifica che contenuto è uguale a s
    let buf = BufReader::new(std::fs::File::open("test.txt").unwrap());
    let lines = buf.lines();

    for line in lines {
        let line = line.unwrap();
        let t2 = Test::from_fixed_width_string(&line).unwrap();
        assert_eq!(t2.name, "Mario");
        assert_eq!(t2.description, "Rossi");
        assert_eq!(t2.filler, "\r\n");
    }
}

#[test]
fn test_constant_field_read_without_newline() {
    #[derive(Debug, FixedWidth)]
    struct Test {
        #[fixed_width(size = 10)]
        name: String,
        #[fixed_width(size = 10)]
        description: String,
        #[fixed_width(size = 2, constant = "\r\n")]
        pub filler: String,
    }

    // Simulate what happens when a line reader strips the \r\n:
    // the input is 20 bytes instead of 22
    let input = format!("{:>10}{:>10}", "Mario", "Rossi");
    assert_eq!(input.len(), 20);

    let t = Test::from_fixed_width_string(&input).unwrap();
    assert_eq!(t.name, "Mario");
    assert_eq!(t.description, "Rossi");
    assert_eq!(t.filler, "\r\n");
}

#[test]
fn test_constant_option_string() {
    #[derive(Debug, FixedWidth)]
    struct Test {
        #[fixed_width(size = 10)]
        name: String,
        #[fixed_width(size = 10)]
        description: String,
        #[fixed_width(size = 2, constant = "\r\n")]
        pub filler: Option<String>,
    }

    // Write: constant is always written, even if field is None
    let t = Test {
        name: "Mario".to_string(),
        description: "Rossi".to_string(),
        filler: None,
    };

    let s = t.to_fixed_width_string().unwrap();
    assert_eq!(s.len(), 22);
    assert!(s.ends_with("\r\n"));

    // Read with full input (bytes present)
    let t2 = Test::from_fixed_width_string(&s).unwrap();
    assert_eq!(t2.name, "Mario");
    assert_eq!(t2.description, "Rossi");
    assert_eq!(t2.filler, Some("\r\n".to_string()));

    // Read with truncated input (line-reader stripped \r\n)
    let truncated = &s[..s.len() - 2];
    assert_eq!(truncated.len(), 20);
    let t3 = Test::from_fixed_width_string(truncated).unwrap();
    assert_eq!(t3.name, "Mario");
    assert_eq!(t3.description, "Rossi");
    assert_eq!(t3.filler, Some("\r\n".to_string()));
}
