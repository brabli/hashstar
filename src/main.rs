use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use iced::widget::{column, text};
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
}

#[derive(Default)]
struct Hashstar {
    value: String,
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
        let column = column![text(&self.value).size(50),];

        column.padding(20).into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::EventOccurred(event) => {
                if let Event::Window(window::Event::FileDropped(buf)) = event {
                    self.value = format!("{:x}", hash_file(buf))
                }
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
