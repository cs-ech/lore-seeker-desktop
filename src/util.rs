use nwg::{
    self,
    constants::{
        MessageButtons,
        MessageChoice,
        MessageIcons,
        MessageParams
    },
    fatal_message
};

pub fn yesno(message: &str) -> bool {
    let choice = nwg::message(&MessageParams {
        title: "Lore Seeker",
        content: message,
        buttons: MessageButtons::YesNo,
        icons: MessageIcons::Question
    });
    match choice {
        MessageChoice::Yes => true,
        MessageChoice::No => false,
        c => { fatal_message("Lore Seeker fatal error", &format!("Yes/no message returned unexpected choice: {:?}", c)); }
    }
}
