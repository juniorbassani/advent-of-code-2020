use std::io::BufRead;

const INPUT_PATH: &str = "input/day13";

fn smaller_timestamp_greater_than(timestamp: u32, buses: &[u32]) -> Box<[(u32, u32)]> {
    let mut timestamps = Vec::with_capacity(buses.len());

    for &bus in buses {
        timestamps.push((
            bus,
            (timestamp..timestamp + bus)
                .filter(|&bus_id| bus_id % bus == 0)
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

fn earliest_timestamp_with_offsets(buses: &[(usize, u32)]) -> usize {
    let mut timestamp = 1;
    let mut wait_time = 1;

    for &(index, id) in buses {
        while (timestamp + index) % id as usize != 0 {
            timestamp += wait_time;
        }
        wait_time *= id as usize;
    }

    timestamp
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

pub fn part2() -> usize {
    let input = crate::get_input_as_string(INPUT_PATH);
    let mut index = 0;
    let mut buses = Vec::new();

    input
        .trim_end()
        .split('\n')
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .for_each(|bus| {
            if let Ok(bus_id) = bus.parse::<u32>() {
                buses.push((index, bus_id));
            }
            index += 1;
        });

    earliest_timestamp_with_offsets(&buses)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_part1() {
        assert_eq!(part1(), 4782);
    }

    #[test]
    fn compute_part2() {
        assert_eq!(part2(), 1118684865113056);
    }

    #[test]
    fn test_part1_example() {
        let timestamp = 939;
        let buses = [7, 13, 59, 31, 19];
        let res = earliest_departing_bus(timestamp, &buses);

        assert_eq!(res, 295);
    }

    #[test]
    fn test_part2_example() {
        let buses = [(0, 7), (1, 13), (4, 59), (6, 31), (7, 19)];
        let res = earliest_timestamp_with_offsets(&buses);

        assert_eq!(res, 1068781);
    }
}
