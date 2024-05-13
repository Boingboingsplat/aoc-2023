use nom::{branch::alt, bytes::complete::{tag, take_while}, character::complete::char, combinator::map, multi::separated_list1, sequence::preceded, AsChar, IResult};

use crate::{ModuleId, ModuleKind};

// broadcaster -> a, b, c
// %a -> b
// %b -> c
// %c -> inv
// &inv -> a

fn parse_module_id(i: &str) -> IResult<&str, ModuleId> {
    map(
        take_while(AsChar::is_alpha),
        |s: &str| ModuleId(s.to_string())
    )(i)

}

pub fn parse_line(i: &str) -> IResult<&str, (ModuleId, ModuleKind, Vec<ModuleId>)> {
    let (i, (id, kind)) = alt((
        map(preceded(char('%'), parse_module_id), |id| (id, ModuleKind::new_flipflop())),
        map(preceded(char('&'), parse_module_id), |id| (id, ModuleKind::new_conjunction())),
        map(parse_module_id, |id| (id, ModuleKind::Broadcast)),
    ))(i)?;
    let (i, _) = tag(" -> ")(i)?;
    let (i, output_ids) = separated_list1(tag(", "), parse_module_id)(i)?;

    Ok((i, (id, kind, output_ids)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_module() {
        let input = "broadcaster -> a, b, c";
        assert_eq!(
            parse_line(input),
            Ok(("", (
                ModuleId(String::from("broadcaster")), 
                ModuleKind::Broadcast, 
                vec![ModuleId(String::from("a")), ModuleId(String::from("b")), ModuleId(String::from("c"))],
            )))
        );
        let input = "%a -> b";
        assert_eq!(
            parse_line(input),
            Ok(("", (
                ModuleId(String::from("a")), 
                ModuleKind::new_flipflop(), 
                vec![ModuleId(String::from("b"))],
            )))
        );
        let input = "&inv -> a";
        assert_eq!(
            parse_line(input),
            Ok(("", (
                ModuleId(String::from("inv")), 
                ModuleKind::new_conjunction(), 
                vec![ModuleId(String::from("a"))],
            )))
        );
    }
}