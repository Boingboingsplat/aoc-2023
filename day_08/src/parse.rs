use std::collections::HashMap;

use nom::{bytes::complete::{tag, take_till, take_while}, character::complete::{multispace1, one_of}, combinator::{map, map_res}, multi::{many1, separated_list1}, sequence::{delimited, separated_pair}, AsChar, IResult};

use crate::{Direction, LabeledNode, NodeMap};

fn parse_directions(i: &str) -> IResult<&str, Vec<Direction>> {
    many1(
        map_res(one_of("LR"), char::try_into)
    )(i)
}

fn parse_node(i: &str) -> IResult<&str, LabeledNode> {
    map(
        take_while(AsChar::is_alphanum),
        |s: &str| LabeledNode { label: s.to_string() }
    )(i)
}

// DGK = (KVQ, XHR)
fn parse_key_val_pair(i: &str) -> IResult<&str, (LabeledNode, (LabeledNode, LabeledNode))> {
    let (i, key) = parse_node(i)?;
    let (i, val) = delimited(
        tag(" = ("),
        separated_pair(parse_node, tag(", "), parse_node),
        tag(")"),
    )(i)?;
    Ok((i, (key, val)))
}

pub fn parse_input(i: &str) -> IResult<&str, NodeMap> {
    let (i, dir_list) = parse_directions(i)?;
    let (i, _) = take_till(AsChar::is_alpha)(i)?;
    let (i, key_val_pairs) = separated_list1(multispace1, parse_key_val_pair)(i)?;

    let mut map = HashMap::new();
    for (key, val) in key_val_pairs {
        map.insert(key, val);
    }

    Ok((i, NodeMap { map, dir_list }))
}