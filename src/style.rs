pub mod styleMod {
    use iced::{button, container, Background, Color, Vector};

    pub enum Container {
        Default,
        Green,
        Red,
    }

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(match self {
                    Container::Default => Color::from_rgb(1.0, 1.0, 1.0),
                    Container::Green => Color::from_rgb(0.62, 0.92, 0.13),
                    Container::Red => Color::from_rgb(1.0, 0.133, 0.0),
                })),
                text_color: Some(match self {
                    Container::Default => Color::from_rgb(0.0, 0.0, 0.0),
                    Container::Green => Color::from_rgb(0.0, 0.0, 0.0),
                    Container::Red => Color::from_rgb(1.0, 1.0, 1.0),
                }),
                ..container::Style::default()
            }
        }
    }

    pub enum Button {
        Primary,
        Secondary,
        Destructive,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
                    Button::Destructive => Color::from_rgb(0.8, 0.2, 0.2),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }
    }
}
