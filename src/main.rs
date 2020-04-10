extern crate cursive;
extern crate mpv;
extern crate curl;
extern crate serde_json;
extern crate rusqlite;

mod twitch;
mod use_mpv;
mod ui;

use std::string::String;
use curl::easy::{Handler, Easy2, List, WriteError};
use std::str;
use cursive::Cursive;
use serde_json::Value;
use cursive::theme::{Color, PaletteColor};
use cursive::traits::*;
use cursive::event::EventResult;
use cursive::views::{Dialog,OnEventView,DummyView,EditView,LinearLayout,TextView};

fn main() {

    
    let mut siv : Cursive = Cursive::default();

    ui::construct_ui(&mut siv);
    siv.add_global_callback('q', |s| s.quit());

    siv.run();


}
