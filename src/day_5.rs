use std::collections::{BTreeSet, HashMap};

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
    start: u64,
    range: u64,
}
impl SeedRange {
    fn make_vec(map: &Vec<&str>) -> Vec<SeedRange> {
        assert_eq!(map.len(), 1);
        let line = map[0];
        let (_, line) = line.split_once(": ").unwrap();
        let nums = line
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        assert_eq!(nums.len() % 2, 0);
        let mut seeds = Vec::new();
        for i in 0..nums.len() / 2 {
            let start = nums[i * 2];
            let range = nums[i * 2 + 1];
            seeds.push(SeedRange { start, range });
        }
        seeds
    }
}

#[derive(Debug)]
struct Seeds {
    seeds: Vec<u64>,
}

impl Seeds {
    fn new(map: &Vec<&str>) -> Seeds {
        assert_eq!(map.len(), 1);
        let line = map[0];
        let (_, line) = line.split_once(": ").unwrap();
        let mut seeds = Vec::new();
        for num in line.split_whitespace() {
            seeds.push(num.parse::<u64>().unwrap());
        }
        Seeds { seeds }
    }
}

#[derive(Copy, Clone, Debug)]
struct RangeMap {
    dest_start: u64,
    source_start: u64,
    range: u64,
}

impl RangeMap {
    fn new(line: &str) -> RangeMap {
        let nums = line.split_whitespace().collect::<Vec<&str>>();
        assert_eq!(nums.len(), 3);
        let dest_start = nums[0].parse::<u64>().unwrap();
        let source_start = nums[1].parse::<u64>().unwrap();
        let range = nums[2].parse::<u64>().unwrap();
        RangeMap {
            dest_start,
            source_start,
            range,
        }
    }
    fn get_offset(&self) -> i64 {
        return self.dest_start as i64 - self.source_start as i64;
    }
    fn contains_key(&self, key: &u64) -> bool {
        if key < &self.source_start {
            return false;
        }
        key - self.source_start <= self.range - 1
    }
    fn contains_dest(&self, dest: &u64) -> bool {
        if dest < &self.dest_start {
            return false;
        }
        dest - self.dest_start <= self.range - 1
    }
    fn lookup(&self, key: &u64) -> u64 {
        assert!(self.contains_key(key));
        (*key as i64 + self.get_offset()) as u64
    }
    fn reverse_lookup(&self, dest: &u64) -> u64 {
        assert!(self.contains_dest(dest));
        (*dest as i64 - self.get_offset()) as u64
    }
    fn one_beyond(&self) -> u64 {
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

    fn lookup(&self, key: &u64) -> u64 {
        for map in self.maps.iter() {
            if map.contains_key(key) {
                return map.lookup(key);
            }
        }
        *key
    }

    fn reverse_lookup(&self, dest: &u64) -> u64 {
        for map in self.maps.iter() {
            if map.contains_dest(dest) {
                return map.reverse_lookup(dest);
            }
        }
        *dest
    }

    fn find_next(&self, key: &u64) -> Option<u64> {
        for map in self.maps.iter() {
            if map.contains_key(key) {
                return Some(map.one_beyond());
            }
        }
        // was not in map range so find next map
        self.maps
            .iter()
            .filter(|m| m.source_start > *key)
            .map(|m| m.source_start)
            .min()
    }

    fn find_transitions(&self) -> BTreeSet<u64> {
        let mut vals = BTreeSet::new();
        for map in self.maps.iter() {
            vals.insert(map.source_start);
            vals.insert(map.source_start + map.range);
        }
        vals
    }
}

#[derive(Debug)]
struct Plan {
    seeds: Seeds,
    maps: HashMap<String, Map>,
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
        let mut maps = HashMap::new();
        for map in maps_vec {
            maps.insert(map.input.clone(), map.clone());
        }
        Plan { seeds, maps }
    }

    fn map_seeds(&self, dest: &String) -> Vec<u64> {
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

fn combine_last_two_maps(map_n_minus_one: &Map, map_n: &Map) -> Map {
    let input = map_n_minus_one.input.clone();
    let output = map_n.output.clone();
    let mut transitions = map_n_minus_one.find_transitions();
    transitions.append(
        &mut map_n
            .find_transitions()
            .iter()
            .map(|x| map_n_minus_one.reverse_lookup(x))
            .collect::<BTreeSet<u64>>(),
    );
    let mut transitions = transitions.into_iter().collect::<Vec<u64>>();
    transitions.sort();
    let mut maps = Vec::new();
    for window in transitions.windows(2) {
        let start = window[0];
        let next = window[1];
        let range = next - start;
        let dest_start = map_n.lookup(&map_n_minus_one.lookup(&start));
        maps.push(RangeMap {
            dest_start,
            source_start: start,
            range,
        });
    }
    Map {
        input,
        output,
        maps,
    }
}

fn find_last_2_keys(maps: &HashMap<String, Map>, start: &str, dest: &str) -> (String, String) {
    let mut key_n = maps[start].output.clone();
    let mut key_n_minus_one = start.to_string();
    while maps[&key_n].output != dest {
        key_n_minus_one = key_n.clone();
        key_n = maps[&key_n].output.clone();
    }
    (key_n, key_n_minus_one)
}

fn pop_last_2_maps(maps: &mut HashMap<String, Map>, start: &str, dest: &str) -> (Map, Map) {
    let (key_n, key_n_minus_one) = find_last_2_keys(&maps, start, dest);
    let map_n_minus_one = maps.remove(&key_n_minus_one).unwrap();
    let map_n = maps.remove(&key_n).unwrap();
    (map_n_minus_one, map_n)
}

fn flatten_map(maps: &mut HashMap<String, Map>, start: &str, dest: &str) {
    while maps.len() > 1 {
        let (map_n_minus_one, map_n) = pop_last_2_maps(maps, start, dest);
        let new_last_map = combine_last_two_maps(&map_n_minus_one, &map_n);
        maps.insert(new_last_map.input.clone(), new_last_map);
    }
}

#[derive(Debug)]
struct PlanB {
    seeds: Vec<SeedRange>,
    map: Map,
}

impl PlanB {
    fn new(input: &str, source: &str, dest: &str) -> PlanB {
        let maps = find_maps(input);
        assert!(is_seeds(&maps[0]));
        let seeds = SeedRange::make_vec(&maps[0]);
        let maps_vec = maps[1..]
            .iter()
            .map(|map| Map::new(map))
            .collect::<Vec<Map>>();
        let mut maps = HashMap::new();
        for map in maps_vec {
            maps.insert(map.input.clone(), map.clone());
        }
        flatten_map(&mut maps, source, dest);
        let map = maps.remove(source).unwrap();
        PlanB { seeds, map }
    }

    fn find_min_location_from_seed_range(&self, seed_range: &SeedRange) -> u64 {
        let mut key = seed_range.start;
        let mut min_location = self.map.lookup(&key);
        loop {
            key = match self.map.find_next(&key) {
                Some(k) => k,
                None => {
                    break;
                }
            };
            if key - seed_range.start > seed_range.range - 1 {
                break;
            }
            min_location = min_location.min(self.map.lookup(&key));
        }
        min_location
    }

    fn find_min_location(&self) -> u64 {
        self.seeds
            .iter()
            .map(|r| self.find_min_location_from_seed_range(r))
            .min()
            .unwrap()
    }
}

fn is_seeds(map: &Vec<&str>) -> bool {
    return map.len() == 1;
}

fn day_5a(input: &str) -> u64 {
    *Plan::new(input)
        .map_seeds(&"location".to_string())
        .iter()
        .min()
        .unwrap()
}

fn day_5b(input: &str) -> u64 {
    PlanB::new(input, "seed", "location").find_min_location()
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
