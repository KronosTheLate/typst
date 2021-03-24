use super::*;
use crate::layout::SpacingNode;

/// `h`: Horizontal spacing.
///
/// # Positional parameters
/// - Amount of spacing: of type `linear` relative to current font size.
///
/// # Return value
/// A template that inserts horizontal spacing.
pub fn h(ctx: &mut EvalContext, args: &mut FuncArgs) -> Value {
    spacing_impl(ctx, args, SpecAxis::Horizontal)
}

/// `v`: Vertical spacing.
///
/// # Positional parameters
/// - Amount of spacing: of type `linear` relative to current font size.
///
/// # Return value
/// A template that inserts vertical spacing.
pub fn v(ctx: &mut EvalContext, args: &mut FuncArgs) -> Value {
    spacing_impl(ctx, args, SpecAxis::Vertical)
}

fn spacing_impl(ctx: &mut EvalContext, args: &mut FuncArgs, axis: SpecAxis) -> Value {
    let spacing: Option<Linear> = args.require(ctx, "spacing");
    Value::template("spacing", move |ctx| {
        if let Some(linear) = spacing {
            let amount = linear.resolve(ctx.state.font.resolve_size());
            let spacing = SpacingNode { amount, softness: 0 };
            if axis == ctx.state.dirs.main.axis() {
                ctx.push_into_stack(spacing);
            } else {
                ctx.push(spacing);
            }
        }
    })
}