use crate::Context;
use anyhow::{anyhow, Context as _, Result};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{char, multispace0},
    combinator::map,
    multi::fold_many1,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};
use std::collections::HashMap;

pub fn fulfill(raw: &str, input: &HashMap<String, String>, context: &Context) -> Result<String> {
    let (_, texts) = parse(raw).map_err(|_| anyhow!("Unable to parse expression {}.", raw))?;
    let mut result = String::new();
    for text in texts {
        result.push_str(match text {
            Text::Literal(s) => s,
            Text::Expression(Expression { namespace, field }) if namespace == "env" => context
                .env
                .get(field)
                .with_context(|| format!("Missing {}.", field))?,
            Text::Expression(Expression {
                namespace: _,
                field,
            }) => input
                .get(field)
                .with_context(|| format!("Missing {}.", field))?,
        });
    }

    Ok(result)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Text<'a> {
    Literal(&'a str),
    Expression(Expression<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Expression<'a> {
    namespace: &'a str,
    field: &'a str,
}

fn literal(input: &str) -> IResult<&str, Text> {
    map(is_not("{"), |s: &str| Text::Literal(s))(input)
}

fn namespace(input: &str) -> IResult<&str, &str> {
    alt((tag("input"), tag("env")))(input)
}

fn field(input: &str) -> IResult<&str, &str> {
    is_not("\t }")(input)
}

fn expression(input: &str) -> IResult<&str, Expression> {
    map(separated_pair(namespace, tag("."), field), |(ns, f)| {
        Expression {
            namespace: ns,
            field: f,
        }
    })(input)
}

fn enclosed(input: &str) -> IResult<&str, Text> {
    map(
        delimited(
            char('{'),
            preceded(multispace0, expression),
            preceded(multispace0, char('}')),
        ),
        |expr| Text::Expression(expr),
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Text>> {
    fold_many1(alt((literal, enclosed)), Vec::new(), |mut acc, t| {
        acc.push(t);
        acc
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enclosed_expression() {
        assert_eq!(
            enclosed("{ env.gist_secret }"),
            Ok((
                "",
                Text::Expression(Expression {
                    namespace: "env",
                    field: "gist_secret"
                })
            ))
        );
        assert_eq!(
            enclosed("{input.status_code}"),
            Ok((
                "",
                Text::Expression(Expression {
                    namespace: "input",
                    field: "status_code"
                })
            ))
        );
    }

    #[test]
    fn test_text() {
        assert_eq!(
            parse("hello {env.ttt}"),
            Ok((
                "",
                vec![
                    Text::Literal("hello "),
                    Text::Expression(Expression {
                        namespace: "env",
                        field: "ttt"
                    })
                ]
            ))
        );

        assert_eq!(
            parse("这是 Server 结果 {input.status_code}, 今天天气是 {input.text}。"),
            Ok((
                "",
                vec![
                    Text::Literal("这是 Server 结果 "),
                    Text::Expression(Expression {
                        namespace: "input",
                        field: "status_code"
                    }),
                    Text::Literal(", 今天天气是 "),
                    Text::Expression(Expression {
                        namespace: "input",
                        field: "text"
                    }),
                    Text::Literal("。"),
                ]
            ))
        );
    }
}
