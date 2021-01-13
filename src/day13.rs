use std::io::BufRead;

const INPUT_PATH: &str = "input/day13";

fn smaller_timestamp_greater_than(timestamp: u32, buses: &[u32]) -> Box<[(u32, u32)]> {
    let mut timestamps = Vec::with_capacity(buses.len());

    for &bus in buses {
        timestamps.push((
            bus,
            (timestamp..timestamp + bus)
                .filter(|&bus_id| bus_id > timestamp && bus_id % bus == 0)
                .min(),
        ));
    }

    timestamps
        .iter()
        .filter(|&(_, timestamp)| timestamp.is_some())
        .map(|&(bus, timestamp)| (bus, timestamp.unwrap()))
        .collect()
}

fn earliest_departing_bus(timestamp: u32, buses: &[u32]) -> u32 {
    let res = smaller_timestamp_greater_than(timestamp, buses)
        .iter()
        .min_by_key(|elem| elem.1)
        .unwrap()
        .to_owned();
    let (bus_id, departure) = res;

    bus_id * (departure - timestamp)
}

pub fn part1() -> u32 {
    let mut input = crate::get_buffered_input(INPUT_PATH);
    let mut timestamp = String::new();
    let mut buses = String::new();

    input.read_line(&mut timestamp).unwrap();
    input.read_line(&mut buses).unwrap();

    let timestamp: u32 = timestamp.trim_end().parse().unwrap();
    let buses: Vec<u32> = buses
        .trim_end()
        .split(',')
        .filter_map(|id| id.parse::<u32>().ok())
        .collect();

    earliest_departing_bus(timestamp, &buses)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_part1() {
        assert_eq!(part1(), 4782);
    }

    #[test]
    fn test_part1_example() {
        let timestamp = 939;
        let buses = [7, 13, 59, 31, 19];
        let res = earliest_departing_bus(timestamp, &buses);

        assert_eq!(res, 295);
    }
}
