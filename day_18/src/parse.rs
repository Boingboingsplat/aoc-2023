use nom::{bytes::complete::{take, take_until}, character::complete::{char, one_of}, combinator::map_res, IResult};

// R 6 (#70c710)
pub fn parse_line_part_1(i: &str) -> IResult<&str, (i64, i64)> {
    let (i, dir) = one_of("UDLR")(i)?;
    let (i, _) = char(' ')(i)?;
    let (i, steps) = nom::character::complete::i64(i)?;
    // Rest is ignored in part 1

    let offset = match dir {
        'U' => (0, -steps),
        'D' => (0, steps),
        'L' => (steps, 0),
        'R' => (-steps, 0),
        _ => unreachable!(),
    };

    Ok((i, offset))
}

pub fn parse_line_part_2(i: &str) -> IResult<&str, (i64, i64)> {
    // All input until hex code is ignored in part 2
    let (i, _) = take_until("#")(i)?;
    let (i, _) = take(1usize)(i)?;
    let (i, steps) = map_res(
        take(5usize),
        |s| i64::from_str_radix(s, 16),
    )(i)?;
    let (i, dir) = one_of("0123")(i)?;
    // Rest is ignored in part 2
    
    let offset = match dir {
        '0' => (-steps, 0),
        '1' => (0, steps),
        '2' => (steps, 0),
        '3' => (0, -steps),
        _ => unreachable!(),
    };

    Ok((i, offset))
}