mod city;
mod race;
mod state;
mod views;

use crate::city::City;
use crate::race::RaceDatabase;
use crate::state::GameState;
use crate::views::CreateWorldMenu;
use cursive::align::HAlign;
use cursive::theme::{BaseColor::*, Color::*, Palette, PaletteColor::*, Theme};
use cursive::view::Resizable;
use cursive::views::Button;
use cursive::views::LinearLayout;
use cursive::views::{Dialog, TextView};
use cursive::Cursive;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let mut siv = cursive::default();
    let state = Rc::new(RefCell::new(GameState::new(
        StdRng::from_entropy(),
        RaceDatabase::default(),
    )));

    let mut palette = Palette::default();
    palette.extend(vec![
        (Background, Dark(Black)),
        (View, Dark(Black)),
        (Primary, Light(White)),
        (Secondary, Dark(White)),
        (Tertiary, Light(Black)),
        (TitlePrimary, Light(White)),
        (TitleSecondary, Dark(White)),
        (Highlight, Light(Black)),
        (HighlightText, Light(White)),
        (HighlightInactive, Dark(White)),
    ]);

    siv.set_theme(Theme {
        palette,
        shadow: false,
        ..Default::default()
    });

    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(
                    TextView::new(
                        r"                              _ _
                             | (_)            
 ___ _ __  _ __ __ ___      _| |_ _ __   __ _ 
/ __| '_ \| '__/ _` \ \ /\ / / | | '_ \ / _` |
\__ \ |_) | | | (_| |\ V  V /| | | | | | (_| |
|___/ .__/|_|  \__,_| \_/\_/ |_|_|_| |_|\__, |
    | |                                  __/ |
    |_|                                 |___/ ",
                    )
                    .h_align(HAlign::Left)
                    .fixed_size((46, 9)),
                )
                .child(Button::new("Create World", |s| {
                    let mut menu = CreateWorldMenu::default();
                    menu.menu(s)
                }))
                .child(Button::new("Load World", |_s| {}))
                .child(Button::new("Settings", |_s| {}))
                .child(Button::new("Quit", |s| s.quit())),
        )
        .padding_lrtb(2, 2, 1, 1),
    );

    siv.run();
}

fn restart(siv: &mut Cursive, state: Rc<RefCell<GameState>>) {
    state.borrow_mut().update_city();
    let text = state.borrow().to_text();
    siv.pop_layer();
    siv.add_layer(
        Dialog::text(text)
            .title("Esports Ready")
            .button("Quit", |s| s.quit())
            .button("Restart", move |s| restart(s, state.clone()))
            .button("Continue", |s| s.add_layer(Dialog::info("Ok!"))),
    );
}
