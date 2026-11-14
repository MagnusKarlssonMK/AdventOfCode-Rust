//! # 2022 day 7 - No Space Left On Device
//!
//! Parse the terminal output into directory-change and `ls` commands, then replay it with a stack
//! of ancestor sizes. `ls` adds file sizes to the current directory, while `cd ..` and `cd /`
//! finalize finished directories and fold their totals back into their parents. The resulting flat
//! list of directory sizes answers both parts: sum the directories of at most 100,000, then find
//! the smallest directory that frees enough space for the update.
use crate::aoc_util::error::{AocError, OptionExt};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    let (p1, p2) = solution_data.solve()?;
    Ok((p1.to_string(), p2.to_string()))
}

const P1_MAX_DIR_SIZE: u64 = 100_000;
const P2_MIN_UNUSED_SPACE: u64 = 30_000_000;
const P2_TOTAL_SPACE_AVAILABLE: u64 = 70_000_000;

enum Content {
    Directory,
    /// File size
    File(u64),
}

impl FromStr for Content {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["dir", _] => Ok(Self::Directory),
            [value, _] => Ok(Self::File(value.parse()?)),
            _ => Err(AocError::Invalid(format!("invalid content: {}", s))),
        }
    }
}

enum Command {
    /// Change directory to the outermost directory: `$ cd /`
    CdRoot,
    /// Change directory out one level: `$ cd ..`
    CdOut,
    /// Change directory in one level into x: `$ cd x`
    CdIn,
    /// List all content of current directory: `$ ls`
    Ls(Vec<Content>),
}

impl FromStr for Command {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().lines();
        let first = lines.next().ctx("empty command")?;
        match first.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["cd", "/"] => Ok(Self::CdRoot),
            ["cd", ".."] => Ok(Self::CdOut),
            ["cd", _] => Ok(Self::CdIn),
            ["ls"] => Ok(Self::Ls(
                lines.map(|line| line.parse()).collect::<Result<_, _>>()?,
            )),
            _ => Err(AocError::Invalid(format!("invalid command: {}", s))),
        }
    }
}

struct InputData {
    command_blocks: Vec<Command>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            command_blocks: s
                .trim_start_matches("$ ")
                .split("$ ")
                .map(|block| block.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve(&self) -> Result<(u64, u64), AocError> {
        let mut dir_sizes = Vec::new();
        let mut buffer = vec![0];
        let mut total = 0;
        for cmd in &self.command_blocks {
            match cmd {
                Command::CdRoot => {
                    // Finalize the current directory and every ancestor up to root.
                    while buffer.len() > 1 {
                        dir_sizes.push(total);
                        total += buffer.pop().expect("buffer length checked");
                    }
                }
                Command::CdOut => {
                    if buffer.len() <= 1 {
                        return Err(AocError::Invalid("`cd ..` above root".to_string()));
                    }
                    dir_sizes.push(total);
                    total += buffer.pop().expect("buffer length checked");
                }
                Command::CdIn => {
                    buffer.push(total);
                    total = 0;
                }
                Command::Ls(content) => {
                    for c in content.iter() {
                        if let Content::File(v) = c {
                            total += v;
                        }
                    }
                }
            }
        }
        while let Some(n) = buffer.pop() {
            dir_sizes.push(total);
            total += n;
        }

        let p1 = dir_sizes.iter().filter(|&&n| n <= P1_MAX_DIR_SIZE).sum();

        let used = *dir_sizes.last().ok_or(AocError::NoSolution)?;
        let unused = P2_TOTAL_SPACE_AVAILABLE.checked_sub(used).ok_or_else(|| {
            AocError::Invalid(format!(
                "used space {used} exceeds filesystem size {P2_TOTAL_SPACE_AVAILABLE}"
            ))
        })?;
        let space_required = P2_MIN_UNUSED_SPACE.saturating_sub(unused);

        let p2 = *dir_sizes
            .iter()
            .filter(|&&n| n >= space_required)
            .min()
            .ok_or(AocError::NoSolution)?;

        Ok((p1, p2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "$ cd /
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

    const TEST_CUSTOM_1: &str = "$ cd /
$ ls
dir b
dir c
dir d
dir e
dir g
dir f
dir x
dir p
$ cd b
$ ls
100000 bfile
$ cd /
$ cd c
$ ls
dir c1
60000 cfile
$ cd c1
$ ls
450000 c1file
$ cd ..
$ cd /
$ cd d
$ ls
30000 dfile
$ cd /
$ cd e
$ ls
1000 efile
$ cd /
$ cd g
$ ls
100001 gfile
$ cd /
$ cd f
$ ls
39258222 ffile
$ cd /
$ cd x
$ ls
777 xroot
$ cd /
$ cd p
$ ls
dir q
dir x
100 pfile
$ cd q
$ ls
200 qfile
$ cd ..
$ cd x
$ ls
111 xfile
$ cd ..
$ cd /";

    #[test]
    fn part1_2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        let (p1, p2) = solution_data.solve().unwrap();
        assert_eq!(p1, 95437);
        assert_eq!(p2, 24933642);
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        let (p1, p2) = solution_data.solve().unwrap();
        assert_eq!(p1, 132499);
        assert_eq!(p2, 411);
    }

    #[test]
    fn part2_already_enough_free_space() {
        let data: InputData = "$ cd /\n$ ls\n1 a".parse().unwrap();
        assert_eq!(data.solve().unwrap(), (1, 1));
    }

    #[test]
    fn part2_used_exceeds_capacity() {
        let data: InputData = "$ cd /\n$ ls\n70000001 a".parse().unwrap();
        assert!(matches!(data.solve(), Err(AocError::Invalid(_))));
    }
}
