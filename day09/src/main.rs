use std::fs::read_to_string;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Empty,
    FileBlock { id: usize },
}

impl Block {
    fn is_empty_block(&self) -> bool {
        match self {
            Self::Empty => true,
            _ => false
        }
    }

    fn get_file_id(&self) -> usize {
        match self {
            Self::FileBlock { id } => *id,
            _ => panic!("Not a file block!")
        }
    }
}

fn read_input(path: &str) -> std::io::Result<Vec<(usize, Block)>> {
    let data = read_to_string(path)?;
    Ok(data
        .chars()
        .enumerate()
        .map(|(idx, block)| {
            let block_count = (block as u8 - '0' as u8) as usize;
            if idx % 2 == 0 {
                (block_count, Block::FileBlock { id: idx / 2 })
            } else {
                (block_count, Block::Empty)
            }
        })
        .collect())
}

fn get_range_sum(from: usize, to: usize) -> usize {
    (to * (to + 1)) / 2 - (if from == 0 { 0 } else { (from * (from - 1)) / 2 })
}

fn compactify_disk_single_blocks(disk: &Vec<(usize, Block)>) -> Vec<(usize, Block)> {
    let mut compacted_disk = Vec::new();

    let mut right = disk.len() - 1;
    if disk[right].1.is_empty_block() {
        assert!(right > 0);
        right -= 1;
    }
    let mut remaining_right = disk[right].0;

    let mut left = 0;
    while left < right {
        match disk[left].1 {
            Block::FileBlock { .. } => {
                compacted_disk.push(disk[left])
            },
            Block::Empty => {
                let mut space = disk[left].0;

                while space > 0 {
                    let moved_blocks = std::cmp::min(space, remaining_right);
                    compacted_disk.push((moved_blocks, disk[right].1));

                    space -= moved_blocks;
                    remaining_right -= moved_blocks;

                    // move all blocks from the right-most file - skip to the next right-most file
                    if remaining_right == 0 {
                        if left + 1 >= right {
                            break;
                        }
                        right -= 2; // skip one empty block
                        remaining_right = disk[right].0;
                    }
                }
            }
        }
        
        left += 1;
    }

    if remaining_right > 0 {
        compacted_disk.push((remaining_right, Block::FileBlock { id: disk[right].1.get_file_id() }));
    }

    compacted_disk
}

fn calculate_checksum(disk: &Vec<(usize, Block)>) -> usize {
    let mut disk_idx = 0;
    let mut checksum = 0;

    for (cnt, block) in disk {
        if !block.is_empty_block() {
            checksum += get_range_sum(disk_idx, disk_idx + *cnt - 1) * block.get_file_id();
        }
        disk_idx += *cnt;
    }
    
    checksum
}

fn compactify_disk_entire_files(disk: &Vec<(usize, Block)>) -> Vec<(usize, Block)> {
    // index the files based on size
    let mut indexed_files: [VecDeque<usize>; 10] = std::array::from_fn(|_| VecDeque::new());
    for idx in 0..disk.len() {
        if !disk[idx].1.is_empty_block() {
            indexed_files[disk[idx].0].push_back(idx);
        }
    }

    let mut compacted_disk = Vec::new();
    for idx in 0..disk.len() {
        if let Block::FileBlock { .. } = disk[idx].1 {
            if let Some(&next) = indexed_files[disk[idx].0].front() {
                if next == idx {
                    compacted_disk.push(disk[idx]);
                    indexed_files[disk[idx].0].pop_front();
                    continue;
                }
            }

            // if not pushed, then the file was moved and we have an empty space here again!
        }

        let mut space = disk[idx].0;
        
        while space > 0 {
            let mut max_file_id = None;
            for file_size in 0..=space {
                if let Some(&idx) = indexed_files[file_size].back() {
                    max_file_id = match max_file_id {
                        Some(prev) => Some(std::cmp::max(prev, (idx, file_size))),
                        None => Some((idx, file_size))
                    };
                }
            }

            match max_file_id {
                Some((_, target_file_size)) => {
                    let moved_file_idx = indexed_files[target_file_size].pop_back().unwrap();
                    compacted_disk.push(disk[moved_file_idx]);
                    space -= target_file_size;
                },
                None => break
            }
        }

        if space > 0 {
            compacted_disk.push((space, Block::Empty));
        }
    }
 
    compacted_disk
}

fn main() -> std::io::Result<()> {
    // let disk = read_input("example.txt")?;
    // let disk = read_input("example2.txt")?;
    // let disk = read_input("example3.txt")?;
    let disk = read_input("input.txt")?;

    let start = std::time::Instant::now();
    let compacted_disk = compactify_disk_single_blocks(&disk);
    let task1 = calculate_checksum(&compacted_disk);
    let duration = start.elapsed();
    println!("Task 1: {task1} (took {:?})", duration);

    let start = std::time::Instant::now();
    let compacted_disk = compactify_disk_entire_files(&disk);
    let task2 = calculate_checksum(&compacted_disk);
    let duration = start.elapsed();
    println!("Task 2: {task2} (took {:?})", duration);

    Ok(())
}


/*

Example 2:

233313312141413140212

00...111...2...333.44.5555.6666.777.888899.(10)(10)

Compacted Result:
00(10)(10)91119882887333744755556666

=> 2351


Example 3:

12345

0..111....22222

Compacted Result:

022111222

=> 0 * 0 + (1 + 2) * 2 + (3 + 4 + 5) * 1 + (6 + 7 + 8) * 2 = 60 


*/