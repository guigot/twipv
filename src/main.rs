extern crate cursive;
extern crate mpv;
extern crate curl;
extern crate serde_json;
extern crate rusqlite;

// use mpv::mpv;
use std::path::Path;
use std::string::String;
use curl::easy::{Handler, Easy2, List, WriteError};
use serde_json::{Value};
use std::str;
use cursive::Cursive;
use cursive::align::HAlign;
use cursive::theme::{Color, PaletteColor};
use cursive::event::EventResult;
use cursive::views::{Dialog,OnEventView,SelectView};

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

fn retrieve_videos() -> String {

    let mut easy = Easy2::new(Collector(Vec::new()));
    easy.get(true).unwrap();
    easy.url("https://api.twitch.tv/kraken/channels/28575692/videos?limit=10").unwrap();

    let mut list = List::new();
    list.append("Accept: application/vnd.twitchtv.v5+json").unwrap();
    list.append("Client-ID: ja58d80v5sp3m5y3p6kw068xuq49pw").unwrap();

    easy.http_headers(list).unwrap();
    easy.perform().unwrap();
    let contents = easy.get_ref();
    String::from_utf8_lossy(&contents.0).to_string()

}

fn launch_video(video_path: &Path) {
    let mut mpv_builder = mpv::MpvHandlerBuilder::new().expect("Failed to init MPV builder");
    if video_path.is_file() || !video_path.is_file() {
        let video_path = video_path.to_str().expect("Expected a string for Path, got None");

        // enable On Screen Controller (disabled with libmpv by default)
        mpv_builder.set_option("osc",true).unwrap();
        mpv_builder.set_option("save-position-on-quit",true).unwrap();
        mpv_builder.set_option("watch-later-directory","/home/exosta/.config/mpv/watch_later").unwrap();
        mpv_builder.set_option("input-default-bindings",true).unwrap();
        mpv_builder.set_option("input-vo-keyboard",true).unwrap();

        let mut mpv = mpv_builder.build().expect("Failed to build MPV handler");

        mpv.command(&["loadfile", video_path as &str])
           .expect("Error loading file");

        // loop twice, send parameter as a string
        mpv.set_property("loop","1").unwrap();

        // set speed to 100%, send parameter as a f64
        mpv.set_property("speed",1.0).unwrap();

        'main: loop {
            while let Some(event) = mpv.wait_event(0.0) {
                // even if you don't do anything with the events, it is still necessary to empty
                // the event loop
                match event {
                    mpv::Event::Shutdown | mpv::Event::Idle => {
                        break 'main;
                    }
                    _ => {}
                };
            }
        }
    } 
}


fn callback_video(_siv: &mut Cursive, url: &str) {
    launch_video(Path::new(url));
}

fn main() {


    let result : String = retrieve_videos();
    let val: Value = serde_json::from_str(&result).unwrap();

    let mut select_view = SelectView::new()
        .h_align(HAlign::Left);
    select_view.set_on_submit(callback_video);


    for _i in 0..10 {
        let mut plain_title : String = val["videos"][_i]["title"].as_str().unwrap().to_string();
        let size = 70;
        if plain_title.chars().count() > size {
            plain_title = plain_title.chars().take(size-3).collect();
            plain_title.push_str("...");
        }
        else {
            let fill = " ".repeat(size - plain_title.chars().count());
            plain_title.push_str(fill.as_str()); 
        }
                                                       
        let mut line_str = plain_title;
        line_str.push_str("  ");
        let game = val["videos"][_i]["game"].as_str().unwrap();
        line_str.push_str(game);

        select_view.add_item(line_str,
                            val["videos"][_i]["url"].as_str().unwrap().to_string());
    }

    let select_view = OnEventView::new(select_view)
        .on_pre_event_inner('k', |s, _| {
            s.select_up(1);
            Some(EventResult::Consumed(None))
        }).on_pre_event_inner('j', |s, _| {
            s.select_down(1);
            Some(EventResult::Consumed(None))
        });

    let mut siv = Cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    let mut theme = siv.current_theme().clone();
    theme.palette[PaletteColor::Background] = Color::TerminalDefault;
    siv.set_theme(theme);

    siv.add_layer(
        Dialog::around(select_view)
            .title("Derniers streams MV")
    );

    siv.run();


}
