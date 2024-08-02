#[derive(Debug, PartialEq, Clone)]
pub enum GagName {
    TOONUP,
    TRAP,
    LURE,
    THROW, 
    SQUIRT, 
    ZAP, 
    SOUND,
    DROP
}
#[derive(Debug,Clone)]
pub struct GagTrack {
    pub name: GagName,
    pub count: u8,
}

impl GagTrack {
    pub fn new() -> Vec<GagTrack> {
        let v: Vec<GagTrack> = vec![
            GagTrack{name: GagName::TOONUP, count: 0},
            GagTrack{name: GagName::TRAP, count: 0},
            GagTrack{name: GagName::LURE, count: 0},
            GagTrack{name: GagName::THROW, count: 0},
            GagTrack{name: GagName::SQUIRT, count: 0},
            GagTrack{name: GagName::ZAP, count: 0},
            GagTrack{name: GagName::SOUND, count: 0},
            GagTrack{name: GagName::DROP, count: 0}
            ];
        return v;
    }
}

pub fn increment_gag(gag_list: &mut Vec<GagTrack>, name: GagName) -> u8 {
    let gag_track = gag_list.iter_mut().find(|g| g.name == name).unwrap();

    //increment by training point cost rule in-game: Cost 2 to unlock gag, 1 to prestige it  
    match gag_track.count {
        2 => {
            gag_track.count += 1;
            1
        },
        0 => {
            gag_track.count += 2;
            2
        },
        _ => 0
    } 
}

pub fn display_result(gag_list: Vec<GagTrack>) {
    println!("========== RESULT ==========");
    for gag in gag_list {
        let gag_name = get_name_as_str(gag.name);
        match gag.count {
            3 => println!("Prestige {}", gag_name),
            2 => println!("{}", gag_name),
            _ => () //do nothing if there's only 0 or 1 training point used 
        }
    }
    println!("============================");
}

fn get_name_as_str(name: GagName) -> &'static str {
    match name {
        GagName::TOONUP => "Toon-Up",
        GagName::TRAP => "Trap",
        GagName::LURE => "Lure",
        GagName::THROW => "Throw",
        GagName::SQUIRT => "Squirt",
        GagName::ZAP => "Zap",
        GagName::SOUND => "Sound",
        GagName::DROP => "Drop"
    }
}