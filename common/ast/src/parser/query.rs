// Copyright 2022 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use nom::branch::alt;
use nom::combinator::consumed;
use nom::combinator::map;
use nom::combinator::value;

use crate::ast::*;
use crate::parser::expr::*;
use crate::parser::token::*;
use crate::parser::util::*;
use crate::rule;

pub fn query(i: Input) -> IResult<Query> {
    map(
        consumed(rule! {
            #set_expr
            ~ ( ORDER ~ ^BY ~ ^#comma_separated_list1(order_by_expr) )?
            ~ ( LIMIT ~ ^#comma_separated_list1(expr) )?
            ~ ( OFFSET ~ ^#expr )?
            ~ ( FORMAT ~ #ident )?
            : "`SELECT ...`"
        }),
        |(span, (body, opt_order_by_block, opt_limit_block, opt_offset_block, opt_format))| Query {
            span: span.0,
            body,
            order_by: opt_order_by_block
                .map(|(_, _, order_by)| order_by)
                .unwrap_or_default(),
            limit: opt_limit_block.map(|(_, limit)| limit).unwrap_or_default(),
            offset: opt_offset_block.map(|(_, offset)| offset),
            format: opt_format.map(|(_, format)| format.name),
        },
    )(i)
}

pub fn select_target(i: Input) -> IResult<SelectTarget> {
    let qualified_wildcard = map(
        rule! {
            ( #ident ~ "." ~ ( #ident ~ "." )? )? ~ "*"
        },
        |(res, _)| match res {
            Some((fst, _, Some((snd, _)))) => SelectTarget::QualifiedName(vec![
                Indirection::Identifier(fst),
                Indirection::Identifier(snd),
                Indirection::Star,
            ]),
            Some((fst, _, None)) => {
                SelectTarget::QualifiedName(vec![Indirection::Identifier(fst), Indirection::Star])
            }
            None => SelectTarget::QualifiedName(vec![Indirection::Star]),
        },
    );
    let projection = map(
        rule! {
            #expr ~ #alias_name?
        },
        |(expr, alias)| SelectTarget::AliasedExpr {
            expr: Box::new(expr),
            alias,
        },
    );

    rule!(
        #qualified_wildcard
        | #projection
    )(i)
}

pub fn table_reference(i: Input) -> IResult<TableReference> {
    rule!(
        #joined_tables
        | #parenthesized_joined_tables
        | #subquery
        | #table_function
        | #aliased_table
    )(i)
}

pub fn aliased_table(i: Input) -> IResult<TableReference> {
    map(
        rule! {
            #ident ~ ( "." ~ #ident )? ~ ( "." ~ #ident )? ~ #travel_point? ~ #table_alias?
        },
        |(fst, snd, third, travel_point, alias)| {
            let (catalog, database, table) = match (fst, snd, third) {
                (catalog, Some((_, database)), Some((_, table))) => {
                    (Some(catalog), Some(database), table)
                }
                (database, Some((_, table)), None) => (None, Some(database), table),
                (database, None, Some((_, table))) => (None, Some(database), table),
                (table, None, None) => (None, None, table),
            };

            TableReference::Table {
                catalog,
                database,
                table,
                alias,
                travel_point,
            }
        },
    )(i)
}

pub fn travel_point(i: Input) -> IResult<TimeTravelPoint> {
    map(
        rule! {
            AT ~ "(" ~ SNAPSHOT ~ "=>" ~ #literal_string ~ ")"
        },
        |(_, _, _, _, s, _)| TimeTravelPoint::Snapshot(s),
    )(i)
}

pub fn alias_name(i: Input) -> IResult<Identifier> {
    let as_alias = map(rule! { AS ~ #ident_after_as }, |(_, name)| name);

    rule!(
        #ident
        | #as_alias
    )(i)
}

pub fn table_alias(i: Input) -> IResult<TableAlias> {
    map(alias_name, |name| TableAlias {
        name,
        columns: vec![],
    })(i)
}

pub fn table_function(i: Input) -> IResult<TableReference> {
    map(
        rule! {
            #ident ~ "(" ~ #comma_separated_list0(expr) ~ ")" ~ #table_alias?
        },
        |(name, _, params, _, alias)| TableReference::TableFunction {
            name,
            params,
            alias,
        },
    )(i)
}

pub fn subquery(i: Input) -> IResult<TableReference> {
    map(
        rule! {
            ( #parenthesized_query | #query ) ~ #table_alias?
        },
        |(subquery, alias)| TableReference::Subquery {
            subquery: Box::new(subquery),
            alias,
        },
    )(i)
}

pub fn parenthesized_query(i: Input) -> IResult<Query> {
    map(
        rule! {
            "(" ~ ( #parenthesized_query | #query ) ~ ")"
        },
        |(_, query, _)| query,
    )(i)
}

pub fn parenthesized_joined_tables(i: Input) -> IResult<TableReference> {
    map(
        rule! {
            "(" ~ ( #parenthesized_joined_tables | #joined_tables ) ~ ")"
        },
        |(_, joined_tables, _)| joined_tables,
    )(i)
}

pub fn joined_tables(i: Input) -> IResult<TableReference> {
    struct JoinElement<'a> {
        op: JoinOperator,
        condition: JoinCondition<'a>,
        right: Box<TableReference<'a>>,
    }

    let table_ref_without_join = |i| {
        rule!(
            #parenthesized_joined_tables
            | #subquery
            | #table_function
            | #aliased_table
        )(i)
    };
    let join_condition_on = map(
        rule! {
            ON ~ #expr
        },
        |(_, expr)| JoinCondition::On(Box::new(expr)),
    );
    let join_condition_using = map(
        rule! {
            USING ~ "(" ~ #comma_separated_list1(ident) ~ ")"
        },
        |(_, _, idents, _)| JoinCondition::Using(idents),
    );
    let join_condition = rule! { #join_condition_on | #join_condition_using };

    let join = map(
        rule! {
            #join_operator? ~ JOIN ~ #table_ref_without_join ~ #join_condition?
        },
        |(opt_op, _, right, condition)| JoinElement {
            op: opt_op.unwrap_or(JoinOperator::Inner),
            condition: condition.unwrap_or(JoinCondition::None),
            right: Box::new(right),
        },
    );
    let natural_join = map(
        rule! {
            NATURAL ~ #join_operator? ~ JOIN ~ #table_ref_without_join
        },
        |(_, opt_op, _, right)| JoinElement {
            op: opt_op.unwrap_or(JoinOperator::Inner),
            condition: JoinCondition::Natural,
            right: Box::new(right),
        },
    );

    map(
        rule!(
            #table_ref_without_join ~ ( #join | #natural_join )+
        ),
        |(fst, joins)| {
            joins.into_iter().fold(fst, |acc, elem| {
                let JoinElement {
                    op,
                    condition,
                    right,
                } = elem;
                TableReference::Join(Join {
                    op,
                    condition,
                    left: Box::new(acc),
                    right,
                })
            })
        },
    )(i)
}

pub fn join_operator(i: Input) -> IResult<JoinOperator> {
    alt((
        value(JoinOperator::Inner, rule! { INNER }),
        value(JoinOperator::LeftOuter, rule! { LEFT ~ OUTER? }),
        value(JoinOperator::RightOuter, rule! { RIGHT ~ OUTER? }),
        value(JoinOperator::FullOuter, rule! { FULL ~ OUTER? }),
        value(JoinOperator::CrossJoin, rule! { CROSS }),
    ))(i)
}

pub fn order_by_expr(i: Input) -> IResult<OrderByExpr> {
    map(
        rule! {
            #expr ~ ( ASC | DESC )?
        },
        |(expr, opt_asc)| OrderByExpr {
            expr,
            asc: opt_asc.map(|asc| asc.kind == ASC),
            nulls_first: None,
        },
    )(i)
}

pub fn set_expr(i: Input) -> IResult<SetExpr> {
    let select = map(
        rule! {
             SELECT ~ DISTINCT? ~ #comma_separated_list1(select_target)
                ~ ( FROM ~ ^#comma_separated_list1(table_reference) )?
                ~ ( WHERE ~ ^#expr )?
                ~ ( GROUP ~ ^BY ~ ^#comma_separated_list1(expr) )?
                ~ ( HAVING ~ ^#expr )?
        },
        |(
            _select,
            opt_distinct,
            select_list,
            opt_from_block,
            opt_where_block,
            opt_group_by_block,
            opt_having_block,
        )| {
            SetExpr::Select(Box::new(SelectStmt {
                span: &[],
                distinct: opt_distinct.is_some(),
                select_list,
                from: opt_from_block
                    .map(|(_, table_refs)| table_refs)
                    .unwrap_or_default(),
                selection: opt_where_block.map(|(_, selection)| selection),
                group_by: opt_group_by_block
                    .map(|(_, _, group_by)| group_by)
                    .unwrap_or_default(),
                having: opt_having_block.map(|(_, having)| having),
            }))
        },
    );

    map(
        rule!(#select ~ (UNION | EXCEPT | INTERSECT)? ~ ALL? ~ #set_expr?),
        |(select, set_operator, all_kind, set_expr)| {
            let all = all_kind.is_some();
            let left = Box::new(select.clone());
            match set_operator.map(|token| token.kind) {
                Some(TokenKind::UNION) => SetExpr::SetOperation {
                    op: SetOperator::Union,
                    all,
                    left,
                    right: Box::new(set_expr.unwrap()),
                },
                Some(TokenKind::EXCEPT) => SetExpr::SetOperation {
                    op: SetOperator::Except,
                    all,
                    left,
                    right: Box::new(set_expr.unwrap()),
                },
                Some(TokenKind::INTERSECT) => SetExpr::SetOperation {
                    op: SetOperator::Intersect,
                    all,
                    left,
                    right: Box::new(set_expr.unwrap()),
                },
                None => select,
                _ => unreachable!(),
            }
        },
    )(i)
}
