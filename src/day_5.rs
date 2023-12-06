pub fn day_5() {
    let input = include_str!("day_5_data.txt");
    println!("day 5a {}", day_5a(input));
    println!("day 5b {}", day_5b(input));
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
struct SeedRange {
    start: u32,
    range: u32,
}
impl SeedRange {
    fn make_vec(map: &Vec<&str>) -> Vec<SeedRange> {
        assert_eq!(map.len(), 1);
        let line = map[0];
        let (_, line) = line.split_once(": ").unwrap();
        let nums = line
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        assert_eq!(nums.len() % 2, 0);
        let mut seeds = Vec::new();
        for i in 0..nums.len() / 2 {
            let start = nums[i * 2];
            let range = nums[i * 2 + 1];
            seeds.push(SeedRange{start,range});
        }
        seeds
    }
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
        Seeds { seeds }
    }
}

#[derive(Copy, Clone, Debug)]
struct RangeMap {
    dest_start: u32,
    source_start: u32,
    range: u32,
}

impl RangeMap {
    fn new(line: &str) -> RangeMap {
        let nums = line.split_whitespace().collect::<Vec<&str>>();
        assert_eq!(nums.len(), 3);
        let dest_start = nums[0].parse::<u32>().unwrap();
        let source_start = nums[1].parse::<u32>().unwrap();
        let range = nums[2].parse::<u32>().unwrap();
        RangeMap {
            dest_start,
            source_start,
            range,
        }
    }
    fn get_offset(&self) -> u32 {
        return self.dest_start - self.source_start
    }
    fn contains_key(&self, key: &u32) -> bool {
        if key < &self.source_start {
            return false;
        }
        key - self.source_start <= self.range - 1
    }
    fn contains_dest(&self, dest: &u32) -> bool {
        if dest < &self.dest_start {
            return false;
        }
        dest - self.dest_start <= self.range - 1
    }
    fn lookup(&self, key: &u32) -> u32 {
        assert!(self.contains_key(key));
        return key - self.source_start + self.dest_start;
    }
    fn reverse_lookup(&self, dest: &u32) -> u32 {
        assert!(self.contains_dest(dest));
        return dest + self.source_start - self.dest_start;
    }
    fn one_beyond(&self) -> u32 {
        self.source_start + self.range
    }
}

#[derive(Debug, Clone)]
struct Map {
    input: String,
    output: String,
    maps: Vec<RangeMap>,
}

impl Map {
    fn new(map: &Vec<&str>) -> Map {
        assert!(map.len() > 1);
        let (line, _) = map[0].split_once(" map").unwrap();
        let (input, output) = line.split_once("-to-").unwrap();
        let maps = map[1..]
            .iter()
            .map(|line| RangeMap::new(line))
            .collect::<Vec<RangeMap>>();
        Map {
            input: input.to_string(),
            output: output.to_string(),
            maps: maps,
        }
    }

    fn lookup(&self, key: &u32) -> u32 {
        for map in self.maps.iter() {
            if map.contains_key(key) {
                return map.lookup(key);
            }
        }
        *key
    }

    fn reverse_lookup(&self, dest: &u32) -> u32 {
        for map in self.maps.iter() {
            if map.contains_dest(key) {
                return map.reverse_lookup(dest);
            }
        }
        *key
    }

    fn find_next(&self, key: &u32) -> u32 {
        for map in self.maps.iter() {
            if map.contains_key(key) {
                return map.one_beyond();
            }
        }
        // was not in map range so find next map
        self.maps.iter().filter(|m| m.source_start > key).map(|m| m.source_start).min()
    }

    fn get_offset(&self, key: &u32) -> u32 {
        for map in self.maps.iter() {
            if map.contains_key(key) {
                return map.get_offset()
            }
        }
        0
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
        let maps_vec = maps[1..]
            .iter()
            .map(|map| Map::new(map))
            .collect::<Vec<Map>>();
        let mut maps = std::collections::HashMap::new();
        for map in maps_vec {
            maps.insert(map.input.clone(), map.clone());
        }
        Plan { seeds, maps }
    }

    fn map_seeds(&self, dest: &String) -> Vec<u32> {
        let mut out = Vec::new();
        for seed in &self.seeds.seeds {
            let mut key = &"seed".to_string();
            let mut val = *seed;
            while key != dest {
                let map = &self.maps[key];
                val = map.lookup(&val);
                key = &map.output;
            }
            out.push(val);
        }
        out
    }
}

fn find_next_last_two_maps(map_n_minus_one: &Map, map_n: &Map, start: &u32) -> u32 {
    let next_top = map_n_minus_one.find_next(start)
    let next_low = map_n_minus_one.reverse_lookup(map_n.find_next(map_n_minus_one.lookup(start)));
    next_top.min(next_low);
}

fn combine_last_two_maps(map_n_minus_one: &Map, map_n: &Map) -> Map {
    let input = map_n_minus_one.input;
    let output = map_n.output;
    let start = map_n_minus_one.maps.iter().map(|m| m.dest_start).min().unwrap();
    let next = find_next_last_two_maps(map_n_minus_one, map_n, start);
    Map{input, output, maps}
}


fn flatten_map(maps: std::collections::HashMap<Map>, start: &str, dest:&str) -> Map {
    let mut key = start;
    while key != dest {
        let map = maps[key].unwrap();
    }
}

#[derive(Debug)]
struct PlanB {
    seeds: Vec<SeedRange>,
    maps: std::collections::HashMap<String, Map>,
}

impl PlanB {
    fn new(input: &str) -> Plan {
        let maps = find_maps(input);
        assert!(is_seeds(&maps[0]));
        let seeds = SeedRange::make_vec(&maps[0]);
        let maps_vec = maps[1..]
            .iter()
            .map(|map| Map::new(map))
            .collect::<Vec<Map>>();
        let mut maps = std::collections::HashMap::new();
        for map in maps_vec {
            maps.insert(map.input.clone(), map.clone());
        }
        Plan { seeds, maps }
    }

    fn map_seeds(&self, dest: &String) -> Vec<u32> {
        let mut out = Vec::new();
        for seed in &self.seeds.seeds {
            let mut key = &"seed".to_string();
            let mut val = *seed;
            while key != dest {
                let map = &self.maps[key];
                val = map.lookup(&val);
                key = &map.output;
            }
            out.push(val);
        }
        out
    }
}

fn is_seeds(map: &Vec<&str>) -> bool {
    return map.len() == 1;
}

fn day_5a(input: &str) -> u32 {
    *Plan::new(input)
        .map_seeds(&"location".to_string())
        .iter()
        .min()
        .unwrap()
}

fn day_5b(input: &str) -> u32 {
    *Plan::new_b(input)
        .map_seeds(&"location".to_string())
        .iter()
        .min()
        .unwrap()
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
        assert_eq!(super::day_5a(input), 35);
        assert_eq!(super::day_5b(input), 46);
    }
}
