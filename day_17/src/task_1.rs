use std::fs;
use std::path::Path;


pub fn task_1() {
    const TOTAL_ROCKS: usize = 2022;
    let contents = fs::read_to_string(Path::new("./input1"))
        .expect("Should have been able to read the file");
    let mut arr = [0u8; TOTAL_ROCKS*4];
    let chars: Vec<char> = contents.chars().collect();
    let shapes = ['-', '+', 'L', '|', '▫'];
    let mut time = 0;
    let mut rocks = 0;
    let mut height = 0;
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
    }

    println!("{}", height);
}
