extern crate serde_json;
extern crate cursive;


use use_mpv::callback_video;
use twitch::retrieve_videos;
use self::serde_json::{Value};
use cursive::align::HAlign;
use cursive::theme::{BaseColor, Color, Effect, PaletteColor};
use cursive::Cursive;
use cursive::traits::*;
use cursive::event::{Event};
use cursive::views::{Dialog,OnEventView,SelectView,DummyView,EditView,LinearLayout,NamedView,TextView,ViewRef};


pub fn construct_view_streamers(siv : &mut Cursive) {

    let mut select_view : ViewRef<SelectView> = siv.find_name::<SelectView>("view_streamers").unwrap();
    select_view.set_on_submit(submit_streamer);

    select_view.add_item("mistermv", "mistermv".to_string());
    select_view.add_item("pandovstrochnis","pandovstrochnis".to_string());
    select_view.add_item("modiiie","modiiie".to_string());
    select_view.add_item("bazoukha2x", "bazoukha2x".to_string());

}

pub fn construct_select_view(siv : &mut Cursive, last_videos : &str) {

    let mut select_view : ViewRef<SelectView> = siv.find_name::<SelectView>("select_view").unwrap();
    select_view.set_on_submit(callback_video);
    select_view.clear();

    let val: Value = serde_json::from_str(last_videos).unwrap();

    let mut max_videos = 10;

    if val["videos"].as_array().unwrap().len() < 10 {
        max_videos = val["videos"].as_array().unwrap().len();
    }

    for _i in 0..max_videos {
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
}

fn submit_streamer(siv : &mut Cursive, streamer : &str) {

    let last_videos = retrieve_videos(streamer);
    let mut text_view : ViewRef<TextView> = siv.find_name::<TextView>("streamer_last").unwrap();
    let title = format!("{}'s last streaming", streamer);
    text_view.set_content(title);

    construct_select_view(siv, &last_videos);
}

fn construct_edit_view(siv : &mut Cursive) {

    let mut edit_view = siv.find_name::<EditView>("edit_view").unwrap();
    edit_view.set_on_submit(submit_streamer);

}

pub fn construct_ui(siv : &mut Cursive) {

    let last_videos = retrieve_videos("mistermv");
    let view_streamers : NamedView<SelectView> = SelectView::new()
        .h_align(HAlign::Left)
        .with_name("view_streamers");

    let select_view : NamedView<SelectView> = SelectView::new()
        .h_align(HAlign::Left)
        .with_name("select_view");

    // TODO : Mettre espace pour "enter"
    // TODO : add trait to select_view pour gérer les inputs
    let select_view = OnEventView::new(select_view)
        .on_event(Event::Char('k'), move |siv| {
            siv.call_on_name("select_view", |select_view : &mut SelectView| {
                select_view.select_up(1);
            });
        })
        .on_event(Event::Char('j'), move |siv| {
            siv.call_on_name("select_view", |select_view : &mut SelectView| {
                select_view.select_down(1);
            });
        });

    let view_streamers = OnEventView::new(view_streamers)
        .on_event(Event::Char('k'), move |siv| {
            siv.call_on_name("view_streamers", |view_streamers : &mut SelectView| {
                view_streamers.select_up(1);
            });
        })
        .on_event(Event::Char('j'), move |siv| {
            siv.call_on_name("view_streamers", |view_streamers : &mut SelectView| {
                view_streamers.select_down(1);
            });
        });
    let edit_view = EditView::new()
        .with_name("edit_view");

    // Bug with bold :(
    let text_last_streams = TextView::new("mistermv's last streamings")
        .h_align(HAlign::Center)
        .effect(Effect::Underline)
        .with_name("streamer_last");

    let text_favorites = TextView::new("Favorites")
        .h_align(HAlign::Center)
        .effect(Effect::Underline);

    let mut theme = siv.current_theme().clone();
    theme.palette[PaletteColor::Background] = Color::TerminalDefault;
    theme.palette[PaletteColor::HighlightInactive] = Color::Dark(BaseColor::Magenta);
    siv.set_theme(theme);

    let views_streamers = LinearLayout::vertical()
        .child(DummyView.fixed_height(1))
        .child(text_favorites)
        .child(view_streamers)
        .child(DummyView.fixed_height(1))
        .child(edit_view);

    let last_streamers = LinearLayout::vertical()
        .child(DummyView.fixed_height(1))
        .child(text_last_streams)
        .child(select_view);

    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(views_streamers)
                .child(DummyView.fixed_width(1))
                .child(last_streamers)
        )
        .title("Last streamings")
    );

    construct_view_streamers(siv);
    construct_select_view(siv, &last_videos);
    construct_edit_view(siv);

    siv.add_global_callback('q', |s| s.quit());

}
