use nom::{
    bytes::complete::tag, character::complete as cc, character::complete::space1,
    multi::separated_list1, sequence::tuple, IResult,
};

use core::ops::Range;
use rangemap::RangeMap;

#[derive(Clone, Copy)]
pub struct GardenMap {
    pub dest_range_start: i64,
    pub source_range_start: i64,
    pub length: i64,
}

impl GardenMap {
    pub fn source_range_end(self) -> i64 {
        self.source_range_start + self.length
    }
    pub fn dest_range_end(self) -> i64 {
        self.dest_range_start + self.length
    }
}

pub fn parse_block(i: &str) -> IResult<&str, Vec<GardenMap>> {
    let (i, _) = parse_name(i)?;
    let (i, garden_maps) = separated_list1(tag("\n"), parse_line)(i)?;
    Ok((i, garden_maps))
}

pub fn parse_block_range_map(i: &str) -> IResult<&str, RangeMap<i64, (i64, i64, i64)>> {
    let mut range_map: RangeMap<i64, (i64, i64, i64)> = RangeMap::new();
    let (i, _) = parse_name(i)?;
    let (i, maps) = separated_list1(tag("\n"), parse_line_range)(i)?;
    for map in maps {
        range_map.insert(map.0..map.0 + map.2, (map.0, map.1, map.2))
    }
    Ok((i, range_map))
}

pub fn parse_block_range_map_reverse(i: &str) -> IResult<&str, RangeMap<i64, (i64, i64, i64)>> {
    let mut range_map: RangeMap<i64, (i64, i64, i64)> = RangeMap::new();
    let (i, _) = parse_name(i)?;
    let (i, maps) = separated_list1(tag("\n"), parse_line_range)(i)?;
    for map in maps {
        range_map.insert(map.1..map.1 + map.2, (map.1, map.0, map.2))
    }
    Ok((i, range_map))
}

pub fn parse_line_range(i: &str) -> IResult<&str, (i64, i64, i64)> {
    let (i, (dest_range_start, _, source_range_start, _, length)) =
        tuple((cc::i64, space1, cc::i64, space1, cc::i64))(i)?;
    Ok((i, (source_range_start, dest_range_start, length)))
}

pub fn parse_name(i: &str) -> IResult<&str, String> {
    // seed-to-soil map:\n
    let (i, (from, _, _, _, to, _, _)) = tuple((
        cc::alphanumeric1,
        tag("-"),
        cc::alphanumeric1,
        tag("-"),
        cc::alphanumeric1,
        space1,
        tag("map:\n"),
    ))(i)?;
    Ok((i, format!("{from}-{to}")))
}

pub fn parse_line(i: &str) -> IResult<&str, GardenMap> {
    let (i, (dest_range_start, _, source_range_start, _, length)) =
        tuple((cc::i64, space1, cc::i64, space1, cc::i64))(i)?;
    Ok((
        i,
        GardenMap {
            dest_range_start,
            source_range_start,
            length,
        },
    ))
}

pub fn parse_seeds(i: &str) -> IResult<&str, Vec<i64>> {
    let (i, (_, seeds)) = tuple((tag("seeds: "), separated_list1(tag(" "), cc::i64)))(i)?;
    Ok((i, seeds))
}
