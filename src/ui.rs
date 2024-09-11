use crate::config::array_field_config;
use crate::twitch_api::get_vods;
use crate::use_mpv::callback_video;
use cursive::align::HAlign;
use cursive::event::Event;
use cursive::theme::{BaseColor, Color, Effect, PaletteColor};
use cursive::traits::*;
use cursive::views::{
    Dialog, DummyView, EditView, LinearLayout, NamedView, OnEventView, SelectView, TextView,
    ViewRef,
};
use cursive::Cursive;
use serde_json::Value;
use std::vec::Vec;

pub fn construct_view_streamers(siv: &mut Cursive, favorites_streamers: Vec<toml::Value>) {
    let mut select_view: ViewRef<SelectView> =
        siv.find_name::<SelectView>("view_streamers").unwrap();
    select_view.set_on_submit(submit_streamer);

    submit_streamer(siv, favorites_streamers[0].as_str().unwrap());

    let mut sorted_streamers : Vec<String> = Vec::new();
    for streamer in favorites_streamers {
        sorted_streamers.push(streamer.as_str().unwrap().to_string());
    }
    sorted_streamers.sort();
    for streamer in sorted_streamers.iter() {
        select_view.add_item(streamer, streamer.to_string())
    }
}

pub fn construct_select_view(siv: &mut Cursive, last_videos: &str) {
    let mut select_view: ViewRef<SelectView> = siv.find_name::<SelectView>("select_view").unwrap();
    select_view.set_on_submit(callback_video);
    select_view.clear();

    let val: Value = serde_json::from_str(last_videos).unwrap();
    let mut max_videos = 10;

    if val["data"].as_array().unwrap().len() < 10 {
        max_videos = val["data"].as_array().unwrap().len();
    }

    for _i in 0..max_videos {
        let mut plain_title: String = val["data"][_i]["title"].as_str().unwrap().to_string();
        let size = 70;
        if plain_title.chars().count() > size {
            plain_title = plain_title.chars().take(size - 3).collect();
            plain_title.push_str("...");
        } else {
            let fill = " ".repeat(size - plain_title.chars().count());
            plain_title.push_str(fill.as_str());
        }

        select_view.add_item(
            plain_title,
            val["data"][_i]["url"].as_str().unwrap().to_string(),
        );
    }
}

fn submit_streamer(siv: &mut Cursive, streamer: &str) {
    let last_videos = get_vods(streamer);
    let mut text_view: ViewRef<TextView> = siv.find_name::<TextView>("streamer_last").unwrap();
    let title = format!("{}'s last streaming", streamer);
    text_view.set_content(title);

    construct_select_view(siv, &last_videos);
}

fn construct_edit_view(siv: &mut Cursive) {
    let mut edit_view = siv.find_name::<EditView>("edit_view").unwrap();
    edit_view.set_on_submit(submit_streamer);
}

pub fn construct_ui(siv: &mut Cursive) {
    let view_streamers: NamedView<SelectView> = SelectView::new()
        .h_align(HAlign::Left)
        .with_name("view_streamers");

    let select_view: NamedView<SelectView> = SelectView::new()
        .h_align(HAlign::Left)
        .with_name("select_view");

    let select_view = OnEventView::new(select_view)
        .on_event(Event::Char('r'), move |siv| {
            siv.call_on_name("select_view", |select_view: &mut SelectView| {
                select_view.select_up(1);
            });
        })
        .on_event(Event::Char('s'), move |siv| {
            siv.call_on_name("select_view", |select_view: &mut SelectView| {
                select_view.select_down(1);
            });
        });

    let view_streamers = OnEventView::new(view_streamers)
        .on_event(Event::Char('r'), move |siv| {
            siv.call_on_name("view_streamers", |view_streamers: &mut SelectView| {
                view_streamers.select_up(1);
            });
        })
        .on_event(Event::Char('s'), move |siv| {
            siv.call_on_name("view_streamers", |view_streamers: &mut SelectView| {
                view_streamers.select_down(1);
            });
        });
    let edit_view = EditView::new().with_name("edit_view");

    // Bug with bold :(
    let text_last_streams = TextView::new("")
        .h_align(HAlign::Center)
        .style(Effect::Underline)
        .with_name("streamer_last");

    let text_favorites = TextView::new("Favorites")
        .h_align(HAlign::Center)
        .style(Effect::Underline);

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
                .child(last_streamers),
        )
        .title("Last streamings"),
    );

    let favorites_streamers = array_field_config("favorites");
    let last_videos = get_vods(favorites_streamers[0].as_str().unwrap());
    construct_view_streamers(siv, favorites_streamers);
    construct_select_view(siv, &last_videos);
    construct_edit_view(siv);

    siv.add_global_callback('q', |s| s.quit());
}
