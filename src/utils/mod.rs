use colored::Colorize;
use core::time;
use dirs::home_dir;
use std::io::Write;
use std::io::{stdin, stdout};
use std::process::exit;
use std::sync::mpsc;
use std::thread::sleep;
use std::usize;
mod handlers;

pub fn init(rx: mpsc::Receiver<String>) {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    let mut queue: Vec<String> = Vec::new();
    let all_songs = handlers::index_all(
        home_dir()
            .unwrap()
            .join("Music")
            .to_str()
            .unwrap()
            .to_string(),
    );
    let mut current_index = 0;
    loop {
        match rx.try_recv() {
            Ok(recieved) => {
                //println!("player: `{recieved}` recieved");
                let recieved_split = recieved
                    .split_ascii_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();

                match recieved_split[0].as_str() {
                    "add" => {
                        let blob = recieved_split[1..]
                            .iter()
                            .fold(String::new(), |t, v| format!("{t} {v}"))
                            .trim()
                            .to_string();
                        let songs = handlers::search(&all_songs, &blob);
                        if songs.len() != 1 {
                            if songs.len() == 0 {
                                println!(
                                    "{} {}",
                                    "No match found for".yellow(),
                                    blob.yellow().bold()
                                );
                            } else {
                                println!(
                                    "{}",
                                    "Found multiple matches, which do I add? :".yellow()
                                );
                                let mut c = 0;
                                for i in &songs {
                                    let i = i.split('/').last().unwrap();
                                    c += 1;
                                    println!("  {c}) {}", i.yellow().bold());
                                }
                                println!("  *) {}", "Add nothing...".yellow().bold());
                                println!("{}", "Enter a number: ".yellow());
                                let ch = rx.recv().unwrap();
                                match ch.parse::<usize>() {
                                    Ok(val) => {
                                        if val == 0 {
                                            println!("{}", "Adding nothing".yellow().italic())
                                        } else {
                                            let val = val - 1;
                                            if val > songs.len() {
                                                println!("{}", "Adding nothing".yellow().italic());
                                            } else {
                                                println!(
                                                    "{} {}",
                                                    "Adding to queue:".green().italic(),
                                                    songs[val].green().italic()
                                                );
                                                queue.push(songs[val].clone());
                                            }
                                        }
                                    }
                                    Err(_) => println!("{}", "Adding nothing".yellow().italic()),
                                }
                            }
                        } else {
                            println!(
                                "{} {}",
                                "Added to queue:".green(),
                                songs[0].green().italic()
                            );
                            queue.push(songs[0].clone());
                        }
                    }
                    "replay" => {
                        handlers::play(current_index, &queue, &sink);
                        println!("{}", "Replaying...".yellow().italic());
                    }
                    "play" | "pause" | "p" => {
                        if sink.is_paused() {
                            sink.play();
                            println!("{}", "Playing...".yellow().italic());
                        } else {
                            sink.pause();
                            println!("{}", "Paused.".yellow().italic());
                        }
                    }
                    "clear" => {
                        queue.clear();
                        println!("{}", "Queue cleared".yellow().italic());
                    }
                    "next" => {
                        if queue.len() != 0 {
                            let mut n = 1;
                            if recieved_split.len() > 1 {
                                match recieved_split[1].parse::<usize>() {
                                    Ok(num) => n = num,
                                    Err(_e) => {
                                        println!(
                                            "{}",
                                            "next: argument needs to be an positive integer".red()
                                        );
                                        n = 0;
                                    }
                                }
                            }
                            current_index = (current_index + n) % queue.len();
                            println!("{}", "Playing Next...".yellow().italic());
                            handlers::play(current_index, &queue, &sink);
                        } else {
                            println!("{}", "Nothing in queue".yellow().italic());
                        }
                    }
                    "prev" => {
                        if queue.len() != 0 {
                            let mut n = 1;
                            if recieved_split.len() > 1 {
                                match recieved_split[1].parse::<usize>() {
                                    Ok(num) => n = num,
                                    Err(_e) => {
                                        println!(
                                            "{}",
                                            "prev: argument needs to be an positive integer".red()
                                        );
                                        n = 0;
                                    }
                                }
                            }
                            if (current_index as i32 - n as i32) < 0 {
                                current_index =
                                    queue.len() - (-(current_index as i32 - n as i32) as usize);
                            } else {
                                current_index -= n;
                            }
                            println!("{}", "Playing Next...".yellow().italic());
                            handlers::play(current_index, &queue, &sink);
                        } else {
                            println!("{}", "Nothing in queue".yellow().italic());
                        }
                    }
                    "exit" => {
                        println!("{}", "Exiting...".yellow().italic());
                        exit(0);
                    }
                    "show" | "ls" => {
                        if queue.len() != 0 {
                            if recieved_split.len() == 2 {
                                match recieved_split[1].as_str() {
                                    "current" | "cp" => handlers::pretty_print(
                                        &vec![queue[current_index]
                                            .clone()
                                            .split('/')
                                            .last()
                                            .unwrap()
                                            .to_string()],
                                        "Current",
                                        Some(0),
                                    ),
                                    cmd => println!(
                                        "{} {}",
                                        "show: Unknown Command".red(),
                                        cmd.red().bold()
                                    ),
                                }
                            } else {
                                handlers::pretty_print(
                                    &queue
                                        .iter()
                                        .map(|s| s.split('/').last().unwrap().to_string())
                                        .collect(),
                                    "Queue",
                                    Some(current_index),
                                )
                            }
                        } else {
                            println!("{}", "Nothing in queue".yellow().italic());
                        }
                    }
                    "playlist" | "pl" => {
                        if recieved_split.len() < 2 {
                            println!("{}", "playlist: Insufficient arguments".red());
                            println!("{}", "playlist <new|load|show> [name]".yellow().italic());
                        } else {
                            match recieved_split[1].as_str() {
                                "new" => {
                                    if recieved_split.len() < 2 {
                                        println!(
                                            "{}",
                                            "playlist: new: Insufficient arguments".red()
                                        );
                                    } else if queue.len() > 0 {
                                        handlers::make_playlist(&queue, recieved_split[2].clone());
                                    }
                                }
                                "show" | "ls" => {
                                    handlers::show_playlists();
                                }
                                "load" => {
                                    if recieved_split.len() < 2 {
                                        println!(
                                            "{}",
                                            "playlist: load: Insufficient arguments".red()
                                        );
                                    } else {
                                        println!(
                                            "{} {}",
                                            "Playing from playlist".green(),
                                            recieved_split[2].green().bold()
                                        );
                                        let new_queue = handlers::load_playlist(
                                            recieved_split[2].clone() + ".list",
                                        );
                                        if new_queue.len() > 0 {
                                            queue = new_queue.clone();
                                            handlers::pretty_print(
                                                &queue
                                                    .iter()
                                                    .map(|s| {
                                                        s.split('/')
                                                            .last()
                                                            .unwrap()
                                                            .trim()
                                                            .to_string()
                                                    })
                                                    .collect(),
                                                recieved_split[2].as_str(),
                                                None,
                                            );
                                            current_index = 0;
                                            handlers::play(current_index, &queue, &sink)
                                        }
                                    }
                                }
                                cmd => {
                                    println!(
                                        "{} {}",
                                        "playlist: Unknown command".red(),
                                        cmd.red().bold()
                                    );
                                    println!("{}", "playlist <new|load> [name]".yellow().italic());
                                }
                            }
                        }
                    }

                    cmd => {
                        println!("{} {}", "Unknown command".red(), cmd.red().bold());
                        println!(
                            "{}",
                            "<add|clear|exit|next|p|playlist|prev|replay|show>"
                                .yellow()
                                .italic(),
                        );
                    }
                }
            }
            Err(_) => {
                if sink.empty() && queue.len() != 0 {
                    current_index = (current_index + 1) % queue.len();
                    handlers::play(current_index, &queue, &sink);
                }
            }
        }
    }
}

pub fn commandline(tx: mpsc::Sender<String>) {
    std::thread::spawn(move || loop {
        sleep(time::Duration::from_millis(10));
        print!("{}", "musicman❯ ".green().bold());
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        //println!("commandline: `{input}` sent");
        tx.send(input).unwrap();
    });
}
