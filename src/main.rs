
mod twitch;
mod use_mpv;
mod ui;
mod config;

use cursive::Cursive;
use cursive::CursiveExt;

fn main() {

    let mut siv : Cursive = Cursive::default();

    ui::construct_ui(&mut siv);

    siv.run();
}
