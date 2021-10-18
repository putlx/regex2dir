use clap::{App, AppSettings, Arg};
use regex_automata::{dense, DFA};
use std::boxed::Box;
use std::collections::{hash_map, HashMap};
use std::path::{Path, PathBuf};
use std::{env, error, fs, io};

fn walk<T: DFA>(
    dfa: &T,
    state_to_dir: &mut HashMap<T::ID, Box<Path>>,
    state: T::ID,
    path: PathBuf,
) -> io::Result<()> {
    if dfa.is_match_state(state) {
        fs::File::create(path.join("ACCEPT"))?;
    }
    for input in (32..=45).chain(48..=126) {
        let current = dfa.next_state(state, input);
        if !dfa.is_dead_state(current) {
            match state_to_dir.entry(current) {
                hash_map::Entry::Occupied(entry) => {
                    let original = entry.get().canonicalize()?;
                    let current_dir = env::current_dir()?;
                    env::set_current_dir(path.as_path())?;
                    std::os::unix::fs::symlink(
                        original,
                        String::from_utf8_lossy(&[input]).into_owned(),
                    )?;
                    env::set_current_dir(current_dir)?;
                }
                hash_map::Entry::Vacant(entry) => {
                    let path = path.join(&String::from_utf8_lossy(&[input]).into_owned());
                    fs::create_dir(path.as_path())?;
                    entry.insert(path.to_owned().into_boxed_path());
                    walk(dfa, state_to_dir, current, path)?;
                }
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let matches = App::new("regex2dir")
        .setting(AppSettings::DisableVersion)
        .arg(
            Arg::with_name("anchor")
                .short("a")
                .long("anchor")
                .help("Anchor regex at beginning"),
        )
        .arg(Arg::with_name("PATTERN").required(true).index(1))
        .arg(Arg::with_name("DIRECTORY").required(true).index(2))
        .get_matches();
    let dfa = dense::Builder::new()
        .anchored(matches.is_present("anchor"))
        .build(matches.value_of("PATTERN").unwrap())?;
    let root = matches.value_of("DIRECTORY").unwrap();
    fs::create_dir(root)?;
    let mut state_to_dir = HashMap::new();
    state_to_dir.insert(dfa.start_state(), PathBuf::from(root).into_boxed_path());
    walk(
        &dfa,
        &mut state_to_dir,
        dfa.start_state(),
        PathBuf::from(root),
    )?;
    Ok(())
}
