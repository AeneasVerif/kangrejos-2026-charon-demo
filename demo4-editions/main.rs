#!/usr/bin/env -S cargo -Zscript
---
[package]
edition = "2024"

[dependencies]
anyhow = "1"
charon = {
    git = "https://github.com/AeneasVerif/charon",
    rev = "779d2657719174a57fc8e76b8e2c917565f01be1",
    default-features = false
}
derive_generic_visitor = "1.1"
similar = "2"
---

use anyhow::{bail, Context, Result};
use similar::TextDiff;
use std::collections::{BTreeMap, BTreeSet};
use std::ops::ControlFlow::{self, Continue};
use std::path::Path;

use charon_lib::errors::display_spanned_error;
use charon_lib::formatter::{AstFormatter, IntoFormatter};
use charon_lib::llbc_ast::*;
use charon_lib::options::SerializationFormat;
use charon_lib::pretty::FmtWithCtx;

// Visitor that compares two ast nodes ignoring storage statements and body lifetimes.
struct BodyComparer;

impl Visitor for BodyComparer {
    type Break = ();
}

impl<'a, T: AstVisitable> derive_generic_visitor::VisitTwo<'a, T> for BodyComparer {
    fn visit(&mut self, left: &'a T, right: &'a T) -> ControlFlow<Self::Break> {
        ZipAst::visit(self, left, right)
    }
}

impl ZipAst for BodyComparer {
    fn visit_span(&mut self, _left: &Span, _right: &Span) -> ControlFlow<Self::Break> {
        Continue(())
    }

    fn visit_llbc_block(&mut self, left: &Block, right: &Block) -> ControlFlow<Self::Break> {
        let is_not_storage = |statement: &&Statement| {
            !matches!(
                statement.kind,
                StatementKind::StorageLive(_) | StatementKind::StorageDead(_)
            )
        };

        derive_generic_visitor::drive_iter_two(
            left.statements.iter().filter(is_not_storage),
            right.statements.iter().filter(is_not_storage),
            self,
        )
    }

    fn visit_region(&mut self, left: &Region, right: &Region) -> ControlFlow<Self::Break> {
        match (left, right) {
            (Region::Body(_), Region::Body(_)) => Continue(()),
            _ => self.visit_inner(left, right),
        }
    }
}

fn load(path: &Path) -> Result<TranslatedCrate> {
    charon_lib::deserialize_llbc_with_format(path, SerializationFormat::Postcard)
        .with_context(|| format!("failed to load `{}`", path.display()))
}

fn functions_by_name(krate: &TranslatedCrate) -> Result<BTreeMap<String, &FunDecl>> {
    let mut functions = BTreeMap::new();
    for function in &krate.fun_decls {
        let name = function
            .item_meta
            .name
            .to_string_with_ctx(&krate.into_fmt());
        anyhow::ensure!(
            functions.insert(name.clone(), function).is_none(),
            "multiple functions pretty-print as `{name}`",
        );
    }
    Ok(functions)
}

fn main() -> Result<()> {
    let mut arguments = std::env::args_os().skip(1);
    let path_2021 = arguments
        .next()
        .context("usage: main.rs <edition-2021.llbc.postcard> <edition-2024.llbc.postcard>")?;
    let path_2024 = arguments
        .next()
        .context("usage: main.rs <edition-2021.llbc.postcard> <edition-2024.llbc.postcard>")?;
    anyhow::ensure!(
        arguments.next().is_none(),
        "usage: main.rs <edition-2021.llbc.postcard> <edition-2024.llbc.postcard>",
    );

    let edition_2021 = load(Path::new(&path_2021))?;
    let edition_2024 = load(Path::new(&path_2024))?;
    let functions_2021 = functions_by_name(&edition_2021)?;
    let functions_2024 = functions_by_name(&edition_2024)?;
    let all_function_names: BTreeSet<_> =
        functions_2021.keys().chain(functions_2024.keys()).collect();
    let pretty_body = |krate: &TranslatedCrate, function: &FunDecl| {
        let mut crate_fmt = krate.into_fmt();
        crate_fmt.hide_storage_statements = true;
        let fmt = crate_fmt.set_generics(&function.generics);
        format!("{}\n", function.body.with_ctx(&fmt))
    };

    let mut difference_count = 0;
    for name in all_function_names {
        let function_2021 = functions_2021.get(name);
        let function_2024 = functions_2024.get(name);
        let bodies_match = match (function_2021, function_2024) {
            (Some(left), Some(right)) => left
                .body
                .drive_two(&right.body, &mut BodyComparer)
                .is_continue(),
            _ => false,
        };
        if bodies_match {
            continue;
        }

        difference_count += 1;
        let body_2021 = match function_2021 {
            Some(function) => pretty_body(&edition_2021, function),
            None => "<function absent in edition 2021>\n".to_owned(),
        };
        let body_2024 = match function_2024 {
            Some(function) => pretty_body(&edition_2024, function),
            None => "<function absent in edition 2024>\n".to_owned(),
        };
        let diff = TextDiff::from_lines(&body_2021, &body_2024)
            .unified_diff()
            .header("edition 2021", "edition 2024")
            .to_string();
        let (krate, span) = match (function_2021, function_2024) {
            (Some(function), _) => (&edition_2021, function.item_meta.span),
            (None, Some(function)) => (&edition_2024, function.item_meta.span),
            (None, None) => unreachable!(),
        };
        display_spanned_error(
            krate,
            span,
            &format!("function `{name}` differs between Rust editions"),
            "this function has edition-dependent behavior",
        );
        eprintln!("{diff}");
    }

    if difference_count == 0 {
        println!("All function bodies are identical in editions 2021 and 2024.");
        Ok(())
    } else {
        bail!("found {difference_count} edition-dependent function body/bodies")
    }
}
