
use nom::{branch::alt, bytes::complete::{tag, take_while1}, character::complete::{char, one_of}, combinator::map, multi::{many0, separated_list0}, sequence::{delimited, preceded, terminated, tuple}, AsChar, IResult};

use crate::{Attribute, Check, Part, Res, Rule, Workflow};

fn parse_res(i: &str) -> IResult<&str, Res> {
    alt((
        map(char('A'), |_| Res::Accept),
        map(char('R'), |_| Res::Reject),
        map(take_while1(AsChar::is_alpha), |s: &str| Res::Send(s.to_string())),
    ))(i)
}

// a<2006:qkq
fn parse_rule(i: &str) -> IResult<&str, Rule> {
    map(
        tuple((
            map(
                one_of("xmas"),
                |c| {
                    match c {
                        'x' => Attribute::X,
                        'm' => Attribute::M,
                        'a' => Attribute::A,
                        's' => Attribute::S,
                        _ => unreachable!(),
                    }
                },
            ),
            map(
                one_of("<>"),
                |c| {
                    match c {
                        '<' => Check::LessThan,
                        '>' => Check::GreaterThan,
                        _ => unreachable!(),
                    }
                },
            ),
            nom::character::complete::u64,
            preceded(char(':'), parse_res),
        )),
        |(attr, check, target, res)| Rule(attr, check, target, res)
    )(i)
}

// px{a<2006:qkq,m>2090:A,rfg}
pub fn parse_workflow(i: &str) -> IResult<&str, (String, Workflow)> {
    tuple((
        map(take_while1(AsChar::is_alpha), |s: &str| s.to_string()),
        delimited(
            char('{'),
            map(
                tuple((
                    many0(terminated(parse_rule, char(','))),
                    parse_res,
                )),
                |(rules, fallback)| Workflow { rules, fallback },
            ),
            char('}'),
        ),
    ))(i)
}

// {x=787,m=2655,a=1222,s=2876}
pub fn parse_part(i: &str) -> IResult<&str, Part> {
    delimited(
        char('{'),
        map(
            separated_list0(
                char(','),
                preceded(
                    alt((tag("x="), tag("m="), tag("a="),tag("s="))),
                    nom::character::complete::u64,
                )
            ),
            |attrs| Part { x: attrs[0], m: attrs[1], a: attrs[2], s: attrs[3] }
        ), 
        char('}'),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_workflow() {
        assert_eq!(
            parse_workflow("px{a<2006:qkq,m>2090:A,rfg}"),
            Ok(("", (
                String::from("px"),
                Workflow { 
                    rules: vec![
                        Rule(Attribute::A, Check::LessThan, 2006, Res::Send(String::from("qkq"))),
                        Rule(Attribute::M, Check::GreaterThan, 2090, Res::Accept),
                    ],
                    fallback: Res::Send(String::from("rfg")), 
                }
            )))
        );
    }

    #[test]
    fn test_parse_part() {
        assert_eq!(
            parse_part("{x=787,m=2655,a=1222,s=2876}"),
            Ok(("",
                Part { x: 787, m: 2655, a: 1222, s: 2876 }
            ))
        )
    }
}
