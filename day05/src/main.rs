use std::path::Path;
use::std::fs;
use std::cmp::min;
use std::io::BufRead;
use std::ops::Range;


use nom::{
    IResult,
    character::complete as cc,
    character::complete::space1,
    bytes::complete::tag,
    multi::separated_list1,
    sequence::tuple,
};

fn main() {
    day5_task1();
    day5_task2();
}

fn day5_task1() {
    let path = Path::new("src/day5.txt");
    let contents = fs::read_to_string(path).expect("Should have been able to read file.");

    let mut maps = contents.split("\n\n");
    let seeds = parse_seeds(maps.next().expect("Has no first element.")).unwrap().1;

    let fields: Vec<Vec<GardenMap>> = maps.map(|block| parse_block(block).unwrap().1).collect();
    let mut sol: i64 = std::i64::MAX;

    for seed in seeds {
        let mut translations = vec![seed];
        let mut breaked = false;
        for field in &fields {
            translations = find_translation(&translations, &field);
        }
        sol = min(sol, *translations.iter().min().unwrap());
    }

    // implement here
    println!("Day5 Task1: {sol}");
}

fn find_translation(translations: &Vec<i64>, field: &Vec<GardenMap>) -> Vec<i64> {
    let mut new_translations: Vec<i64> = vec![];
    for translation in translations {
        let mut pushed = false;
        for map in field {
            if (map.source_range_start..map.source_range_end()).contains(translation) {
                new_translations.push(map.dest_range_start + translation - map.source_range_start);
                pushed = true;
            }
        }
        if !pushed {
            new_translations.push(*translation)
        }
    }
    new_translations
}

#[derive(Clone, Copy)]
struct GardenMap {
    dest_range_start: i64,
    source_range_start: i64,
    length: i64,
}

impl GardenMap {
    fn source_range_end(self,) -> i64 {
        self.source_range_start + self.length
    }
    fn dest_range_end(self,) -> i64 {
        self.dest_range_start + self.length
    }
}

fn parse_block(i: &str) -> IResult<&str, Vec<GardenMap>> {
    let (i, _) = parse_name(i)?;
    let (i, garden_maps) = separated_list1(tag("\n"), parse_line)(i)?;
    Ok((i, garden_maps))
}

fn parse_name(i: &str) -> IResult<&str, String> {
    // seed-to-soil map:\n
    let (i, (from, _, _, _, to, _, _)) = tuple((cc::alphanumeric1, tag("-"), cc::alphanumeric1, tag("-"), cc::alphanumeric1, space1, tag("map:\n")))(i)?;
    Ok((i, format!("{from}-{to}")))
}

fn parse_line(i: &str) -> IResult<&str, GardenMap> {
    let (i, (dest_range_start, _, source_range_start, _, length)) = tuple((cc::i64, space1, cc::i64, space1, cc::i64))(i)?;
    Ok((i, GardenMap{
        dest_range_start,
        source_range_start,
        length,
    }))
}

fn parse_seeds(i: &str) -> IResult<&str, Vec<i64>> {
    let (i, (_, seeds)) = tuple((tag("seeds: "), separated_list1(tag(" "), cc::i64)))(i)?;
    Ok((i, seeds))
}

fn day5_task2() {
    let path = Path::new("src/day5.txt");
    let contents = fs::read_to_string(path).expect("Should have been able to read file.");

    let mut maps = contents.split("\n\n");
    let seeds = parse_seeds(maps.next().expect("Has no first element.")).unwrap().1;
    let seed_ranges: Vec<Range<i64>> = seeds.chunks(2).map(|x| x[0]..(x[0]+x[1])).collect();

    let fields: Vec<Vec<GardenMap>> = maps.map(|block| parse_block(block).unwrap().1).collect();
    let mut sol: i64 = std::i64::MAX;

    let soil_maps = fields.last().unwrap();
    let soil_ranges: Vec<Range<i64>> = soil_maps.into_iter().map(|map| map.dest_range_start..map.dest_range_end()).collect();

    for soil_range in soil_ranges {
        for index in soil_range {
            if index >= sol { break; }
            if is_valid_soil(index, &fields, &seed_ranges) {
                sol = min(sol, index);
                println!("Current min: {}", sol)
            }
        }
    }
    println!("Day5 Task2: {}", sol)
}

fn is_valid_soil(index: i64, garden_maps: &Vec<Vec<GardenMap>>, seed_ranges: &Vec<Range<i64>>) -> bool {
    let mut map_value = index;
    for garden_map in garden_maps.into_iter().rev() {
        for map in garden_map {
            if (map.dest_range_start..map.dest_range_end()).contains(&map_value) {
                map_value = map.source_range_start - map.dest_range_start + map_value;
                break;
            }
        }
    }
    check_values_in_seed_ranges(map_value, seed_ranges)
}

fn check_values_in_seed_ranges(map_value: i64, seed_ranges: &Vec<Range<i64>>) -> bool {
    for seed_range in seed_ranges {
        if seed_range.contains(&map_value) {
            return true
        }
    }
    false
}