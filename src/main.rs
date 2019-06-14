#![warn(trivial_casts)]
#![deny(unused, unused_qualifications)]
#![forbid(unused_import_braces)]

#![windows_subsystem = "windows"]

use std::{
    process::exit,
    time::Duration
};
use azul::{
    dialogs::{
        msg_box,
        save_file_dialog
    },
    prelude::*,
    widgets::{
        button::Button,
        label::Label,
        text_input::{
            TextInput,
            TextInputState
        }
    }
};
use open::that as open;
use lore_seeker_desktop::{
    trice,
    update::{
        download_update,
        update_check
    },
    util::*,
    version::GIT_COMMIT_HASH
};

#[derive(Default)]
struct Ls {
    search_term: TextInputState
}

impl Layout for Ls {
    fn layout(&self, info: LayoutInfo<Ls>) -> Dom<Ls> {
        Dom::div()
            .with_child(Dom::div() // search bar
                .with_child(TextInput::new().bind(info.window, &self.search_term, self).dom(&self.search_term))
                .with_child(Button::with_label("Search").dom()
                    .with_callback(On::MouseUp, search)
                )
            )
            .with_child(Button::with_label("Install Cockatrice").dom()
                .with_callback(On::MouseUp, install_trice)
            )
            .with_child(Label::new(format!("Lore Seeker Desktop version {}", &GIT_COMMIT_HASH[..7])).dom())
    }
}

fn install_trice(_: CallbackInfo<Ls>) -> UpdateScreen {
    if let Err(e) = trice::install(false) {
        error_message("Lore Seeker: Error installing Cockatrice", &format!("{}", e));
    }
    DontRedraw
}

fn search(info: CallbackInfo<Ls>) -> UpdateScreen {
    let query = &info.state.data.search_term.text;
    if let Err(e) = open(&format!("https://lore-seeker.cards/card?q={}", urlencoding::encode(if query.is_empty() { "*" } else { &query }))) {
        error_message("Lore Seeker: Error opening website", &format!("{:?}", e));
    }
    DontRedraw
}

fn update_timer(_: TimerCallbackInfo<Ls>) -> (UpdateScreen, TerminateTimer) {
    match client() {
        Ok(client) => {
            match update_check(&client) {
                Ok(true) => (DontRedraw, TerminateTimer::Continue),
                Ok(false) => if yesno("An update for Lore Seeker Desktop is available. Download now?") {
                    match save_file_dialog(None) {
                        Some(save_path) => match download_update(&client, save_path) {
                            Ok(()) => {
                                msg_box("Update downloaded. This version of Lore Seeker Desktop will now close. Please open the new version.");
                                exit(0); //TODO exit app cleanly or even auto-restart
                            }
                            Err(e) => {
                                error_message("Lore Seeker: Error downloading update", &format!("{}", e));
                                (DontRedraw, TerminateTimer::Continue)
                            }
                        },
                        None => {
                            error_message("Lore Seeker: Error checking for updates", &format!("Error determining save path"));
                            (DontRedraw, TerminateTimer::Continue)
                        }
                    }
                } else {
                    msg_box("Lore Seeker update ignored, will stop checking for updates. Restart Lore Seeker to resume update checks.");
                    (DontRedraw, TerminateTimer::Terminate)
                },
                Err(e) => {
                    error_message("Lore Seeker: Error checking for updates", &format!("{}", e));
                    (DontRedraw, TerminateTimer::Continue)
                }
            }
            //TODO check for updated Cockatrice files
        }
        Err(e) => {
            error_message("Lore Seeker: Error checking for updates", &format!("Error creating client: {}", e));
            (DontRedraw, TerminateTimer::Continue)
        }
    }
}

fn main() {
    let mut app = App::new(Ls::default(), AppConfig::default()).unwrap();
    let window = app.create_window(WindowCreateOptions::default(), css::native()).unwrap();
    app.app_state.add_timer(TimerId::new(), Timer::new(update_timer).with_interval(Duration::from_secs(3600)));
    app.run(window).unwrap();
}
