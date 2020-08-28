extern crate cursive;
extern crate mpv;
extern crate curl;
extern crate serde_json;

mod twitch;
mod use_mpv;
mod ui;

use cursive::Cursive;
use cursive::CursiveExt;

fn main() {
    let mut siv : Cursive = Cursive::default();

    ui::construct_ui(&mut siv);

    siv.run();
}
