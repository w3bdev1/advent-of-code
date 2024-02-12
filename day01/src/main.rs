fn main() {
   if let Some(lines) = read_file_as_lines("./data") {
       let mut sum = 0;
       for (index, line) in lines.iter().enumerate() {
           if let Some((first_digit, last_digit)) = get_first_and_last_digit(line) {
               let value = 10*first_digit + last_digit;
               sum += value
           } else {
               println!("No number on line {}", index+1);
           }
       }

       println!("Sum of all of the calibration values: {sum}");
   }
}

fn get_first_and_last_digit(line: &str) -> Option<(u32, u32)> {
    let mut digits: Vec<u32> = vec![];

    for c in line.chars() {
        if let Some(num) = c.to_digit(10) {
            digits.push(num);
        }
    }

    match digits.len() {
        0 => return None,
        1 => return Some((digits[0], digits[0])),
        n => return Some((digits[0], digits[n-1]))
    }
}

fn read_file_as_lines(path: &str) -> Option<Vec<String>> {
    let data = std::fs::read_to_string(path);
    match data {
        Ok(data) => {
            let lines = data
                .lines()
                .map(|line| line.to_string())
                .collect();
            return Some(lines);
        }
        Err(e) => {
            eprintln!("Error: {e}");
            return None;
        }
    }
}
