#![warn(trivial_casts)]
#![deny(unused, unused_qualifications)]
#![forbid(unused_import_braces)]

#![windows_subsystem = "windows"]

extern crate lore_seeker_desktop;
#[macro_use] extern crate native_windows_gui as nwg;
extern crate open;
extern crate urlencoding;

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
use lore_seeker_desktop::{
    trice,
    update::update_check,
    util::yesno,
    version::GIT_COMMIT_HASH
};
use self::GuiId::*;

#[derive(Debug, Clone, Copy, Hash)]
pub enum GuiId {
    // controls
    MainWindow,
    SearchInput,
    SearchButton,
    InstallTriceButton,
    Label(u8),
    // events
    StartSearch,
    InstallTrice,
    // resources
    //LargeFont,
    TextFont
}

nwg_template!(
    head: setup_ui<GuiId>,
    controls: [
        (MainWindow, nwg_window!(title="Lore Seeker"; size=(300, 57))),
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
        (InstallTriceButton, nwg_button!(
            parent=MainWindow;
            text="Install Cockatrice";
            position=(4, 30);
            size=(292, 23);
            font=Some(TextFont)
        )),
        (Label(0), LabelT {
            parent: MainWindow,
            text: format!("Lore Seeker Desktop version {}", &GIT_COMMIT_HASH[..7]),
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
                error_message("Lore Seeker: Error opening website", &format!("{:?}", e));
            }
        }),
        (InstallTriceButton, InstallTrice, Event::Click, |_, _, _, _| {
            if let Err(e) = trice::install(false) {
                error_message("Lore Seeker: Error installing Cockatrice", &format!("{}", e));
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
                if yesno("An update for Lore Seeker Desktop is available. Open download page now?") { //TODO download update automatically instead
                    if let Err(e) = open("https://github.com/fenhl/lore-seeker-desktop/releases") {
                        error_message("Lore Seeker: Error opening download page", &format!("{:?}", e));
                    }
                }
            }
            Err(e) => { error_message("Lore Seeker: Error checking for updates", &format!("{}", e)); }
        }
        //TODO check for updated Cockatrice files
        thread::sleep(Duration::from_secs(3600));
    }
}

fn main() {
    if let Err(e) = thread::Builder::new().name("Lore Seeker update check".into()).spawn(update_loop) {
        fatal_message("Lore Seeker: Error starting update check", &format!("{:?}", e));
    }
    if let Err(e) = gui_main() {
        fatal_message("Lore Seeker: Error creating GUI", &format!("{:?}", e));
    }
}
