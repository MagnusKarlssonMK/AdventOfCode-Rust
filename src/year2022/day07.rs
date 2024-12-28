//! # 2022 day 7 - No Space Left On Device
pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    let (p1, p2) = solution_data.solve();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

struct InputData<'a> {
    command_blocks: Vec<&'a str>,
}

impl<'a> InputData<'a> {
    fn parse_input(input: &'a str) -> Self {
        Self {
            command_blocks: input
                .split("$ ")
                .filter(|s| !s.is_empty())
                .map(|s| s.trim_end())
                .collect(),
        }
    }

    fn solve(&self) -> (usize, usize) {
        let mut dir_sizes = Vec::new();
        let mut buffer = Vec::new();
        let mut total = 0;
        for &block in self.command_blocks.iter() {
            let mut lines = block.lines();
            let commandline = lines.next().unwrap();
            match &commandline[0..2] {
                "cd" => {
                    if commandline.ends_with("..") {
                        dir_sizes.push(total);
                        total += buffer.pop().unwrap();
                    } else {
                        buffer.push(total);
                        total = 0;
                    }
                }
                "ls" => {
                    for line in lines {
                        let (first, _) = line.split_once(' ').unwrap();
                        if first.as_bytes()[0].is_ascii_digit() {
                            total += first.parse::<usize>().unwrap();
                        }
                    }
                }
                _ => panic!("Unexpected command in input"),
            }
        }

        while let Some(n) = buffer.pop() {
            dir_sizes.push(total);
            total += n;
        }

        let p1 = dir_sizes.iter().filter(|&&n| n < 100_000).sum();

        let space_required = 30_000_000 - (70_000_000 - dir_sizes.last().unwrap());
        let p2 = *dir_sizes
            .iter()
            .filter(|&&n| n >= space_required)
            .min()
            .unwrap();

        (p1, p2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from(
            "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
        );
        let solution_data = InputData::parse_input(&testdata);
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 95437);
        assert_eq!(p2, 24933642);
    }
}
