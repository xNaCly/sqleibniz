#[allow(unused_macros)]
macro_rules! test_group_pass_assert {
    ($group_name:ident,$($ident:ident:$input:literal=$expected:expr),*) => {
    mod $group_name {
        use crate::{lexer, parser::Parser, types::Type, types::Keyword};

        $(
            #[test]
            fn $ident() {
                let input = $input.as_bytes().to_vec();
                let mut l = lexer::Lexer::new(&input, "parser_test_pass");
                let toks = l.run();
                assert_eq!(l.errors.len(), 0);

                let mut parser = Parser::new(toks, "parser_test_pass");
                let ast = parser.parse();
                assert_eq!(parser.errors.len(), 0);
                assert_eq!(ast.into_iter()
                    .map(|o| o.unwrap().token().ttype.clone())
                    .collect::<Vec<Type>>(), $expected);
            }
        )*
        }
    };
}

#[allow(unused_macros)]
macro_rules! test_group_fail {
    ($group_name:ident,$($ident:ident:$input:literal),*) => {
    mod $group_name {
        use crate::{lexer, parser::Parser};

        $(
            #[test]
            fn $ident() {
                let input = $input.as_bytes().to_vec();
                let mut l = lexer::Lexer::new(&input, "parser_test_fail");
                let toks = l.run();
                assert_eq!(l.errors.len(), 0);

                let mut parser = Parser::new(toks, "parser_test_fail");
                let _ = parser.parse();
                assert_ne!(parser.errors.len(), 0);
            }
        )*
        }
    };
}

#[cfg(test)]
mod should_pass {
    test_group_pass_assert! {
        literals,
        string: "'string1';"= vec![Type::String(String::from("string1"))],
        two_strings: "'string1';'str';"= vec![Type::String(String::from("string1")), Type::String(String::from("str"))],
        numba: r#"0x0;1;1.1;"#=vec![Type::Number(0.0), Type::Number(1.0), Type::Number(1.1)],
        blob: "x'123';X'1234';"=vec![Type::Blob(vec![49,50,51]), Type::Blob(vec![49,50,51,52])],
        null: "null;NULL;"=vec![Type::Keyword(Keyword::NULL), Type::Keyword(Keyword::NULL)],
        boolean: "true;TRUE;false;FALSE;"=vec![Type::Boolean(true), Type::Boolean(true), Type::Boolean(false), Type::Boolean(false)],
        keywords: "CURRENT_TIME;CURRENT_DATE;CURRENT_TIMESTAMP;"=vec![Type::Keyword(Keyword::CURRENT_TIME), Type::Keyword(Keyword::CURRENT_DATE), Type::Keyword(Keyword::CURRENT_TIMESTAMP)]
    }

    test_group_pass_assert! {
        sql_stmt_prefix,
        explain: r#"EXPLAIN NULL;"#=vec![Type::Keyword(Keyword::EXPLAIN)],
        explain_query_plan: r#"EXPLAIN QUERY PLAN NULL;"#=vec![Type::Keyword(Keyword::EXPLAIN)]
    }

    test_group_pass_assert! {
        vacuum,
        vacuum_first_path: r#"VACUUM;"#=vec![Type::Keyword(Keyword::VACUUM)],
        vacuum_second_path: r#"VACUUM schema_name;"#=vec![Type::Keyword(Keyword::VACUUM)],
        vacuum_third_path: r#"VACUUM INTO 'filename';"#=vec![Type::Keyword(Keyword::VACUUM)],
        vacuum_full_path: r#"VACUUM schema_name INTO 'filename';"#=vec![Type::Keyword(Keyword::VACUUM)]
    }
}

#[cfg(test)]
mod should_fail {
    test_group_fail! {
        edge_cases,
        eof_hit_string: "'str'",
        eof_hit_number: "0x0",
        eof_hit_blob: "x''",
        eof_hit_null: "NULL",
        eof_hit_boolean: "true",
        eof_hit_cur_time: "CURRENT_TIME",
        eof_hit_cur_date: "CURRENT_DATE",
        eof_hit_cur_timestamp: "CURRENT_TIMESTAMP"
    }

    test_group_fail! {
        sql_stmt_prefix,
        explain: r#"EXPLAIN;"#,
        explain_query_plan: r#"EXPLAIN QUERY PLAN;"#
    }

    test_group_fail! {
        sql_vacuum,
        vacuum_no_semicolon: r#"VACUUM"#,
        vacuum_invalid_schema: r#"VACUUM 1;"#,
        vacuum_invalid_filename: r#"VACUUM INTO 5;"#,
        vacuum_invalid_combined: r#"VACUUM 5 INTO 5;"#
    }
}
