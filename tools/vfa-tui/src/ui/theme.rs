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

    /// Style for validation gate status: NotRun (dim/gray).
    pub fn gate_not_run(&self) -> Style {
        if self.no_color {
            Style::default().add_modifier(Modifier::DIM)
        } else {
            Style::default().fg(Color::DarkGray)
        }
    }

    /// Style for validation gate status: Running (yellow).
    pub fn gate_running(&self) -> Style {
        if self.no_color {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        }
    }

    /// Style for validation gate status: Passed (green).
    pub fn gate_passed(&self) -> Style {
        if self.no_color {
            Style::default()
        } else {
            Style::default().fg(Color::Green)
        }
    }

    /// Style for validation gate status: Failed (red).
    pub fn gate_failed(&self) -> Style {
        if self.no_color {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        }
    }

    /// Style for validation gate status: TimedOut (magenta).
    pub fn gate_timed_out(&self) -> Style {
        if self.no_color {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD)
        }
    }

    /// Style for filter chips display.
    pub fn filter_chip(&self) -> Style {
        if self.no_color {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
        }
    }

    /// Style for sparkline bar (filled portion).
    pub fn sparkline_filled(&self) -> Style {
        if self.no_color {
            Style::default()
        } else {
            Style::default().fg(Color::Cyan)
        }
    }

    /// Style for sparkline bar (empty portion).
    pub fn sparkline_empty(&self) -> Style {
        if self.no_color {
            Style::default().add_modifier(Modifier::DIM)
        } else {
            Style::default().fg(Color::DarkGray)
        }
    }

    /// Style for help overlay title.
    pub fn help_overlay_title(&self) -> Style {
        if self.no_color {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        }
    }

    /// Style for help overlay section header.
    pub fn help_overlay_section(&self) -> Style {
        if self.no_color {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        }
    }

    /// Style for completion suggestion (highlighted).
    pub fn completion_highlight(&self) -> Style {
        if self.no_color {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightGreen)
        }
    }

    /// Style for completion suggestion (normal).
    pub fn completion_normal(&self) -> Style {
        if self.no_color {
            Style::default()
        } else {
            Style::default().fg(Color::White)
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
