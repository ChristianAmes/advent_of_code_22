use std::env;
use std::fs;


fn main() {


    let args: Vec<String> = env::args().collect();
    
    let file_path = &args[2];

    println!("In file {}", file_path);


    let contents = fs::read_to_string(file_path)
            .expect("No file");

    let mut max:u128 = 0;
    let mut moving_sum:u128 = 0;
    for line in contents.lines(){
        
        if line != "" {
            moving_sum += line.parse::<u128>().unwrap();
            
            println!(": {}",line);
        }else{ 
            if moving_sum > max {
                max = moving_sum;
            }
            moving_sum = 0;
        }

    }


    println!("Maximum : {}", max);


}


