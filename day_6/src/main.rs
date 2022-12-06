use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    for line in io::BufReader::new(file).lines().map(|x| x.unwrap()) {
        println!(
            "First packet marker is at position: {}",
            first_start_of_packet_marker(&line)
        );
        println!(
            "First message marker is at position: {}",
            first_start_of_message_marker(&line)
        );
    }
}

fn first_start_of_message_marker(line: &str) -> usize {
    unique_substring_end(line, 14)
}

fn first_start_of_packet_marker(line: &str) -> usize {
    unique_substring_end(line, 4)
}

fn unique_substring_end(line: &str, size: usize) -> usize {
    let bytes = line.as_bytes();
    for i in 0..line.len() {
        if i > size - 2 {
            let slice = &bytes[i - (size - 1)..i + 1];
            if unique(slice) {
                return i + 1;
            }
        }
    }
    panic!("Cound not find substring of {} unique characters", size)
}

fn unique(slice: &[u8]) -> bool {
    let size = slice.len();
    let set: HashSet<&u8> = HashSet::from_iter(slice.iter());
    return size == set.len();
}

#[cfg(test)]
mod tests {
    use crate::{first_start_of_message_marker, first_start_of_packet_marker};

    #[test]
    fn part_1() {
        assert_eq!(
            first_start_of_packet_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            5
        );
        assert_eq!(
            first_start_of_packet_marker("nppdvjthqldpwncqszvftbrmjlhg"),
            6
        );
        assert_eq!(
            first_start_of_packet_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            10
        );
        assert_eq!(
            first_start_of_packet_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            11
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            first_start_of_message_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            19
        );
        assert_eq!(
            first_start_of_message_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            23
        );
        assert_eq!(
            first_start_of_message_marker("nppdvjthqldpwncqszvftbrmjlhg"),
            23
        );
        assert_eq!(
            first_start_of_message_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            29
        );
        assert_eq!(
            first_start_of_message_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            26
        );
    }
}
