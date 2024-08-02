mod gag;
use gag::{display_result, increment_gag, GagTrack}; // Ensure GagName is imported if needed

use rand::rngs::ThreadRng;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::io::{self, Write};
fn main() {
    let mut input_buff = String::new();
    
    print_intro();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input_buff).unwrap();
        
        if input_buff.trim().eq("quit") {
            print!("Quitting... Good Bye!");
            io::stdout().flush().unwrap();
            return
        }
        let user_result = input_buff.trim().parse::<u8>();
        match user_result {
            Ok(n) if n < 13 => calculate_gag_build(n+4),
            Ok(_) => eprintln!("Out of range."),
            Err(_e) => eprintln!("Not a valid number.")
        }
                
        input_buff.clear(); //clear the buffer for new input
    }
         
}
    
    

fn calculate_gag_build(mut training_points: u8) {
    /*
        Initialize gag list + random thread generator
     */
    let mut gag_tracks: Vec<GagTrack> = GagTrack::new();
    let mut rng: ThreadRng = thread_rng();

    while training_points > 0 {

        /*
        Edge case where you have 1 training point left and attempt to use it to prestige a gag
         */
        if training_points == 1 {
            let unprestiged_gags: Vec<GagTrack> = gag_tracks.iter()
            .filter(|g| g.count == 2)
            .cloned()
            .collect();
            let chosen_gag_result = unprestiged_gags.choose(&mut rng);
            //prestige the gag if it exists, otherwise do nothing with it 
            match chosen_gag_result {
                Some(gag) => {
                    let _ = increment_gag(&mut gag_tracks, gag.name.clone());
                }
                None => ()
            }
            break
        }

        /*
            Gets all available gags (meaning any that you can spend points on)
         */
        let available_gags: Vec<GagTrack> = gag_tracks.iter()
            .filter(|g| g.count < 3)
            .cloned()
            .collect();
        
        //get that gag track and incremenet it based on whether it's unlocked or not
        let picked_gag_track = available_gags.choose(&mut rng).unwrap();
        let cost: u8 = increment_gag(&mut gag_tracks, picked_gag_track.name.clone()); // Ensure `increment_gag` uses `gag` field
        training_points -= cost;
    }

    // Finally display result
    display_result(gag_tracks);
}

fn print_intro() {
    println!("%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%");
    println!("                      Hello and welcome to Benanite's Gag Randomizer!");
    println!("                                 ##### INSTRUCTIONS #####");
    println!("I print out a gag build for you based on how many training points your toon has unlocked.");
    println!("You can enter the number of training points unlocked below.");
    println!("I only accept numbers within the range 0 to 12 inclusive.");
    println!("To quit the program, enter 'quit'.");
    println!("%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%");
}
