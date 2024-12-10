use std::fmt::Display;

pub const TEST: &str = "2333133121414131402";

#[derive(Debug, Clone)]
enum Block {
    Free,
    File(usize),
}

fn read_disk(disk_map: &str) -> Vec<Block> {
    disk_map
        .chars()
        .filter(|c| c.is_ascii_digit())
        .enumerate()
        .flat_map(|(i, char)| {
            if i % 2 == 0 {
                vec![Block::File(i / 2); char.to_digit(10).unwrap() as usize]
            } else {
                vec![Block::Free; char.to_digit(10).unwrap() as usize]
            }
        })
        .collect()
}

fn checksum(blocks: &[Block]) -> usize {
    blocks
        .iter()
        .enumerate()
        .map(|(position, block)| match block {
            Block::Free => 0,
            Block::File(id) => position * id,
        })
        .sum()
}

pub fn part1(input: &str) -> anyhow::Result<String> {
    let mut blocks = read_disk(input);

    let (mut left, mut right) = (0, blocks.len() - 1);
    while left < right {
        while let Block::File(_) = blocks[left] {
            left += 1;
        }

        while let Block::Free = blocks[right] {
            right -= 1;
        }
        if left >= right {
            break;
        }

        blocks.swap(left, right);
    }

    Ok(format!("{}", checksum(&blocks)))
}

#[derive(Debug, Clone, Copy)]
enum DiskSection {
    Free(usize),
    File(usize, usize),
}

impl DiskSection {
    fn size(&self) -> usize {
        match self {
            Self::Free(size) => *size,
            Self::File(size, _) => *size,
        }
    }

    fn resize(&mut self, new_size: usize) {
        match self {
            Self::Free(size) => *size = new_size,
            Self::File(size, _) => *size = new_size,
        };
    }
}

impl Display for DiskSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Free(size) => f.write_str(&".".repeat(*size)),
            Self::File(size, id) => f.write_str(&id.to_string().repeat(*size)),
        }
    }
}

// fn print_sections(sections: &[DiskSection]) -> String {
//     sections.iter().fold(String::new(), |mut s, section| {
//         let _ = write!(s, "{section}");
//         s
//     })
// }

fn read_disk2(disk_map: &str) -> Vec<DiskSection> {
    disk_map
        .chars()
        .filter(|c| c.is_ascii_digit())
        .enumerate()
        .map(|(i, char)| {
            if i % 2 == 0 {
                DiskSection::File(char.to_digit(10).unwrap() as usize, i / 2)
            } else {
                DiskSection::Free(char.to_digit(10).unwrap() as usize)
            }
        })
        .collect()
}

fn collapse_free_sections(sections: &mut Vec<DiskSection>, index: usize) {
    let size = sections[index].size();
    if let Some(DiskSection::Free(left_size)) = sections.get_mut(index - 1) {
        *left_size += size;
        sections.remove(index);
    } else if let Some(DiskSection::Free(right_size)) = sections.get_mut(index + 1) {
        *right_size += size;
        sections.remove(index);
    }
}

fn sections_to_blocks(sections: Vec<DiskSection>) -> Vec<Block> {
    sections
        .into_iter()
        .flat_map(|section| match section {
            DiskSection::Free(size) => vec![Block::Free; size],
            DiskSection::File(size, id) => vec![Block::File(id); size],
        })
        .collect()
}

pub fn part2(input: &str) -> anyhow::Result<String> {
    let mut sections = read_disk2(input);
    let mut lowest_id = usize::MAX;
    for i in (0..sections.len()).rev() {
        if let DiskSection::File(file_size, id) = sections[i] {
            if id >= lowest_id {
                // Found a section we already dealt with
                continue;
            }

            lowest_id = id;

            let free_space = sections
                .iter()
                .enumerate()
                .find(|(free_i, section)| match section {
                    DiskSection::Free(free_size) => *free_size >= file_size && *free_i < i,
                    DiskSection::File(_, _) => false,
                })
                .map(|(i, section)| (i, *section));

            if let Some((free_section_index, free_section)) = free_space {
                if free_section.size() == file_size {
                    sections.swap(i, free_section_index);
                    collapse_free_sections(&mut sections, i);
                } else {
                    sections[free_section_index].resize(free_section.size() - file_size);
                    sections.insert(free_section_index, sections[i]);
                    sections[i + 1] = DiskSection::Free(file_size);
                    collapse_free_sections(&mut sections, i + 1);
                }
            }
        }
    }
    // println!("{}", print_sections(&sections));

    let blocks = sections_to_blocks(sections);
    // println!("{blocks:?}");
    Ok(format!("{}", checksum(&blocks)))
}
