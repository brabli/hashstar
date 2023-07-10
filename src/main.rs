use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use clipboard::{ClipboardContext, ClipboardProvider};
use iced::widget::{button, column, text};
use iced::Element;
use iced::Settings;
use iced::{executor, subscription, window, Application, Command, Subscription};
use iced::{Event, Theme};
use md5::Digest;

fn main() -> iced::Result {
    Hashstar::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
    CopyDigest,
}

#[derive(Default)]
struct Hashstar {
    digest: String,
    err: String,
}

impl Application for Hashstar {
    type Message = Message;

    fn new(_flags: ()) -> (Hashstar, Command<Message>) {
        (Default::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Hashstar")
    }

    fn view(&self) -> Element<Message> {
        let column = column![
            text(&self.err),
            text("MD5 here"),
            text(&self.digest).size(30),
            button("Copy Digest").on_press(Message::CopyDigest)
        ];
        column
            .padding(20)
            .align_items(iced::Alignment::Center)
            .into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::EventOccurred(event) => {
                if let Event::Window(window::Event::FileDropped(buf)) = event {
                    self.digest = format!("{:x}", hash_file(buf))
                }
            }
            Message::CopyDigest => {
                // The iced::clipboard does not appear to write to the clipboard
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                ctx.set_contents(self.digest.to_owned()).unwrap();
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events().map(Message::EventOccurred)
    }

    type Executor = executor::Default;

    type Theme = Theme;

    type Flags = ();
}

fn hash_file(buf: PathBuf) -> Digest {
    let bytes = read_file_to_u8_slice(buf.to_str().expect("Failed to read file")).unwrap();
    md5::compute(bytes)
}

fn read_file_to_u8_slice(filename: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let metadata = file.metadata()?;
    let file_size = metadata.len() as usize;
    let mut buffer = vec![0; file_size];
    file.read_exact(&mut buffer)?;

    Ok(buffer)
}
