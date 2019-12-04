fn main() {
    const RADIX: u32 = 10;
    let from: u32 = 256310;
    let to: u32 = 732736;

    let  mut counter: u32 = 0;
    for num in (from..to).map(|int| int.to_string()) {
        let mut chars = num.chars().map(|ch| ch.to_digit(RADIX).unwrap());

        let mut increasing: bool = true;

        let mut adjacent: bool = false;
        let mut adjacent_counter: u32 = 1;
        
        let mut prev: u32 = chars.next().unwrap();
        for curr in chars {
            if prev == curr {
                adjacent_counter += 1;
            } else if prev > curr {
                increasing = false;
                break;
            } else {  // prev < curr
                if adjacent_counter == 2 {
                    adjacent = true;
                }
                adjacent_counter = 1;
            }

            prev = curr;
        }

        if adjacent_counter == 2 {
            adjacent = true;
        }


        if adjacent && increasing {
            counter += 1;
        }
    }

    println!("Result Part 2: {:?}", counter);
}
