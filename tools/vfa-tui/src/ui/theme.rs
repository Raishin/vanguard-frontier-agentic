use ratatui::style::{Color, Modifier, Style};

/// Theme configuration supporting --no-color mode.
pub struct Theme {
    pub no_color: bool,
}

impl Theme {
    pub fn new(no_color: bool) -> Self {
        Self { no_color }
    }

    pub fn sidebar_style(&self) -> Style {
        if self.no_color {
            Style::default()
        } else {
            Style::default().fg(Color::White)
        }
    }

    pub fn sidebar_selected(&self) -> Style {
        if self.no_color {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        }
    }

    pub fn list_item(&self) -> Style {
        if self.no_color {
            Style::default()
        } else {
            Style::default().fg(Color::White)
        }
    }

    pub fn list_selected(&self) -> Style {
        if self.no_color {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightBlue)
                .add_modifier(Modifier::BOLD)
        }
    }

    pub fn detail_key(&self) -> Style {
        if self.no_color {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        }
    }

    pub fn detail_value(&self) -> Style {
        if self.no_color {
            Style::default()
        } else {
            Style::default().fg(Color::White)
        }
    }

    pub fn status_bar(&self) -> Style {
        if self.no_color {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default().fg(Color::Black).bg(Color::White)
        }
    }

    pub fn help_bar(&self) -> Style {
        if self.no_color {
            Style::default().add_modifier(Modifier::DIM)
        } else {
            Style::default().fg(Color::DarkGray)
        }
    }

    pub fn error_style(&self) -> Style {
        if self.no_color {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        }
    }

    pub fn search_style(&self) -> Style {
        if self.no_color {
            Style::default().add_modifier(Modifier::UNDERLINED)
        } else {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        }
    }

    pub fn border_style(&self) -> Style {
        if self.no_color {
            Style::default()
        } else {
            Style::default().fg(Color::Gray)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_color_mode_uses_no_fg_color() {
        let theme = Theme::new(true);
        assert_eq!(theme.sidebar_style().fg, None);
        assert_eq!(theme.list_item().fg, None);
        assert_eq!(theme.detail_value().fg, None);
    }

    #[test]
    fn color_mode_has_colors() {
        let theme = Theme::new(false);
        assert!(theme.sidebar_style().fg.is_some());
        assert!(theme.list_selected().bg.is_some());
        assert!(theme.detail_key().fg.is_some());
    }
}
