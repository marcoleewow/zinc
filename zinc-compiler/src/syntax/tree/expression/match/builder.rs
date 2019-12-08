//!
//! The match expression builder.
//!

use crate::lexical::Location;
use crate::syntax::Expression;
use crate::syntax::MatchExpression;
use crate::syntax::Pattern;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    scrutinee: Option<Expression>,
    branches: Vec<(Pattern, Option<Expression>)>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_scrutinee(&mut self, value: Expression) {
        self.scrutinee = Some(value);
    }

    pub fn push_branch_pattern(&mut self, value: Pattern) {
        self.branches.push((value, None));
    }

    pub fn set_branch_expression(&mut self, value: Expression) {
        self.branches
            .last_mut()
            .unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                    "branch expression"
                )
            })
            .1 = Some(value);
    }

    pub fn finish(self) -> MatchExpression {
        MatchExpression::new(
            self.location.unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                    "location"
                )
            }),
            self.scrutinee.unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                    "scrutinee"
                )
            }),
            self.branches
                .into_iter()
                .map(|(pattern, expression)| {
                    (
                        pattern,
                        expression.unwrap_or_else(|| {
                            panic!(
                                "{}{}",
                                crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                                "branch expression"
                            )
                        }),
                    )
                })
                .collect::<Vec<(Pattern, Expression)>>(),
        )
    }
}
