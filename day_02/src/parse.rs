use nom::{branch::alt, bytes::complete::tag, combinator::map, multi::separated_list0, sequence::terminated, IResult};

use crate::{CubeSet, Game};

enum CubeCount {
    Red(u32),
    Green(u32),
    Blue(u32),
}

fn parse_cube_set(i: &str) -> IResult<&str, CubeSet> {
    map(
        separated_list0(
            tag(", "),
            alt((
                map(terminated(nom::character::complete::u32, tag(" red")), |n| CubeCount::Red(n)),
                map(terminated(nom::character::complete::u32, tag(" blue")), |n| CubeCount::Blue(n)),
                map(terminated(nom::character::complete::u32, tag(" green")), |n| CubeCount::Green(n)),
            )),
        ),
        |entries| {
            let mut res = CubeSet { red: 0, green: 0, blue: 0 };
            for count in entries {
                match count {
                    CubeCount::Red(n) => res.red += n,
                    CubeCount::Green(n) => res.green += n,
                    CubeCount::Blue(n) => res.blue += n,
                }
            }
            res
        }
    )(i)
}

//  Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
pub fn parse_game(i: &str) -> IResult<&str, Game> {
    let (i, _) = tag("Game ")(i)?;
    let (i, id) = nom::character::complete::u32(i)?;
    let (i, _) = tag(": ")(i)?;
    let (i, rounds) = separated_list0(tag("; "), parse_cube_set)(i)?;

    Ok((i, Game { id, rounds }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cube_set() {
        assert_eq!(parse_cube_set("4 red"), Ok(("", CubeSet { red: 4, green: 0, blue: 0 })));
        assert_eq!(parse_cube_set("4 green, 2 blue"), Ok(("", CubeSet { red: 0, green: 4, blue: 2 })));
        assert_eq!(parse_cube_set("12 blue, 8 red"), Ok(("", CubeSet { red: 8, green: 0, blue: 12 })));
        assert_eq!(parse_cube_set("1 green, 2 red, 3 blue"), Ok(("", CubeSet { red: 2, green: 1, blue: 3 })));
    }
}