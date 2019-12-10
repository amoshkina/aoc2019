use std::fs::read_to_string;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const LAYER_LEN: usize = WIDTH * HEIGHT;

fn count_num(layer: &Vec<i32>, num: i32) -> usize {
    layer.iter().filter(|&&item| item == num).collect::<Vec<&i32>>().len()
}


fn part1(data: &str) -> usize {
    let mut from: usize = 0;
    let mut to = LAYER_LEN;

    let mut min_zeros = LAYER_LEN;
    let mut result: usize = 0;

    while from < data.len() {
        let layer: Vec<i32> = data[from..to].chars().map(|ch| ch.to_string().parse::<i32>().unwrap()).collect();
        let zeros = count_num(&layer, 0);
        if zeros < min_zeros {
            min_zeros = zeros;
            result = count_num(&layer, 1) * count_num(&layer, 2);
        }
        from = to;
        to = from + LAYER_LEN;
    
    }
    result
}

fn print_image(image: &mut Vec<char>) {
    println!("image len - {:?}", image.len());
    let mut from: usize = 0;
    let mut to = WIDTH;

    while to <= image.len() {
        let layer: String = image[from..to].iter().map(|&ch| if ch == '0' { "  ".to_string() } else { "0 ".to_string() }).collect();
        println!("{:?}", layer);
        from = to;
        to = from + WIDTH;
    }
}


fn part2(data: &str) {
    let data: Vec<char> = data.chars().collect();
    println!("data - {:?}, layer - {:?}", data.len(), LAYER_LEN);
    let mut image: Vec<char> = vec![];
    let mut start: usize = 0;
    while start < LAYER_LEN {
        let mut current: usize = start;
        while data[current] == '2' && current < data.len() {
            current += LAYER_LEN;
        }
        image.push(data[current]);
        start += 1;
    }

    print_image(&mut image)
    
}

fn main() -> MyResult<()> {
    let data: &str = &read_to_string("src/input.txt")?;

    println!("Result Part 1: {:?}", part1(data));
    println!("Result Part 2: ");
    part2(data);
    Ok(())
}
