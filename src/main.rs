extern crate cursive;
extern crate mpv;
extern crate curl;
extern crate serde_json;

mod twitch;
mod use_mpv;
mod ui;

use cursive::Cursive;

fn main() {
    let mut siv : Cursive = Cursive::default();

    ui::construct_ui(&mut siv);
    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}
