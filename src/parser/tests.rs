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

        rollback:r"ROLLBACK;"=vec![Type::Keyword(Keyword::ROLLBACK)],
        rollback_to_save_point:r"ROLLBACK TO save_point;"=vec![Type::Keyword(Keyword::ROLLBACK)],
        rollback_to_savepoint_save_point:r"ROLLBACK TO SAVEPOINT save_point;"=vec![Type::Keyword(Keyword::ROLLBACK)],
        rollback_transaction:r"ROLLBACK TRANSACTION;"=vec![Type::Keyword(Keyword::ROLLBACK)],
        rollback_transaction_to_save_point:r"ROLLBACK TRANSACTION TO save_point;"=vec![Type::Keyword(Keyword::ROLLBACK)],
        rollback_transaction_to_savepoint_save_point:r"ROLLBACK TRANSACTION TO SAVEPOINT save_point;"=vec![Type::Keyword(Keyword::ROLLBACK)]
    }

    test_group_pass_assert! {
        detach_stmt,

        detach_schema_name:r"DETACH schema_name;"=vec![Type::Keyword(Keyword::DETACH)],
        detach_database_schema_name:r"DETACH DATABASE schema_name;"=vec![Type::Keyword(Keyword::DETACH)]
    }

    test_group_pass_assert! {
        analyze_stmt,


        analyze:r"ANALYZE;"=vec![Type::Keyword(Keyword::ANALYZE)],
        analyze_schema_name:r"ANALYZE schema_name;"=vec![Type::Keyword(Keyword::ANALYZE)],
        analyze_index_or_table_name:r"ANALYZE index_or_table_name;"=vec![Type::Keyword(Keyword::ANALYZE)],
        analyze_schema_name_with_subtable:r"ANALYZE schema_name.index_or_table_name;"=vec![Type::Keyword(Keyword::ANALYZE)]
    }

    test_group_pass_assert! {
        drop_stmt,

        drop_index_index_name:r"DROP INDEX index_name;"=vec![Type::Keyword(Keyword::DROP)],
        drop_index_if_exists_schema_name_index_name:r"DROP INDEX IF EXISTS schema_name.index_name;"=vec![Type::Keyword(Keyword::DROP)],
        drop_table_table_name:r"DROP TABLE table_name;"=vec![Type::Keyword(Keyword::DROP)],
        drop_table_if_exists_schema_name_table_name:r"DROP TABLE IF EXISTS schema_name.table_name;"=vec![Type::Keyword(Keyword::DROP)],
        drop_trigger_trigger_name:r"DROP TRIGGER trigger_name;"=vec![Type::Keyword(Keyword::DROP)],
        drop_trigger_if_exists_schema_name_trigger_name:r"DROP TRIGGER IF EXISTS schema_name.trigger_name;"=vec![Type::Keyword(Keyword::DROP)],
        drop_view_view_name:r"DROP VIEW view_name;"=vec![Type::Keyword(Keyword::DROP)],
        drop_view_if_exists_schema_name_view_name:r"DROP VIEW IF EXISTS schema_name.view_name;"=vec![Type::Keyword(Keyword::DROP)]
    }

    test_group_pass_assert! {
        savepoint_stmt,

        savepoint_savepoint_name:r"SAVEPOINT savepoint_name;"=vec![Type::Keyword(Keyword::SAVEPOINT)]
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
        rollback_transaction_to_savepoint_save_point_no_semicolon:r"ROLLBACK TRANSACTION TO SAVEPOINT save_point",

        rollback_transaction_to_literal_save_point:r"ROLLBACK TRANSACTION TO SAVEPOINT 'hello';"
    }

    test_group_fail! {
        detach_stmt,

        detach_schema_name_no_semicolon:r"DETACH schema_name",
        detach_database_schema_name_no_semicolon:r"DETACH DATABASE schema_name",

        detach_schema_no_name:r"DETACH;",
        detach_database_no_schema_name:r"DETACH DATABASE;",
        detach_schema_literal_instead_of_name:r"DETACH 'this string should not be here';"
    }

    test_group_fail! {
        drop_stmt,

        drop_index_index_name_no_semicolon:r"DROP INDEX index_name",
        drop_index_if_exists_schema_name_index_name_no_semicolon:r"DROP INDEX IF EXISTS schema_name.index_name",
        drop_table_table_name_no_semicolon:r"DROP TABLE table_name",
        drop_table_if_exists_schema_name_table_name_no_semicolon:r"DROP TABLE IF EXISTS schema_name.table_name",
        drop_trigger_trigger_name_no_semicolon:r"DROP TRIGGER trigger_name",
        drop_trigger_if_exists_schema_name_trigger_name_no_semicolon:r"DROP TRIGGER IF EXISTS schema_name.trigger_name",
        drop_view_view_name_no_semicolon:r"DROP VIEW view_name",
        drop_view_if_exists_schema_name_view_name_no_semicolon:r"DROP VIEW IF EXISTS schema_name.view_name",

        drop_index_no_index_name:r"DROP INDEX;",
        drop_table_no_table_name:r"DROP TABLE;",
        drop_trigger_no_trigger_name:r"DROP TRIGGER;",
        drop_view_no_view_name:r"DROP VIEW;"
    }

    test_group_fail! {
        savepoint_stmt,

        savepoint_savepoint_name_no_semicolon:r"SAVEPOINT savepoint_name",

        savepoint_no_savepoint_name:r"SAVEPOINT;"
    }
}
