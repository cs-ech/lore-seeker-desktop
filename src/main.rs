#![warn(trivial_casts)]
#![deny(unused, unused_qualifications)]
#![forbid(unused_import_braces)]

#![windows_subsystem = "windows"]

extern crate itertools;
#[macro_use] extern crate native_windows_gui as nwg;
extern crate open;
extern crate reqwest;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate urlencoding;

mod version;

use std::{
    thread,
    time::Duration
};
use nwg::{
    Event,
    LabelT,
    Ui,
    constants::HTextAlign,
    dispatch_events,
    error_message,
    fatal_message
};
use open::that as open;
use self::{
    GuiId::*,
    update::update_check
};

#[derive(Debug, Clone, Copy, Hash)]
pub enum GuiId {
    // controls
    MainWindow,
    SearchInput,
    SearchButton,
    Label(u8),
    // events
    StartSearch,
    // resources
    //LargeFont,
    TextFont
}

nwg_template!(
    head: setup_ui<GuiId>,
    controls: [
        (MainWindow, nwg_window!(title="Lore Seeker"; size=(300, 50))),
        (SearchInput, nwg_textinput!(
            parent=MainWindow;
            position=(5, 5);
            size=(212, 21);
            font=Some(TextFont)
        )),
        (SearchButton, nwg_button!(
            parent=MainWindow;
            text="Search";
            position=(221, 4);
            size=(75, 23);
            font=Some(TextFont)
        )),
        (Label(0), LabelT {
            parent: MainWindow,
            text: format!("Lore Seeker Desktop version {}", &version::GIT_COMMIT_HASH[..7]),
            position: (5, 30),
            size: (300, 25),
            font: Some(TextFont),
            visible: true,
            disabled: true,
            align: HTextAlign::Left
        })
    ];
    events: [
        (SearchButton, StartSearch, Event::Click, |ui, _, _, _| {
            let query = nwg_get!(ui; (SearchInput, nwg::TextInput)).get_text();
            if let Err(e) = open(&format!("https://loreseeker.fenhl.net/card?q={}", urlencoding::encode(if query.is_empty() { "*" } else { &query }))) {
                error_message("Error opening website", &format!("{:?}", e));
            }
        })
    ];
    resources: [
        //(LargeFont, nwg_font!(family="Arial"; size=27)),
        (TextFont, nwg_font!(family="Arial"; size=17))
    ];
    values: []
);

fn gui_main() -> Result<(), nwg::Error> {
    let app = Ui::new()?;
    setup_ui(&app)?;
    dispatch_events();
    Ok(())
}

fn update_loop() {
    loop {
        match update_check() {
            Ok(true) => (),
            Ok(false) => {
                unimplemented!(); //TODO ask if Lore Seeker should be updated
            }
            Err(e) => { fatal_message("Error checking for updates", &format("{:?}", e)); }
        }
        thread::sleep(Duration::from_secs(3600));
    }
}

fn main() {
    if let Err(e) = thread::Builder::new().name("Lore Seeker update check".into()).spawn(update_loop) {
        fatal_message("Error starting update check", &format("{:?}", e));
    }
    if let Err(e) = gui_main() {
        fatal_message("Error creating GUI", &format!("{:?}", e));
    }
}
