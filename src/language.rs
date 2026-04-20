pub struct Language<'a> {
    pub title: &'a str,
    pub play: &'a str,
    pub settings: &'a str,
    pub exit: &'a str,
    pub back: &'a str,
    pub join: &'a str,
    pub address: &'a str,
    pub port: &'a str,
}

impl Default for Language<'_> {
    fn default() -> Self {
        Language {
            title: "Lethal 4D",
            play: "Play",
            settings: "Settings",
            exit: "Exit",
            back: "Back",
            join: "Join",
            address: "Address",
            port: "Port",
        }
    }
}