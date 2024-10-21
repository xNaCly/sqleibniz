#[allow(unused_macros)]
macro_rules! test_group {
    ("fail",$group_name:ident,$($name:ident:$value:literal),*) => {
        mod $group_name {
        use crate::lexer;
        $(
            #[test]
            fn $name() {
                let source = $value.as_bytes().to_vec();
                let mut l = lexer::Lexer::new(&source, "lexer_tests_fail");
                let toks = l.run();
                assert_eq!(toks.len(), 0);
                assert_ne!(l.errors.len(), 0);
            }
         )*
        }
    };
    ("pass",$group_name:ident,$($name:ident:$value:literal=$expected:expr),*) => {
        mod $group_name {
        use crate::lexer;
        use crate::types::Type;
        $(
            #[test]
            fn $name() {
                let source = $value.as_bytes().to_vec();
                let mut l = lexer::Lexer::new(&source, "lexer_tests_pass");
                let toks = l.run();
                assert_ne!(toks.len(), 0);
                assert_eq!(l.errors.len(), 0);
                assert_eq!(toks[0].ttype, $expected);
            }
         )*
        }
    };
}

#[cfg(test)]
mod should_pass {
    test_group! {
        "pass",
        booleans,
        r#true: "true"=Type::Boolean(true),
        true_upper: "TRUE"=Type::Boolean(true),
        r#false: "false"=Type::Boolean(false),
        false_upper: "FALSE"=Type::Boolean(false)
    }

    test_group! {
        "pass",
        string,
        string: "'text'"=Type::String(String::from("text")),
        empty_string: "''"=Type::String(String::from(""))
    }

    test_group! {
        "pass",
        symbol,
        // d is needed, because the lexer interprets . as a float start if the next character is
        // not an identifier, if so, it detects Type::Dot
        dot: ".d"=Type::Dot,
        star: "*"=Type::Asteriks,
        semicolon: ";"=Type::Semicolon,
        comma: ","=Type::Comma,
        percent: "%"=Type::Percent
    }

    test_group! {
        "pass",
        number,
        // edge cases
        zero: "0"=Type::Number(0.0),
        zero_float: ".0"=Type::Number(0.0),
        zero_hex: "0x0"=Type::Number(0.0),
        zero_float_with_prefix_zero: "0.0"=Type::Number(0.0),

        float_all_paths: "1_000.12_000e+3_5"=Type::Number(1.00012e+38),
        float_all_paths2: ".1_000e-1_2"=Type::Number(1e-13),
        hex: "0xABCDEF"=Type::Number(0xABCDEF as f64),
        hex_large_x: "0XABCDEF"=Type::Number(0xABCDEF as f64)
    }

    test_group! {
        "pass",
        blob,
        // edge cases
        empty: "X''"=Type::Blob(vec![]),
        empty_small: "x''"=Type::Blob(vec![]),

        filled: "X'12345'"=Type::Blob(vec![49, 50, 51, 52, 53]),
        filled_small: "x'1234567'"=Type::Blob(vec![49, 50, 51, 52, 53, 54, 55])
    }
}

#[cfg(test)]
mod should_fail {
    test_group! {
        "fail",
        empty_input,
        empty: "",
        empty_with_escaped: "\\",
        empty_with_space: " \t\n\r"
    }

    test_group! {
        "fail",
        string,
        unterminated_string_eof: "'",
        unterminated_string_with_space: "'\n\t\r\n "
    }

    test_group! {
        "fail",
        comment,
        line_comment: "-- comment",
        multiline_comment_single_line: "/**/",
        multiline_comment: "/*\n\n\n*/"
    }

    test_group! {
        "fail",
        number,
        bad_hex: "0x",
        bad_hex2: "0X",
        bad_float: ".",
        bad_float_multiple_dots: "....",
        bad_float_with_e: ".e",
        bad_float_with_large_e: ".E",
        bad_float_multiple_e: ".eeee",
        bad_float_combination: "12.e+-15"
    }

    test_group! {
        "fail",
        blob,
        // edge cases
        no_quotes: "X",
        no_quotes_small: "x",
        unterminated: "X'",
        unterminated_small: "x'",
        unterminated1: "X'12819281",
        unterminated_small1: "x'102812",
        bad_hex: "X'1281928FFFY'"
    }
}
