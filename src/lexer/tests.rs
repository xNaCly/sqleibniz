#[allow(unused_macros)]
macro_rules! test_group {
    ("fail",$group_name:ident,$($name:ident:$value:literal),*) => {
        mod $group_name {
        use crate::lexer;
        $(
            #[test]
            fn $name() {
                let source = $value.as_bytes().to_vec();
                let mut l = lexer::Lexer::new(&source, String::from("lexer_tests_fail"));
                let toks = l.run();
                assert_eq!(toks.len(), 0);
                assert_ne!(l.errors.len(), 0);
            }
         )*
        }
    };
    ("pass",$group_name:ident,$($name:ident:$value:literal),*) => {
        mod $group_name {
        use crate::lexer;
        $(
            #[test]
            fn $name() {
                let source = $value.as_bytes().to_vec();
                let mut l = lexer::Lexer::new(&source, String::from("lexer_tests_pass"));
                let toks = l.run();
                assert_ne!(toks.len(), 0);
                assert_eq!(l.errors.len(), 0);
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
        r#true: "true",
        true_upper: "TRUE",
        r#false: "false",
        false_upper: "FALSE"
    }

    test_group! {
        "pass",
        string,
        string: "'text'",
        empty_string: "''"
    }

    test_group! {
        "pass",
        symbol,
        star: "* ",
        semicolon: "; ",
        comma: ", ",
        percent: "% "
    }

    test_group! {
        "pass",
        number,
        // edge cases
        zero: "0",
        zero_float: ".0",
        zero_hex: "0x0",
        zero_float_with_prefix_zero: "0.0",

        float_all_paths: "1_000.12_000e+3_5",
        float_all_paths2: ".1_000e-1_2",
        hex: "0xABCDEF",
        hex_large_x: "0XABCDEF"
    }

    test_group! {
        "pass",
        blob,
        // edge cases
        empty: "X''",
        empty_small: "x''",

        filled: "X'12345'",
        filled_small: "x'12345'"
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
