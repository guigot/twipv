extern crate mpv;
extern crate cursive;

use std::path::Path;
use cursive::Cursive;

pub fn callback_video(_siv: &mut Cursive, url: &str) {
    launch_video(Path::new(url));
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
