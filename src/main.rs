#![warn(trivial_casts)]
#![deny(unused, unused_qualifications)]
#![forbid(unused_import_braces)]

#![windows_subsystem = "windows"]

#[macro_use] extern crate native_windows_gui as nwg;
extern crate open;
extern crate urlencoding;

mod version;

use nwg::{
    Event,
    Ui,
    dispatch_events,
    fatal_message
};
use open::that as open;
use self::GuiId::*;

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
        (MainWindow, nwg_window!(title="Lore Seeker"; size=(300, 60))),
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
        (Label(0), nwg_label!(
            parent=MainWindow;
            text=&format!("Lore Seeker Desktop version {}", version::GIT_COMMIT_HASH[..7]);
            position=(5, 30);
            size=(100, 25);
            font=Some(TextFont)
        ))
    ];
    events: [
        (SearchButton, StartSearch, Event::Click, |ui, _, _, _| {
            let query = nwg_get!(ui; (SearchInput, nwg::TextInput)).get_text();
            if let Err(e) = open(&format!("https://loreseeker.fenhl.net/card?q={}", urlencoding::encode(if query.is_empty() { "*" } else { &query }))) {
                fatal_message("Error opening website", &format!("{:?}", e));
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

fn main() {
    if let Err(e) = gui_main() {
        fatal_message("Fatal Error", &format!("{:?}", e));
    }
}
