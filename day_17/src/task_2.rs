use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fs;
use std::path::Path;


pub fn task_2() {
    const TOTAL_ROCKS: usize = 1000000000000;
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let chars: Vec<char> = contents.chars().collect();
    let shapes = ['-', '+', 'L', '|', '▫'];
    let mut arr = [0u8; 2000000];
    let mut time = 0;
    let mut rocks = 0;
    let mut height = 0;
    let mut cycle_height: i128 = 0;
    let mut seen: HashMap<(u128, usize, char), (i32,i32)> = HashMap::new();
    while rocks < TOTAL_ROCKS {
        let mut offset:i32 = 3;
        let mut shape = [0; 4];
        /*
        0000001 << 4 = 0010000
        0000001 << 4 = 0010000
        0000001 << 4 = 0010000
        0000001 << 4 = 0010000

        0000001 << 3 = 0001000
        0000111 << 2 = 0011100
        0000001 << 3 = 0001000

        0000001 << 2 = 0000100
        0000001 << 2 = 0000100
        0000111 << 2 = 0011100

        0000011 << 3 = 0011000
        0000011 << 3 = 0011000

        0001111 << 1 = 0011110
        */
        match shapes[rocks % shapes.len()] {
            '|' => shape = [1 << 4; 4],
            '+' => shape = [1 << 3, 7 << 2, 1 << 3, 0],
            'L' => shape = [7 << 2, 1 << 2, 1 << 2, 0],
            '▫' => shape = [3 << 3, 3 << 3, 0, 0],
            '-' => shape = [15 << 1, 0, 0, 0],
            _ => ()
        }
        loop {
            match chars.get(time % chars.len()).unwrap() {
                '<' => {
                    if !shape.iter().any(|x| x & (1 << 6) != 0) && 
                    !(0..4).any(|i| shape[i] << 1 & arr[(height + offset) as usize + i] != 0)
                    {
                        shape = shape.map(|x| x << 1);
                    }
                }
                '>' => {
                    if !shape.iter().any(|x| x & 1 != 0) && 
                    !(0..4).any(|i| shape[i] >> 1 & arr[(height + offset) as usize + i] != 0)
                    {
                        shape = shape.map(|x| x >> 1);
                    }
                }
                _ => ()
            }
            if height as i32 + offset == 0 || (0..4).any(|i| shape[i] & arr[i + (height + offset - 1) as usize] != 0){
                for i in 0..4 {
                    arr[(height + offset + i) as usize] = arr[(height + offset + i) as usize] | shape[i as usize];
                }
                match shapes[rocks % shapes.len()]{
                    '|' => height = height.max(height + offset + 4),
                    '+' => height = height.max(height + offset + 3),
                    'L' => height = height.max(height + offset + 3),
                    '▫' => height = height.max(height + offset + 2),
                    '-' => height = height.max(height + offset + 1),
                    _ => ()
                }
                time += 1;
                break;
            }
            offset -= 1;
            time += 1;
        }
        // let re: &[u8] = &(arr[0..(1+height as usize)]);
        // for l in re.iter().rev() {
        //     println!("{:#09b}", l);
        // }
        // println!("");
        // println!("");
        
        rocks += 1;

        if height < 16{
            continue;
        } 
        let mut top: u128 = 0;
        for i in 0..16 {
            top = top | (arr[(height-i) as usize] as u128) << (i*8);
        }
        match seen.entry((top, time % chars.len(), shapes[rocks % shapes.len()])) {
            Entry::Occupied(e) => {
                let (old_n, old_height) = e.get();
                let num_rocks_in_cycle = rocks as i128 - *old_n as i128;
                let num_cycles = (TOTAL_ROCKS - rocks) as i128 / num_rocks_in_cycle;
                rocks += (num_rocks_in_cycle * num_cycles) as usize;
                cycle_height += num_cycles * (height - old_height) as i128;
                seen.clear();
            }
            Entry::Vacant(e) => {
                e.insert((rocks as i32, height));
            }
        }

    }

    println!("{}", cycle_height + height as i128);
}
