//! Utilities for finding nice, user-facing hints for function-not-found errors.
//!
//! Rhai, by default, simply says "function not found", if a functions arguments are bad.
//! This is because functions are looked up by the full signature, including argument types,
//! So for rhai, a function being callded with an i32 argument is completely unrelated to the same function name but defined with a &str argument.
//!
//! For those function not found errors, we look through all existing function signatures within the engine,
//! and try to find functions with the same name but different signatures,
//! and use those to show a nice error to the user.
//!
//! In case the function name is completely unknown, we also try to find similar function names, and suggest those to the user.
use std::collections::BTreeSet;

use rhai::Engine;

const MAX_SUGGESTIONS: usize = 3;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MissingFunctionCall {
    pub name: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FunctionCallHint {
    SignatureMismatch {
        call: MissingFunctionCall,
        signatures: Vec<String>,
    },
    UnknownFunction {
        call: MissingFunctionCall,
        suggestions: Vec<String>,
    },
}

impl FunctionCallHint {
    pub fn message(&self) -> String {
        match self {
            FunctionCallHint::SignatureMismatch { call, .. } => format!(
                "Function `{}` exists, but no overload accepts arguments: {}",
                call.name,
                format_args(&call.args)
            ),
            FunctionCallHint::UnknownFunction { call, suggestions } if suggestions.is_empty() => {
                format!("Unknown function `{}`", call.name)
            }
            FunctionCallHint::UnknownFunction { call, suggestions } => {
                format!(
                    "Unknown function `{}`. Did you mean `{}`?",
                    call.name, suggestions[0]
                )
            }
        }
    }

    pub fn help(&self) -> Option<String> {
        match self {
            FunctionCallHint::SignatureMismatch { signatures, .. } if !signatures.is_empty() => {
                Some(format!(
                    "Available overloads:\n{}",
                    signatures
                        .iter()
                        .map(|signature| format!("  - {signature}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                ))
            }
            FunctionCallHint::UnknownFunction { suggestions, .. } if !suggestions.is_empty() => {
                Some(format!(
                    "Similar functions: {}",
                    suggestions
                        .iter()
                        .map(|suggestion| format!("`{suggestion}`"))
                        .collect::<Vec<_>>()
                        .join(", ")
                ))
            }
            _ => None,
        }
    }
}

/// Build a user-facing hint for Rhai's `ErrorFunctionNotFound` signature string.
///
/// Rhai reports the attempted call as text, while registered candidates come
/// from metadata, so we parse this string only on the error path.
pub fn hint_for_function_not_found(engine: &Engine, signature: &str) -> FunctionCallHint {
    let call = parse_missing_function_call(signature);
    let signatures = engine.gen_fn_signatures(true);
    hint_from_signatures(call, signatures)
}

fn hint_from_signatures(
    call: MissingFunctionCall,
    signatures: impl IntoIterator<Item = String>,
) -> FunctionCallHint {
    let mut exact_signatures = Vec::new();
    let mut names = BTreeSet::new();

    for signature in signatures {
        let Some(function_name) = signature_name(&signature) else {
            continue;
        };
        names.insert(function_name.to_string());
        if function_name == call.name {
            exact_signatures.push(signature);
        }
    }

    if !exact_signatures.is_empty() {
        exact_signatures.sort();
        exact_signatures.dedup();
        return FunctionCallHint::SignatureMismatch {
            call,
            signatures: exact_signatures,
        };
    }

    let suggestions = similar_names(&call.name, names);
    FunctionCallHint::UnknownFunction { call, suggestions }
}

fn similar_names(name: &str, names: BTreeSet<String>) -> Vec<String> {
    let threshold = (name.chars().count() / 3).clamp(1, 3);
    let mut suggestions = names
        .into_iter()
        .filter_map(|candidate| {
            let distance = levenshtein(name, &candidate);
            (distance <= threshold).then_some((distance, candidate))
        })
        .collect::<Vec<_>>();
    suggestions.sort_by(|(left_distance, left), (right_distance, right)| {
        left_distance.cmp(right_distance).then(left.cmp(right))
    });
    suggestions
        .into_iter()
        .map(|(_, candidate)| candidate)
        .take(MAX_SUGGESTIONS)
        .collect()
}

/// Parse Rhai's attempted call signature, e.g. `foo(i64, string)`.
///
/// `ErrorFunctionNotFound` stores the failed call in this display-oriented
/// form, so extracting the name lets us distinguish typos from bad arguments.
pub fn parse_missing_function_call(signature: &str) -> MissingFunctionCall {
    let signature = signature.trim();
    let Some(open) = signature.find('(') else {
        return MissingFunctionCall {
            name: signature.to_string(),
            args: Vec::new(),
        };
    };
    let close = signature.rfind(')').unwrap_or(signature.len());
    let args = signature[open + 1..close]
        .split(',')
        .map(str::trim)
        .filter(|arg| !arg.is_empty())
        .map(ToOwned::to_owned)
        .collect();
    MissingFunctionCall {
        name: signature[..open].trim().to_string(),
        args,
    }
}

/// Extract the function name from a metadata signature such as `foo(x: i64)`.
fn signature_name(signature: &str) -> Option<&str> {
    signature.split_once('(').map(|(name, _)| name.trim())
}

fn format_args(args: &[String]) -> String {
    if args.is_empty() {
        "none".to_string()
    } else {
        args.join(", ")
    }
}

fn levenshtein(left: &str, right: &str) -> usize {
    if left == right {
        return 0;
    }
    if left.is_empty() {
        return right.chars().count();
    }
    if right.is_empty() {
        return left.chars().count();
    }

    let right_chars = right.chars().collect::<Vec<_>>();
    let mut previous = (0..=right_chars.len()).collect::<Vec<_>>();
    let mut current = vec![0; right_chars.len() + 1];

    for (left_index, left_char) in left.chars().enumerate() {
        current[0] = left_index + 1;
        for (right_index, right_char) in right_chars.iter().enumerate() {
            let substitution_cost = usize::from(left_char != *right_char);
            current[right_index + 1] = (previous[right_index + 1] + 1)
                .min(current[right_index] + 1)
                .min(previous[right_index] + substitution_cost);
        }
        std::mem::swap(&mut previous, &mut current);
    }

    previous[right_chars.len()]
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use super::*;

    #[test]
    fn parses_missing_function_signature() {
        assert_eq!(
            parse_missing_function_call("add_param_setting (&str, i64)"),
            MissingFunctionCall {
                name: "add_param_setting".to_string(),
                args: vec!["&str".to_string(), "i64".to_string(),],
            }
        );
    }

    #[test]
    fn classifies_known_name_as_signature_mismatch() {
        let hint = hint_from_signatures(
            parse_missing_function_call("path_exists(i64)"),
            ["path_exists(p: &str) -> Result<bool>".to_string()],
        );

        assert_matches!(
            hint,
            FunctionCallHint::SignatureMismatch { signatures, .. }
                if signatures == vec!["path_exists(p: &str) -> Result<bool>".to_string()]
        );
    }

    #[test]
    fn suggests_similarly_named_functions() {
        let hint = hint_from_signatures(
            parse_missing_function_call("path_exits(&str)"),
            [
                "path_exists(p: &str) -> Result<bool>".to_string(),
                "read_file(p: &str) -> Result<String>".to_string(),
            ],
        );

        assert_matches!(
            hint,
            FunctionCallHint::UnknownFunction { suggestions, .. }
                if suggestions == vec!["path_exists".to_string()]
        );
    }

    #[test]
    fn omits_distant_suggestions() {
        let hint = hint_from_signatures(
            parse_missing_function_call("frobnicate(&str)"),
            ["path_exists(p: &str) -> Result<bool>".to_string()],
        );

        assert_matches!(
            hint,
            FunctionCallHint::UnknownFunction { suggestions, .. } if suggestions.is_empty()
        );
    }
}
