use ra_syntax::{
    algo::non_trivia_sibling,
    ast::{self, AstNode},
    Direction, T,
};

use crate::{Assist, AssistCtx, AssistId};

// Assist: flip_trait_bound
//
// Flips two trait bounds.
//
// ```
// fn foo<T: Clone +<|> Copy>() { }
// ```
// ->
// ```
// fn foo<T: Copy + Clone>() { }
// ```
pub(crate) fn flip_trait_bound(ctx: AssistCtx) -> Option<Assist> {
    // We want to replicate the behavior of `flip_binexpr` by only suggesting
    // the assist when the cursor is on a `+`
    let plus = ctx.find_token_at_offset(T![+])?;

    // Make sure we're in a `TypeBoundList`
    if ast::TypeBoundList::cast(plus.parent()).is_none() {
        return None;
    }

    let (before, after) = (
        non_trivia_sibling(plus.clone().into(), Direction::Prev)?,
        non_trivia_sibling(plus.clone().into(), Direction::Next)?,
    );

    ctx.add_assist(AssistId("flip_trait_bound"), "Flip trait bounds", |edit| {
        edit.target(plus.text_range());
        edit.replace(before.text_range(), after.to_string());
        edit.replace(after.text_range(), before.to_string());
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::helpers::{check_assist, check_assist_not_applicable, check_assist_target};

    #[test]
    fn flip_trait_bound_assist_available() {
        check_assist_target(flip_trait_bound, "struct S<T> where T: A <|>+ B + C { }", "+")
    }

    #[test]
    fn flip_trait_bound_not_applicable_for_single_trait_bound() {
        check_assist_not_applicable(flip_trait_bound, "struct S<T> where T: <|>A { }")
    }

    #[test]
    fn flip_trait_bound_works_for_struct() {
        check_assist(
            flip_trait_bound,
            "struct S<T> where T: A <|>+ B { }",
            "struct S<T> where T: B <|>+ A { }",
        )
    }

    #[test]
    fn flip_trait_bound_works_for_trait_impl() {
        check_assist(
            flip_trait_bound,
            "impl X for S<T> where T: A +<|> B { }",
            "impl X for S<T> where T: B +<|> A { }",
        )
    }

    #[test]
    fn flip_trait_bound_works_for_fn() {
        check_assist(flip_trait_bound, "fn f<T: A <|>+ B>(t: T) { }", "fn f<T: B <|>+ A>(t: T) { }")
    }

    #[test]
    fn flip_trait_bound_works_for_fn_where_clause() {
        check_assist(
            flip_trait_bound,
            "fn f<T>(t: T) where T: A +<|> B { }",
            "fn f<T>(t: T) where T: B +<|> A { }",
        )
    }

    #[test]
    fn flip_trait_bound_works_for_lifetime() {
        check_assist(
            flip_trait_bound,
            "fn f<T>(t: T) where T: A <|>+ 'static { }",
            "fn f<T>(t: T) where T: 'static <|>+ A { }",
        )
    }

    #[test]
    fn flip_trait_bound_works_for_complex_bounds() {
        check_assist(
            flip_trait_bound,
            "struct S<T> where T: A<T> <|>+ b_mod::B<T> + C<T> { }",
            "struct S<T> where T: b_mod::B<T> <|>+ A<T> + C<T> { }",
        )
    }

    #[test]
    fn flip_trait_bound_works_for_long_bounds() {
        check_assist(
            flip_trait_bound,
            "struct S<T> where T: A + B + C + D + E + F +<|> G + H + I + J { }",
            "struct S<T> where T: A + B + C + D + E + G +<|> F + H + I + J { }",
        )
    }
}
