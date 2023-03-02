use std::collections::HashMap;
use std::env;
use std::process;
use std::str::FromStr;

#[derive(Debug, Eq, Hash, PartialEq)]
enum Note {
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
    A,
    Bb,
    B,
}

impl FromStr for Note {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(Note::C),
            "Db" => Ok(Note::Db),
            "D" => Ok(Note::D),
            "Eb" => Ok(Note::Eb),
            "E" => Ok(Note::E),
            "F" => Ok(Note::F),
            "Gb" => Ok(Note::Gb),
            "G" => Ok(Note::G),
            "Ab" => Ok(Note::Ab),
            "A" => Ok(Note::A),
            "Bb" => Ok(Note::Bb),
            "B" => Ok(Note::B),
            _ => Err(()),
        }
    }
}

pub trait ToInt {
    fn to_int(&self) -> i32;
}

impl ToInt for Note {
    fn to_int(&self) -> i32 {
        match self {
            Note::C => 0,
            Note::Db => 1,
            Note::D => 2,
            Note::Eb => 3,
            Note::E => 4,
            Note::F => 5,
            Note::Gb => 6,
            Note::G => 7,
            Note::Ab => 8,
            Note::A => 9,
            Note::Bb => 10,
            Note::B => 11,
        }
    }
}

#[derive(Debug)]
struct Config {
    root: Note,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("Only one argument (root note) is supported");
        }
        let root = Note::from_str(&args[1]).unwrap();
        Ok(Config { root })
    }
}

fn main() {
    let notes_int = 0..12;
    let notes_str = [
        "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B",
    ];
    let notes = notes_str.iter().map(|e| Note::from_str(e).unwrap());
    let note2int: HashMap<i32, Note> = HashMap::from_iter(notes_int.zip(notes));

    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Wrong input: {err}");
        process::exit(1);
    });
    println!("the root note is {:?}", config.root);
    let major_triad = generate_major_triad(config.root, &note2int);
    println!("the major triad is {:#?}", major_triad);
}

fn generate_major_triad(root: Note, note_map: &HashMap<i32, Note>) -> [&Note; 3] {
    let root_int = root.to_int();
    let triad_notes_int = [root_int, (root_int + 4) % 12, (root_int + 7) % 12];
    println!("{:#?}", triad_notes_int);
    let triad_notes: [&Note; 3] =
        triad_notes_int.map(|e| note_map.get(&e).expect("value cannot be converted to note"));
    triad_notes
}
