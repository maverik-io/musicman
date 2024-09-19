use std::fs;
use std::io::BufReader;

use colored::Colorize;

pub fn play(current_index: usize, queue: &Vec<String>, sink: &rodio::Sink) {
    sink.clear();
    let song = queue[current_index].clone();
    let file = std::fs::File::open(&song).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    sink.play();
}

pub fn pretty_print(data: &Vec<String>, title: &str, selected: Option<usize>) {
    let mut index = -1;
    if selected.is_some() {
        index = selected.unwrap() as i32;
    }
    let mut data_cpy = data.clone();
    data_cpy.push(title.to_string());

    let maxlen = data_cpy
        .iter()
        .fold(0, |t, v| if v.len() > t { v.len() } else { t });
    println!("╭{title:─<len$}╮", len = maxlen + 6);
    let mut c = -1;
    for s in data {
        c += 1;
        let mut playing = "  ";
        if c == index {
            playing = "|>"
        }
        println!("│ {} {:<maxlen$}  │", playing.yellow(), s.blue());
    }
    println!("╰{}╯", "─".repeat(maxlen + 6));
}
pub fn show_playlists() {}
pub fn set_playlist(queue: &Vec<String>, name: String) {}
pub fn index_all(root: String) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for entry in fs::read_dir(&root).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            let new_root = root.clone() + "/" + (entry.file_name().into_string().unwrap().as_str());
            // println!("{new_root}");
            for elem in index_all(new_root) {
                out.push(elem);
            }
        } else {
            out.push(entry.path().to_str().unwrap().to_string());
        }
    }
    out
}

pub fn search(names_in: &Vec<String>, target: &String) -> Vec<String> {
    //println!("Searching for {target:?}");
    let mut names = names_in.clone();
    if names.contains(&target) {
        return vec![target.clone()];
    } else {
        let mut found = false;
        let mut index = 1;
        while !found {
            let mut searchlist: Vec<String> = Vec::new();
            let searchstr = &target[..index];
            for name in &names {
                let name_short = name.split("/").last().unwrap().to_lowercase();
                // println!("{name_short}");
                if name_short.len() > index {
                    if &name_short[..index] == searchstr {
                        searchlist.push(name.clone());
                    }
                }
            }
            // println!("{:?}", &searchlist);

            if searchlist.len() == 0 {
                break;
            }

            if index > target.len() - 1 {
                if searchlist.len() == 1 {
                    found = true;
                } else {
                    return searchlist;
                }
            }

            names = searchlist.clone();
            index += 1;
        }
        if found {
            names
        } else {
            vec![]
        }
    }
}
