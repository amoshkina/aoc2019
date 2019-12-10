use std::fs::read_to_string;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;


fn count_num(layer: &Vec<i32>, num: i32) -> usize {
    layer.iter().filter(|&&item| item == num).collect::<Vec<&i32>>().len()
}


fn main() -> MyResult<()> {
    let layer_len: usize = 25 * 6;
    let data: &str = &read_to_string("src/input.txt")?;

    let mut from: usize = 0;
    let mut to = layer_len;

    let mut min_zeros = layer_len;
    let mut result: usize = 0;

    while from < data.len() {
        let layer: Vec<i32> = data[from..to].chars().map(|ch| ch.to_string().parse::<i32>().unwrap()).collect();
        let zeros = count_num(&layer, 0);
        if zeros < min_zeros {
            min_zeros = zeros;
            result = count_num(&layer, 1) * count_num(&layer, 2);
        }
        from = to;
        to = from + layer_len;
    
    }

    println!("Result Part 1: {:?}", result);
    Ok(())
}
