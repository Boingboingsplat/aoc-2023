use nom::{bytes::complete::{tag, take_till}, character::complete::{multispace1, space1}, combinator::map, multi::separated_list1, sequence::{preceded, tuple}, AsChar, IResult};

use crate::{Almanac, RangeMap};

fn parse_range_map(i: &str) -> IResult<&str, RangeMap> {
    map(
        tuple((
            nom::character::complete::u64,
            preceded(space1, nom::character::complete::u64),
            preceded(space1, nom::character::complete::u64),
        )),
        |(dest_start, source_start, len)| { RangeMap::new(dest_start, source_start, len) }
    )(i)
}

fn parse_almanac(i: &str) -> IResult<&str, Almanac> {
    let (i, _) = take_till(AsChar::is_dec_digit)(i)?;
    let (i, maps) = separated_list1(multispace1, parse_range_map)(i)?;
    Ok((i, Almanac::new(maps)))
}

// seeds: 79 14 55 13

// seed-to-soil map:
// 50 98 2
// 52 50 48

pub fn parse_input(i: &str) -> IResult<&str, (Vec<u64>, Vec<Almanac>)> {
    let (i, _) = tag("seeds: ")(i)?;
    let (i, seeds) = separated_list1(space1, nom::character::complete::u64)(i)?;
    let (i, almanacs) = separated_list1(multispace1, parse_almanac)(i)?;
    Ok((i, (seeds, almanacs)))
}