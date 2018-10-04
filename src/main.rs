#![warn(trivial_casts)]
#![deny(unused, unused_qualifications)]
#![forbid(unused_import_braces)]

#![windows_subsystem = "windows"]

#[macro_use] extern crate native_windows_gui as nwg;

use nwg::{
    Event,
    Ui,
    dispatch_events,
    fatal_message,
    simple_message
};
use self::GuiId::*;

#[derive(Debug, Clone, Copy, Hash)]
pub enum GuiId {
    // controls
    MainWindow,
    NameInput,
    HelloButton,
    Label(u8),
    // events
    SayHello,
    // resources
    MainFont,
    TextFont
}

nwg_template!(
    head: setup_ui<GuiId>,
    controls: [
        (MainWindow, nwg_window!(title="Template Example"; size=(280, 105))),
        (Label(0), nwg_label!(
            parent=MainWindow;
            text="Your Name: ";
            position=(5, 15);
            size=(80, 25);
            font=Some(TextFont)
        )),
        (NameInput, nwg_textinput!(
            parent=MainWindow;
            position=(85, 13);
            size=(185, 22);
            font=Some(TextFont)
        )),
        (HelloButton, nwg_button!(
            parent=MainWindow;
            text="Hello World!";
            position=(5, 45);
            size=(270, 50);
            font=Some(MainFont)
        ))
    ];
    events: [
        (HelloButton, SayHello, Event::Click, |ui, _, _, _| {
            let your_name = nwg_get!(ui; (NameInput, nwg::TextInput));
            simple_message("Hello", &format!("Hello {}!", your_name.get_text()));
        })
    ];
    resources: [
        (MainFont, nwg_font!(family="Arial"; size=27)),
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
