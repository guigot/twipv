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
use rusqlite::Connection;
use cursive::Cursive;
use cursive::align::HAlign;
use cursive::theme::{Color, PaletteColor};
use cursive::event::EventResult;
use cursive::views::{Dialog,OnEventView,SelectView};

const DB_POSITION: &str = "/home/exosta/programming/mpv_stream/positions.db";

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

fn save_in_database(path: &Path, time_pos: i64) {

    let connection = Connection::open(DB_POSITION).unwrap();

    connection.execute("INSERT OR REPLACE INTO t_positions (url, time_pos)
                       VALUES (
                       ?1, ?2);", &[&path.to_str().unwrap(), &time_pos.to_string().as_str()]).unwrap();
}

fn get_time_pos_database(path: &Path) -> i64 {

    let connection = Connection::open(DB_POSITION).unwrap();

    let result = connection.query_row("SELECT time_pos FROM t_positions WHERE
                         url = ?1;", &[&path.to_str().unwrap()],
                         |row| row.get(0),);

    match result {
        Ok(time_pos) => { time_pos },
        Err(ref error) => match error {
            rusqlite::Error::QueryReturnedNoRows => { 0 },
            _ => panic!("MISTAKE {:?}", error),
        },
    }

}

fn launch_video(video_path: &Path) {
    let mut mpv_builder = mpv::MpvHandlerBuilder::new().expect("Failed to init MPV builder");
    if video_path.is_file() || !video_path.is_file() {
        let video_path = video_path.to_str().expect("Expected a string for Path, got None");

        // enable On Screen Controller (disabled with libmpv by default)
        mpv_builder.set_option("osc",true).unwrap();
        mpv_builder.set_option("input-default-bindings",true).unwrap();
        mpv_builder.set_option("input-vo-keyboard",true).unwrap();

        let mut mpv = mpv_builder.build().expect("Failed to build MPV handler");

        mpv.command(&["loadfile", video_path as &str])
           .expect("Error loading file");

        // loop twice, send parameter as a string
        mpv.set_property("loop","1").unwrap();

        // set speed to 100%, send parameter as a f64
        mpv.set_property("speed",1.0).unwrap();

        let mut seeked = false;

        'main: loop {
            while let Some(event) = mpv.wait_event(0.0) {
                // even if you don't do anything with the events, it is still necessary to empty
                // the event loop
                match event {
                    // Shutdown will be triggered when the window is explicitely closed,
                    // while Idle will be triggered when the queue will end
                    mpv::Event::PlaybackRestart => {
                        if !seeked {
                            //let pos = get_time_pos(Path::new(video_path)).expect("error?");
                            let pos = get_time_pos_database(Path::new(video_path));
                            mpv.set_property("playback-time",pos).unwrap();
                            seeked = true;
                        }
                    }
                    mpv::Event::Pause => {
                        let time_pos = mpv.get_property("time-pos").unwrap();
                        save_in_database(Path::new(video_path), time_pos);
                        //save_time_pos(Path::new(video_path), time_pos);
                    }
                    mpv::Event::Shutdown | mpv::Event::Idle => {
                        break 'main;
                    }
                    _ => {}
                };
            }
        }
    } 
    else {
        /*
        println!("A file is required; {} is not a valid file",
                 video_path.to_str().unwrap());
                 */
    }
}


/*
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        retrieve_videos(-1);
    } else {
        retrieve_videos(args[1].parse::<i32>().unwrap());
    }
}*/

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
        .on_pre_event_inner('k', |s| {
            s.select_up(1);
            Some(EventResult::Consumed(None))
        }).on_pre_event_inner('j', |s| {
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
