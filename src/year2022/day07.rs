//! # 2022 day 7 - No Space Left On Device
use std::error::Error;

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input).unwrap();
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

struct InputData<'a> {
    command_blocks: Vec<&'a str>,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = ();
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            command_blocks: s
                .split("$ ")
                .filter(|s| !s.is_empty())
                .map(|s| s.trim_end())
                .collect(),
        })
    }
}

impl InputData<'_> {
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
        let testdata = "$ cd /
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
7214296 k";
        let solution_data = InputData::try_from(testdata).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 95437);
        assert_eq!(p2, 24933642);
    }
}
