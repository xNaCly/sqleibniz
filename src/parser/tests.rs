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
        sqleibniz_instructions,
        expect: r"
-- @sqleibniz::expect lets skip this error
VACUUM 25;
EXPLAIN VACUUM;
        "=vec![Type::Keyword(Keyword::EXPLAIN)],
        expect_with_semicolons_in_comment: r"
-- @sqleibniz::expect lets skip this error;;;;;;;;
VACUUM 25;
EXPLAIN VACUUM;
        "=vec![Type::Keyword(Keyword::EXPLAIN)]
    }

    test_group_pass_assert! {
        sql_stmt_prefix,
        explain: r#"EXPLAIN VACUUM;"#=vec![Type::Keyword(Keyword::EXPLAIN)],
        explain_query_plan: r#"EXPLAIN QUERY PLAN VACUUM;"#=vec![Type::Keyword(Keyword::EXPLAIN)]
    }

    test_group_pass_assert! {
        vacuum,
        vacuum_first_path: r#"VACUUM;"#=vec![Type::Keyword(Keyword::VACUUM)],
        vacuum_second_path: r#"VACUUM schema_name;"#=vec![Type::Keyword(Keyword::VACUUM)],
        vacuum_third_path: r#"VACUUM INTO 'filename';"#=vec![Type::Keyword(Keyword::VACUUM)],
        vacuum_full_path: r#"VACUUM schema_name INTO 'filename';"#=vec![Type::Keyword(Keyword::VACUUM)]
    }

    test_group_pass_assert! {
        begin_stmt,
        begin: r#"BEGIN;"#=vec![Type::Keyword(Keyword::BEGIN)],
        begin_transaction: r#"BEGIN TRANSACTION;"#=vec![Type::Keyword(Keyword::BEGIN)],
        begin_deferred: r#"BEGIN DEFERRED;"#=vec![Type::Keyword(Keyword::BEGIN)],
        begin_immediate: r#"BEGIN IMMEDIATE;"#=vec![Type::Keyword(Keyword::BEGIN)],
        begin_exclusive: r#"BEGIN EXCLUSIVE;"#=vec![Type::Keyword(Keyword::BEGIN)],

        begin_deferred_transaction: r"BEGIN DEFERRED TRANSACTION;"=vec![Type::Keyword(Keyword::BEGIN)],
        begin_immediate_transaction: r"BEGIN IMMEDIATE TRANSACTION;"=vec![Type::Keyword(Keyword::BEGIN)],
        begin_exclusive_transaction: r"BEGIN EXCLUSIVE TRANSACTION;"=vec![Type::Keyword(Keyword::BEGIN)]
    }

    test_group_pass_assert! {
        commit_stmt,
        commit:            r"COMMIT;"=vec![Type::Keyword(Keyword::COMMIT)],
        end:               r"END;"=vec![Type::Keyword(Keyword::END)],
        commit_transaction:r"COMMIT TRANSACTION;"=vec![Type::Keyword(Keyword::COMMIT)],
        end_transaction:   r"END TRANSACTION;"=vec![Type::Keyword(Keyword::END)]
    }

    test_group_pass_assert! {
        rollback_stmt,

        // rollback:r"ROLLBACK;"=vec![Type::Keyword(Keyword::ROLLBACK)],
        // rollback_to_save_point:r"ROLLBACK TO save_point;"=vec![Type::Keyword(Keyword::ROLLBACK)],
        // rollback_to_savepoint_save_point:r"ROLLBACK TO SAVEPOINT save_point;"=vec![Type::Keyword(Keyword::ROLLBACK)],
        // rollback_transaction:r"ROLLBACK TRANSACTION;"=vec![Type::Keyword(Keyword::ROLLBACK)],
        // rollback_transaction_to_save_point:r"ROLLBACK TRANSACTION TO save_point;"=vec![Type::Keyword(Keyword::ROLLBACK)],
        // rollback_transaction_to_savepoint_save_point:r"ROLLBACK TRANSACTION TO SAVEPOINT save_point;"=vec![Type::Keyword(Keyword::ROLLBACK)]
    }
}

#[cfg(test)]
mod should_fail {
    test_group_fail! {
        edge_cases,
        eof_semi: ";",
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

    test_group_fail! {
        sql_begin,
        begin_no_semicolon: r#"BEGIN"#,
        begin_transaction_no_semicolon: r#"BEGIN TRANSACTION"#,
        begin_deferred_no_semicolon: r#"BEGIN DEFERRED"#,
        begin_immediate_no_semicolon: r#"BEGIN IMMEDIATE"#,
        begin_exclusive_no_semicolon: r#"BEGIN EXCLUSIVE"#,

        begin_transaction_with_literal: r#"BEGIN TRANSACTION 25;"#,
        begin_transaction_with_other_keyword: r#"BEGIN TRANSACTION AS;"#,
        begin_too_many_modifiers: r#"BEGIN DEFERRED IMMEDIATE EXCLUSIVE EXCLUSIVE;"#

    }

    test_group_fail! {
        commit_stmt,
        commit_no_semicolon:            r"COMMIT",
        end_no_semicolon:               r"END",
        commit_transaction_no_semicolon:r"COMMIT TRANSACTION",
        end_transaction_no_semicolon:   r"END TRANSACTION",

        commit_with_literal:            r"COMMIT 25;",
        end_with_literal:               r"END 12;",
        commit_transaction_with_literal:r"COMMIT TRANSACTION x'81938912';",
        end_transaction_with_literal:   r"END TRANSACTION 'kadl';"
    }

    test_group_fail! {
        rollback_stmt,

        rollback_no_semicolon:r"ROLLBACK",
        rollback_to_save_point_no_semicolon:r"ROLLBACK TO save_point",
        rollback_to_savepoint_save_point_no_semicolon:r"ROLLBACK TO SAVEPOINT save_point",
        rollback_transaction_no_semicolon:r"ROLLBACK TRANSACTION",
        rollback_transaction_to_save_point_no_semicolon:r"ROLLBACK TRANSACTION TO save_point",
        rollback_transaction_to_savepoint_save_point_no_semicolon:r"ROLLBACK TRANSACTION TO SAVEPOINT save_point"
    }
}
