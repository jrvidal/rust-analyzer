//! Completion for attributes
//!
//! This module uses a bit of static metadata to provide completions
//! for built-in attributes.

use super::completion_context::CompletionContext;
use super::completion_item::{CompletionItem, CompletionItemKind, CompletionKind, Completions};
use ra_syntax::{ast, AstNode};

const ATTRIBUTES: &[AttrCompletion] = &[
    AttrCompletion { label: "allow", snippet: Some("allow(${0:lint})"), should_be_inner: false },
    AttrCompletion {
        label: "cfg_attr",
        snippet: Some("cfg_attr(${1:flag}, ${0:attr})"),
        should_be_inner: false,
    },
    AttrCompletion { label: "cfg", snippet: Some("cfg(${0:flag})"), should_be_inner: false },
    AttrCompletion { label: "deny", snippet: Some("deny(${0:lint})"), should_be_inner: false },
    AttrCompletion {
        label: "deprecated",
        snippet: Some(r#"deprecated = "${0:warning}""#),
        should_be_inner: false,
    },
    AttrCompletion {
        label: "derive",
        snippet: Some(r#"derive(${0:Debug})"#),
        should_be_inner: false,
    },
    AttrCompletion { label: "doc", snippet: Some(r#"doc = "${0:docs}""#), should_be_inner: false },
    AttrCompletion { label: "feature", snippet: Some("feature(${0:flag})"), should_be_inner: true },
    AttrCompletion { label: "global_allocator", snippet: None, should_be_inner: true },
    AttrCompletion { label: "ignore", snippet: Some("ignore(${0:lint})"), should_be_inner: false },
    AttrCompletion { label: "inline", snippet: Some("inline(${0:lint})"), should_be_inner: false },
    AttrCompletion {
        label: "link_name",
        snippet: Some(r#"link_name = "${0:symbol_name}""#),
        should_be_inner: false,
    },
    AttrCompletion { label: "link", snippet: None, should_be_inner: false },
    AttrCompletion { label: "macro_export", snippet: None, should_be_inner: false },
    AttrCompletion { label: "macro_use", snippet: None, should_be_inner: false },
    AttrCompletion {
        label: "must_use",
        snippet: Some(r#"must_use = "${0:message}""#),
        should_be_inner: false,
    },
    AttrCompletion { label: "no_mangle", snippet: None, should_be_inner: false },
    AttrCompletion { label: "no_std", snippet: None, should_be_inner: true },
    AttrCompletion { label: "non_exhaustive", snippet: None, should_be_inner: false },
    AttrCompletion { label: "panic_handler", snippet: None, should_be_inner: true },
    AttrCompletion { label: "path", snippet: Some("path =\"${0:path}\""), should_be_inner: false },
    AttrCompletion {
        label: "recursion_limit",
        snippet: Some("recursion_limit = ${0:128}"),
        should_be_inner: true,
    },
    AttrCompletion { label: "repr", snippet: Some("repr(${0:Rust})"), should_be_inner: false },
    AttrCompletion {
        label: "should_panic",
        snippet: Some(r#"expected = "${0:message}""#),
        should_be_inner: false,
    },
    AttrCompletion {
        label: "target_feature",
        snippet: Some("target_feature = \"${0:feature}\""),
        should_be_inner: false,
    },
    AttrCompletion { label: "test", snippet: None, should_be_inner: false },
    AttrCompletion { label: "used", snippet: None, should_be_inner: false },
    AttrCompletion { label: "warn", snippet: Some("warn(${0:lint})"), should_be_inner: false },
    AttrCompletion {
        label: "windows_subsystem",
        snippet: Some(r#"windows_subsystem = "${0:subsystem}""#),
        should_be_inner: true,
    },
];

struct AttrCompletion {
    label: &'static str,
    snippet: Option<&'static str>,
    should_be_inner: bool,
}

pub(super) fn complete_attribute(acc: &mut Completions, ctx: &CompletionContext) {
    if !ctx.is_attribute {
        return;
    }

    let is_inner = ctx
        .name_ref_syntax
        .as_ref()
        .into_iter()
        .flat_map(|name_ref| name_ref.syntax().ancestors())
        .filter_map(ast::Attr::cast)
        .next()
        .and_then(|attr| attr.excl_token())
        .is_some();

    for attr_completion in ATTRIBUTES {
        let mut item =
            CompletionItem::new(CompletionKind::Postfix, ctx.source_range(), attr_completion.label)
                .kind(CompletionItemKind::Attribute);
        if let Some(snippet) = attr_completion.snippet {
            item = item.insert_snippet(snippet);
        }
        if is_inner || !attr_completion.should_be_inner {
            acc.add(item);
        }
    }
}
