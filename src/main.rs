use clap::{App, AppSettings, Arg};
use regex_automata::{dense, DFA};
use std::collections::{hash_map, HashMap};
use std::{error, fs, io, path};

fn walk<T: DFA>(
    dfa: &T,
    state_to_dir: &mut HashMap<T::ID, Box<path::Path>>,
    state: T::ID,
    path: path::PathBuf,
) -> io::Result<()> {
    if dfa.is_match_state(state) {
        fs::File::create(path.join("ACCEPT"))?;
    }
    for input in (32..=45).chain(48..=126) {
        let current = dfa.next_state(state, input);
        if !dfa.is_dead_state(current) {
            let new_path = path.join(&String::from_utf8_lossy(&[input]).into_owned());
            match state_to_dir.entry(current) {
                hash_map::Entry::Occupied(entry) => {
                    let mut original = pathdiff::diff_paths(entry.get(), path.as_path()).unwrap();
                    if original.to_str().unwrap().is_empty() {
                        original.push(".");
                    }
                    std::os::unix::fs::symlink(original, new_path)?;
                }
                hash_map::Entry::Vacant(entry) => {
                    fs::create_dir(new_path.as_path())?;
                    entry.insert(new_path.to_owned().into_boxed_path());
                    walk(dfa, state_to_dir, current, new_path)?;
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
    state_to_dir.insert(
        dfa.start_state(),
        path::PathBuf::from(root).into_boxed_path(),
    );
    walk(
        &dfa,
        &mut state_to_dir,
        dfa.start_state(),
        path::PathBuf::from(root),
    )?;
    Ok(())
}
