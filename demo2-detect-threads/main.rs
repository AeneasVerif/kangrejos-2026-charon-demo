#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"

[dependencies]
anyhow = "1"
charon = { git = "https://github.com/AeneasVerif/charon", default-features = false }
---

//! This example implements a small linter that detects whether a crate uses threads by catching
//! calls to `thread::spawn`.
//!
//! This is a demo example: it doesn't capture other ways of spawning threads, or even using
//! `thread::spawn` using function pointers.
use anyhow::{Context, Result};

use charon_lib::errors::display_spanned_error;
use charon_lib::llbc_ast::*;
use charon_lib::name_matcher::NamePattern;

fn main() -> Result<()> {
    let llbc_path = std::env::args_os()
        .nth(1)
        .context("usage: main.rs <crate.llbc>")?;
    let krate: TranslatedCrate = charon_lib::deserialize_llbc(llbc_path.as_ref())?;

    // A pattern that detects the `thread::spawn` function we care about.
    let thread_spawn = NamePattern::parse("std::thread::_::spawn").unwrap();

    // Iterate over all the functions in the crate (including dependencies) to find calls to
    // `thread::spawn`.
    let mut calls_count = 0;
    for function in &krate.fun_decls {
        // If that function has a body we can inspect.
        if let Body::Structured(body) = &function.body {
            let caller = function.item_meta.name.debug_repr(&krate);
            // Iterate through all the statements of this function's body.
            body.body
                .dyn_visit_in_body::<Statement>(|statement: &Statement| {
                    // Find calls...
                    if let StatementKind::Call { call, .. } = &statement.kind
                    // statically known (as opposed to indirect/via a function pointer)
                    && let FnOperand::Regular(callee) = &call.func
                    // to a known function (as opposed to a trait method)
                    && let FnPtrKind::Fun(callee) = callee.kind.as_ref()
                    // that has the right name
                    && thread_spawn.matches(&krate, krate.item_name(*callee))
                    {
                        calls_count += 1;
                        display_spanned_error(
                            &krate,
                            statement.span,
                            "spawning threads is forbidden",
                            &format!("`{caller}` spawns a thread here"),
                        );
                    }
                });
        }
    }

    if calls_count == 0 {
        println!("No calls to std::thread::spawn found.");
    } else {
        println!("Found {calls_count} thread-spawn violation(s).");
    }

    Ok(())
}
