use rand::Rng;

mod cards;
fn main() {
    // Specify the path to the file you want to open
    let  files = cards::get_files("cards");

    match files {
        Ok(files) => run(files),
        Err(_) => panic!("Error occured")
    }
}


fn run(mut files: Vec<String>) {
    let number_of_cards = files.len();
    let mut completed = 0;

    loop {

        if files.len() < 1 {
            break;
        }
        println!("----------------------");
        println!("🦍 Completed {}/{} ✅",completed,number_of_cards);
        println!("----------------------");
 
        let index = rand::thread_rng().gen_range(0..=files.len() -1);

        let current_file = &files[index];

        let file_name = &current_file.to_string()[6..];
        println!("🟠: {}", file_name);


        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "s" {
            completed += 1;
            continue;
        }

        cards::open_file(String::from(current_file));

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "x" {
            continue;
        }

        files.remove(index);
        completed += 1;
        

    }
}


