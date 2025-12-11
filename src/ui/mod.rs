use iced::widget::{button, column, container, pick_list, row, text, vertical_space};
use iced::{executor, Application, Color, Command, Element, Length, Theme};
use midir::{MidiInput, MidiInputConnection};
use std::sync::Arc;
use crate::audio::AudioEngine;

pub struct ToyPianoApp {
    audio_engine: Arc<AudioEngine>,
    midi_connection: Option<MidiInputConnection<()>>, // Holds the active connection
    available_ports: Vec<String>,
    selected_port: Option<String>,
    status_message: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    PortSelected(String),
    Rescan,
}

impl Application for ToyPianoApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = Arc<AudioEngine>;

    fn new(audio_engine: Arc<AudioEngine>) -> (Self, Command<Message>) {
        let ports = match MidiInput::new("Toy Piano UI Input") {
            Ok(input) => {
                let ports = input.ports();
                ports.iter()
                    .map(|p| input.port_name(p).unwrap_or_else(|_| "Unknown".to_string()))
                    .collect()
            },
            Err(_) => vec![]
        };

        let app = ToyPianoApp {
            audio_engine,
            midi_connection: None,
            available_ports: ports,
            selected_port: None,
            status_message: "Ready. Select a MIDI Input.".to_string(),
        };

        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Toy Piano")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Rescan => {
               if let Ok(input) = MidiInput::new("Toy Piano scanner") {
                   let ports = input.ports();
                   self.available_ports = ports.iter()
                       .map(|p| input.port_name(p).unwrap_or_else(|_| "Unknown".to_string()))
                       .collect();
                    match self.available_ports.len() {
                        0 => self.status_message = "No MIDI ports found.".to_string(),
                        n => self.status_message = format!("Found {} MIDI ports.", n),
                    }
               }
            }
            Message::PortSelected(port_name) => {
                self.selected_port = Some(port_name.clone());
                self.status_message = format!("Connecting to {}...", port_name);
                
                // Disconnect old
                self.midi_connection = None;

                // Connect new
                if let Ok(input) = MidiInput::new("Toy Piano Input Connection") {
                     let ports = input.ports();
                     if let Some(port) = ports.into_iter().find(|p| input.port_name(p).unwrap_or_default() == port_name) {
                         
                         let synth = self.audio_engine.get_synthesizer();
                         
                        let conn_result = input.connect(
                            &port,
                            "toy-piano-input-ui",
                            move |_stamp, message, _| {
                                crate::midi::handle_midi_message(message, &synth);
                            },
                            (),
                        );
                        
                        match conn_result {
                            Ok(conn) => {
                                self.status_message = format!("Connected to {}", port_name);
                                self.midi_connection = Some(conn);
                            },
                            Err(e) => {
                                self.status_message = format!("Failed to connect: {}", e);
                            }
                        }
                     }
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        // Design: Deep Purple Theme
        let header = text("TOY PIANO")
            .size(40)
            .style(Color::from_rgb(0.8, 1.0, 0.8)); // Slightly Greener

        let status = text(&self.status_message)
            .size(16)
            .style(Color::from_rgb(0.0, 1.0, 0.5)); // Green accent

        let port_picker = pick_list(
            self.available_ports.clone(),
            self.selected_port.clone(),
            Message::PortSelected
        )
        .placeholder("Select MIDI Device...")
        .width(Length::Fixed(300.0))
        .style(iced::theme::PickList::Custom(std::rc::Rc::new(DeepPurplePickList), std::rc::Rc::new(DeepPurpleOverlay)));

        let rescan_button = button("Rescan Devices")
            .style(iced::theme::Button::Custom(Box::new(ForestGreenButton)))
            .on_press(Message::Rescan);

        let content = column![
            header,
            vertical_space().height(20),
            status,
            vertical_space().height(40),
            row![
                text("MIDI Input:").size(20).style(Color::from_rgb(0.8, 1.0, 0.8)),
                port_picker,
                rescan_button
            ].spacing(20).align_items(iced::Alignment::Center)
        ]
        .spacing(10)
        .padding(40)
        .align_items(iced::Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(iced::theme::Container::Custom(Box::new(DeepPurpleTheme)))
            .into()
    }
}

struct DeepPurpleTheme;

impl container::StyleSheet for DeepPurpleTheme {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(Color::from_rgb8(30, 0, 50))), // Deep Purple
            text_color: Some(Color::from_rgb(0.9, 1.0, 0.9)), // Pale Green
            ..Default::default()
        }
    }
}

struct ForestGreenButton;

impl button::StyleSheet for ForestGreenButton {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(Color::from_rgb8(11, 60, 14))), // Darker Forest Green
            text_color: Color::from_rgb(0.9, 1.0, 0.9), // Pale Green text
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        button::Appearance {
            background: Some(iced::Background::Color(Color::from_rgb8(0, 100, 0))), // Darker Green
            ..active
        }
    }
}

struct DeepPurplePickList;

impl pick_list::StyleSheet for DeepPurplePickList {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> pick_list::Appearance {
        pick_list::Appearance {
            text_color: Color::from_rgb(0.9, 1.0, 0.9), // Pale Green
            placeholder_color: Color::from_rgb(0.5, 0.8, 0.5), // Darker Green for placeholder
            handle_color: Color::WHITE,
            background: iced::Background::Color(Color::from_rgb8(50, 20, 70)), // Lighter Purple
            border: iced::Border {
                radius: 4.0.into(),
                width: 1.0,
                color: Color::from_rgb8(60, 30, 80),
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> pick_list::Appearance {
        let active = self.active(style);
        pick_list::Appearance {
            background: iced::Background::Color(Color::from_rgb8(60, 30, 80)), // Lighter on hover
            ..active
        }
    }
}

struct DeepPurpleOverlay;

impl iced::overlay::menu::StyleSheet for DeepPurpleOverlay {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::overlay::menu::Appearance {
        iced::overlay::menu::Appearance {
            text_color: Color::from_rgb(0.8, 1.0, 0.8), // Matches "MIDI Input" label
            background: iced::Background::Color(Color::from_rgb8(40, 10, 60)), // Menu Background
            border: iced::Border {
                width: 1.0,
                color: Color::from_rgb8(80, 50, 100),
                radius: 4.0.into(),
            },
            selected_text_color: Color::WHITE,
            selected_background: iced::Background::Color(Color::from_rgb8(34, 139, 34)), // Forest Green selection
        }
    }
}
