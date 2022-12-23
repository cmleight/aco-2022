fn parse_data(input: &str) -> Option<Vec<[[u32; 2]; 2]>> {
    fn to_range(s: &str) -> Option<[u32; 2]> {
        let (min, max) = s.split_once('-')?;
        Some([min.parse().ok()?, max.parse().ok()?])
    }

    let mut pairs = Vec::new();
    for line in input.lines() {
        let (elf1, elf2) = line.split_once(',')?;
        pairs.push([to_range(elf1)?, to_range(elf2)?]);
    }
    Some(pairs)
}

pub fn part_1(input: &str) -> usize {
    parse_data(input).unwrap().iter()
        .filter(|[[min1, max1], [min2, max2]]| {
            (min1 >= min2 && max1 <= max2) || (min2 >= min1 && max2 <= max1)
        })
        .count()
}

pub fn part_2(input: &str) -> usize {
    parse_data(input).unwrap().iter()
        .filter(|[[min1, max1], [min2, max2]]| {
            min1 <= max2 && max1 >= min2
        })
        .count()
}

fn main() {
    let input = std::fs::read_to_string("input.txt");
    let converted = input.unwrap();
    let expected = part_1(converted.as_str());
    println!("{expected}");
    let expected = part_2(converted.as_str());
    println!("{expected}");
}
