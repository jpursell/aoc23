pub fn day_5() {
    // let input = include_str!("day_5_data.txt");
    // println!("day 5a {}", day_5a(input));
    // println!("day 5b {}", day_5b(input));
}

fn find_maps(input: &str) -> Vec<Vec<&str>> {
    let mut out = Vec::new();
    let mut map = Vec::new();
    let mut in_map = false;
    for line in input.lines() {
        match in_map {
            true => {
                if line.trim().is_empty() {
                    out.push(map.clone());
                    map.clear();
                    in_map = false;
                } else {
                    map.push(line);
                }
            }
            false => {
                if !line.trim().is_empty() {
                    in_map = true;
                    map.push(line);
                }
            }
        }
    }
    if in_map {
        out.push(map.clone());
    }
    out
}

#[derive(Debug)]
struct Seeds {
    seeds: Vec<u32>,
}

impl Seeds {
    fn new(map: &Vec<&str>) -> Seeds {
        assert_eq!(map.len(), 1);
        let line = map[0];
        let (_, line) = line.split_once(": ").unwrap();
        let mut seeds = Vec::new();
        for num in line.split_whitespace() {
            seeds.push(num.parse::<u32>().unwrap());
        }
        Seeds{seeds}
    }
}

#[derive(Debug, Clone)]
struct Map {
    input: String,
    output: String,
    map: std::collections::BTreeMap<u32, u32>,
}

impl Map {
    fn new(map: &Vec<&str>) -> Map {
        assert!(map.len() > 1);
        let (line, _) = map[0].split_once(" map").unwrap();
        let (input, output) = line.split_once("-to-").unwrap();
        let mut num_map = std::collections::BTreeMap::new();
        for line in &map[1..] {
            let nums = line.split_whitespace().collect::<Vec<&str>>();
            assert_eq!(nums.len(), 3);
            let dest_start = nums[0].parse::<u32>().unwrap();
            let source_start = nums[1].parse::<u32>().unwrap();
            let length = nums[2].parse::<u32>().unwrap();
            for i in 0..length{
                assert_eq!(num_map.insert(source_start + i, dest_start + i), None);
            }
        }
        Map{input: input.to_string(), output: output.to_string(), map: num_map}
    }
}

#[derive(Debug)]
struct Plan {
    seeds: Seeds,
    maps: std::collections::HashMap<String, Map>,
}

impl Plan {
    fn new(input: &str) -> Plan {
        let maps = find_maps(input);
        assert!(is_seeds(&maps[0]));
        let seeds = Seeds::new(&maps[0]);
        let maps_vec = maps[1..].iter().map(|map| Map::new(map)).collect::<Vec<Map>>();
        let mut maps = std::collections::HashMap::new();
        for map in maps_vec {
            maps.insert(map.input.clone(), map.clone());
        }
        Plan{seeds, maps}
    }

    fn map_seeds(&self, dest: String) -> Vec<u32> {
        let mut out = Vec::new();
        let mut key = "seed".to_string;
        for seed in self.seeds {
            let mut val = seed;
            while key != dest {
                let map = self.maps[key];
                let val = match map.contains_key(val) {
                    true => map[val],
                    false => val,
                };
                key = map.output;
            }
            out.push(val);
        }
    }
}

fn is_seeds(map: &Vec<&str>) -> bool {
    return map.len() == 1;
}

fn day_5a(input: &str) -> u32 {
    let plan = Plan::new(input);
    dbg!(plan);
    0
}

fn day_5b(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
        assert_eq!(super::day_5a(input), 0);
        assert_eq!(super::day_5b(input), 0);
    }
}
