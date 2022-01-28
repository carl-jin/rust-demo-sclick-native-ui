#[macro_use]
extern crate serde_derive;

mod bindHotkey;
mod config;
mod style;

use bindHotkey::BindHotKey;
use config::SlickConfig;
use iced::{
    button, executor,
    keyboard::{self, KeyCode},
    window, Align, Application, Button, Clipboard, Column, Command, Container, Element,
    HorizontalAlignment, Length, Row, Settings, Text,
};
use iced_native::{event, subscription, Event};
use mouse_rs::{types::keys::Keys, Mouse};
use style::styleMod;

const APP_NAME: &str = "SClick";

pub fn main() -> iced::Result {
    SClick::run(Settings {
        antialiasing: true,
        window: window::Settings {
            size: (200, 80),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug, Clone, Copy)]
enum Message {
    UpdateBindKey(KeyCode),
}

struct SClick {
    toggle: button::State,
    bindKey: String,
    bindFailed: bool,
    hotkey_manager: BindHotKey,
    config: SlickConfig,
}

impl Application for SClick {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (SClick, Command<Self::Message>) {
        let config: SlickConfig = confy::load(APP_NAME).expect("获取配置失败");

        (
            SClick {
                toggle: button::State::new(),
                bindKey: String::from(config.key.clone()),
                hotkey_manager: BindHotKey::new(),
                bindFailed: false,
                config,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from(APP_NAME)
    }

    fn update(
        &mut self,
        _message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match _message {
            Message::UpdateBindKey(keycode) => {
                let keycode = format!("{:?}", keycode);
                self.hotkey_manager.un_bind();
                let is_binded = self
                    .hotkey_manager
                    .bind(keycode.clone(), || trigger_click());
                self.bindKey = keycode.clone();

                if is_binded {
                    self.bindFailed = false;
                    self.config.save_key(keycode.clone());
                    ()
                } else {
                    self.bindFailed = true;
                    self.config.save_key(String::new());
                }
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        // let toggle_button = Button::new(
        //     &mut self.toggle,
        //     Text::new("Click234").horizontal_alignment(HorizontalAlignment::Center),
        // )
        // .min_width(180)
        // .padding(10)
        // .style(style::Button::Primary);

        let keycode = if self.bindKey.len() == 0 {
            String::from("Pls Enter a Key")
        } else {
            self.bindKey.clone()
        };
        let text = Text::new(keycode);

        // let keycode = toggle_button;

        let content = Column::new()
            .align_items(Align::Start)
            .spacing(0)
            .push(text);

        //  根据绑定成功与否显示对应的背景色
        let background = if self.bindKey.len() == 0 {
            styleMod::Container::Default
        } else {
            if self.bindFailed {
                styleMod::Container::Red
            } else {
                styleMod::Container::Green
            }
        };

        Container::new(content)
            .style(background)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        subscription::events_with(|event, status| {
            if let event::Status::Captured = status {
                return None;
            }

            match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    modifiers,
                    key_code,
                }) => Some(Message::UpdateBindKey(key_code)),
                _ => None,
            }
        })
    }
}

fn trigger_click() {
    let mouse = Mouse::new();
    mouse.click(&Keys::LEFT).expect("Unable to press button");
}
