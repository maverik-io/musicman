#[derive(Debug)]
pub enum Command {
    Add(String),
    Clear,
    Exit,
    Next(usize),
    Prev(usize),
    P,
    Playlist(PlaylistCommand),
    Replay,
    Show,
    Unknown,
}

#[derive(Debug)]
pub enum PlaylistCommand {
    New(String),
    Load(String),
    Show,
    Unknown,
}

#[derive(Debug)]
pub enum PlayerCommand {
    Play(String),
    TogglePlayState,
}
