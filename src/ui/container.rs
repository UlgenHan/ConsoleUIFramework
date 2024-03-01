// container.rs

// use crate::ui::widget::Widget;

// pub struct Container {
//     children: Vec<(String, Box<dyn Widget>)>,
// }

// impl Container {
//     pub fn new() -> Self {
//         Container {
//             children: Vec::new(),
//         }
//     }

//     pub fn add_child(&mut self, key: &str, child: Box<dyn Widget>) {
//         self.children.push((key.to_string(), child));
//     }

//     pub fn render(&self) -> String {
//         let mut result = String::new();
//         for (key, child) in &self.children {
//             result.push_str(&format!("{}: {}\n", key, child.render()));
//         }
//         result
//     }
// }

// container.rs

// use crossterm::{
//     cursor::{Hide, Show},
//     execute,
//     style::{Color, Print, SetBackgroundColor, SetForegroundColor},
//     terminal::{self, ClearType},
//     QueueableCommand, // Add this import
// };
// use std::io::{self, Write};

// use crate::ui::widget::Widget;

// pub struct Style {
//     pub foreground_color: String,
//     pub background_color: String,
//     pub border_color: String,
//     pub padding: u16,
//     // Add other styling properties specific to containers
// }

// impl Style {
//     pub fn new() -> Self {
//         Style {
//             foreground_color: String::from(""),
//             background_color: String::from(""),
//             border_color: String::from(""),
//             padding: 0,
//             // Initialize other properties
//         }
//     }
// }

// pub struct Container {
//     children: Vec<(String, Box<dyn Widget>)>,
//     style: Style,
// }

// impl Container {
//     pub fn new() -> Self {
//         Container {
//             children: Vec::new(),
//             style: Style::new(),
//         }
//     }

//     pub fn set_style(&mut self, style: Style) {
//         self.style = style;
//     }

//     pub fn render(&self) {
//         self.render_to_crossterm();
//     }

//     pub fn render_to_crossterm(&self) {
//         // Set up crossterm terminal
//         let mut stdout = io::stdout();
//         let _ = terminal::enable_raw_mode();
//         let _ = stdout.execute(Hide);

//         // Apply container styles
//         let _ = stdout.queue(SetForegroundColor(Color::Rgb {
//             r: 255,
//             g: 255,
//             b: 255,
//         }));
//         let _ = stdout.queue(SetBackgroundColor(Color::Rgb {
//             r: 0,
//             g: 0,
//             b: 0,
//         }));
//         // Apply other styles...

//         // Render container content
//         let _ = stdout.queue(ClearType::All);
//         let _ = stdout.queue(Print("Container content")); // Update this line with actual content
//         // Apply other rendering logic...

//         // Cleanup and flush
//         let _ = stdout.execute(Show);
//         let _ = terminal::disable_raw_mode();
//         let _ = stdout.flush();
//     }
// }

// container.rs

// container.rs

use std::io;

use crossterm::{
    execute,
    style::Print,
    terminal::{self, ClearType}, // Import ClearType from terminal submodule
    QueueableCommand,
};

use crossterm::terminal::Clear; // Import Clear separately

use crate::ui::widget::Widget; // Assuming Widget is defined elsewhere

// Define the style for the container
pub struct Style {
    pub foreground_color: String,
    pub background_color: String,
    pub border_color: String,
}

impl Style {
    pub fn new() -> Self {
        Style {
            foreground_color: String::from(""),
            background_color: String::from(""),
            border_color: String::from(""),
        }
    }
}

// Define the container struct
pub struct Container {
    children: Vec<(String, Box<dyn Widget>)>,
    style: Style,
}

impl Container {
    pub fn new() -> Self {
        Container {
            children: Vec::new(),
            style: Style::new(),
        }
    }

    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    pub fn render(&self) -> String {
        let mut result = String::new();

        // Wrap stdout in RawScreen
        let mut stdout = io::stdout();
        execute!(stdout, terminal::EnterAlternateScreen); // Use DisableCursor

        // Apply styles to the result
        result.push_str(&format!(
            "Container Style: Foreground: {}, Background: {}, Border: {}\n",
            self.style.foreground_color,
            self.style.background_color,
            self.style.border_color
        ));

        for (key, child) in &self.children {
            result.push_str(&format!("{}: {}\n", key, child.render()));
        }

        // Add a placeholder content for demonstration purposes
        stdout.queue(Print("Container content"));

        // Clear the screen using Clear command
        stdout.queue(Clear(ClearType::All));

        // Show the cursor and exit alternate screen
        execute!(stdout, terminal::LeaveAlternateScreen); // Use EnableCursor

        result
    }
}
