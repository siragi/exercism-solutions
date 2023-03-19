pub fn reply(message: &str) -> &str {
    // have Bob reply to the incoming message
    let msg = Message::new(message);

    match msg {
        a if a.style == "nodding" => "Fine. Be that way!",
        b if b.intonation == "yelling" && b.style == "question" => {
            "Calm down, I know what I'm doing!"
        }
        c if c.intonation == "yelling" => "Whoa, chill out!",
        d if d.style == "question" => "Sure.",
        _ => "Whatever.",
    }
}

#[derive(Debug)]
struct Message<'a> {
    text: &'a str,
    style: &'a str,
    intonation: &'a str,
}

impl<'a> Message<'a> {
    fn new(text: &'a str) -> Message<'a> {
        let mut m = Message {
            text,
            style: "",
            intonation: "",
        };

        Message::analyze(&mut m);
        m
    }

    pub fn analyze(&mut self) {
        let m = self.text.trim();

        // style
        match m {
            "" => self.style = "nodding",
            m if m.ends_with("?") => self.style = "question",
            m if m.ends_with("!") => self.style = "command",
            m if m.ends_with(".") => self.style = "statement",
            _ => self.style = "unconventional",
        };

        // intonation
        if m.contains(|c: char| c.is_alphabetic()) // only alphabetic text can be yelled.
            && m.to_uppercase() == m
        {
            self.intonation = "yelling"
        } else {
            self.intonation = "calm"
        }
    }
}
