// use std::io::{self, Write};

// struct Container {
//     children: Vec<Box<dyn UIComponent>>,
// }

// impl Container {
//     fn new() -> Self {
//         Self { children: Vec::new() }
//     }

//     fn add_child(&mut self, child: Box<dyn UIComponent>) {
//         self.children.push(child);
//     }

//     fn render(&self) {
//         for child in &self.children {
//             child.render();
//         }
//     }
// }

// trait UIComponent {
//     fn render(&self);
// }

// struct Text {
//     content: String,
// }

// impl Text {
//     fn new(content: &str) -> Self {
//         Self {
//             content: content.to_string(),
//         }
//     }
// }

// impl UIComponent for Text {
//     fn render(&self) {
//         println!("{}", self.content);
//     }
// }

// fn main() {
//     // Create a UI container
//     let mut ui_container = Container::new();

//     // Add components to the container
//     ui_container.add_child(Box::new(Text::new("Hello, Console UI!")));
//     ui_container.add_child(Box::new(Text::new("Welcome to the Rust UI language!")));

//     // Render the UI
//     ui_container.render();

//     // Handle user input (placeholder)
//     let mut input = String::new();
//     print!("Enter something: ");
//     io::stdout().flush().unwrap();
//     io::stdin().read_line(&mut input).unwrap();
//     println!("You entered: {}", input.trim());
// }

// use std::cell::RefCell;
// use std::rc::Rc;
// use std::any::Any;

// // Define a trait for components with mutable state
// trait StatefulMut {
//     fn render(&mut self);
//     fn as_any_mut(&mut self) -> &mut dyn Any;
// }

// // Define a generic state type for the Text component
// struct TextState {
//     content: String,
// }

// // Implement the StatefulMut trait for TextState
// impl StatefulMut for TextState {
//     fn render(&mut self) {
//         println!("{}", self.content);
//     }

//     fn as_any_mut(&mut self) -> &mut dyn Any {
//         self
//     }
// }

// // Define the Text component
// struct Text {
//     state: Rc<RefCell<Box<dyn StatefulMut>>>,
// }

// impl Text {
//     // Constructor for the Text component
//     fn new(content: &str) -> Self {
//         let state = Rc::new(RefCell::new(Box::new(TextState {
//             content: content.to_string(),
//         }) as Box<dyn StatefulMut>));

//         Self { state }
//     }

//     // Method to render the component
//     fn render(&mut self) {
//         self.state.borrow_mut().render();
//     }

//     // Method to update the content of the Text component
//     fn set_content(&mut self, new_content: &str) {
//         if let Some(text_state) = self.state.borrow_mut().as_any_mut().downcast_mut::<TextState>() {
//             text_state.content = new_content.to_string();
//         } else {
//             // Handle unexpected state type
//             println!("Error: Unexpected state type");
//         }
//     }
// }

// fn main() {
//     // Create a Text component with initial content
//     let mut text_component = Text::new("Initial Content");

//     // Render the initial state
//     text_component.render();

//     // Update the content of the Text component
//     text_component.set_content("Updated Content");

//     // Render the updated state
//     text_component.render();
// }

// use std::cell::RefCell;
// use std::rc::{Rc, Weak};
// use std::any::Any;

// // Define a trait for components with mutable state
// trait StatefulMut {
//     fn render(&self);
//     fn as_any_mut(&mut self) -> &mut dyn Any;
// }

// // Define a generic state type for the Text component
// struct TextState {
//     content: String,
//     on_state_change: Option<Box<dyn Fn()>>,
// }

// // Implement the StatefulMut trait for TextState
// impl StatefulMut for TextState {
//     fn render(&self) {
//         println!("{}", self.content);
//     }

//     fn as_any_mut(&mut self) -> &mut dyn Any {
//         self
//     }
// }

// // Define the Text component
// struct Text {
//     state: Rc<RefCell<Box<dyn StatefulMut>>>,
// }

// impl Text {
//     // Constructor for the Text component
//     fn new(content: &str) -> Self {
//         let on_state_change: Option<Box<dyn Fn()>> = None;
//         let state = Rc::new(RefCell::new(Box::new(TextState {
//             content: content.to_string(),
//             on_state_change,
//         }) as Box<dyn StatefulMut>));

//         Self { state }
//     }

//     // Method to render the component
//     fn render(&self) {
//         self.state.borrow().render();
//     }

//     // Method to update the content of the Text component
//     fn set_content(&mut self, new_content: &str) {
//         if let Some(text_state) = self.state.borrow_mut().as_any_mut().downcast_mut::<TextState>() {
//             text_state.content = new_content.to_string();

//             // Trigger re-render if a callback is registered
//             if let Some(ref on_state_change) = text_state.on_state_change {
//                 on_state_change();
//             }
//         } else {
//             // Handle unexpected state type
//             println!("Error: Unexpected state type");
//         }
//     }

//     // Method to register a callback for state changes
//     fn on_state_change(&self, callback: Box<dyn Fn()>) {
//         let state = Rc::clone(&self.state);
//         let text_component_weak = Rc::downgrade(&self.state);

//         let callback = move || {
//             if let Some(text_component_state) = text_component_weak.upgrade() {
//                 text_component_state.borrow().render();
//             } else {
//                 println!("Error: Text component no longer exists");
//             }
//         };

//         if let Some(text_state) = self.state.borrow_mut().as_any_mut().downcast_mut::<TextState>() {
//             text_state.on_state_change = Some(Box::new(callback));
//         } else {
//             // Handle unexpected state type
//             println!("Error: Unexpected state type");
//         }
//     }

//     // Method to create a weak reference to the Text component
//     fn weak(&self) -> Weak<RefCell<Box<dyn StatefulMut>>> {
//         Rc::downgrade(&self.state)
//     }
// }

// fn main() {
//     // Create a Text component with initial content
//     let mut text_component = Text::new("Initial Content");

//     // Register a callback for state changes
//     text_component.on_state_change(Box::new(|| {}));

//     // Render the initial state
//     text_component.render();

//     // Update the content of the Text component, triggering a re-render
//     text_component.set_content("Updated Content");
// }

use deneme::ui::container::{Container};
use deneme::ui::text::Text;

fn main() {
    let mut container = Container::new();

    // let text1 = Box::new(Text::new("Helloworld") as Box<dyn Widget>);
    // let text2 = Box::new(Text::new("second") as Box<dyn Widget>);


    // let text1 = Box::new(Text::new("Helloworld"));
    // let text2 = Box::new(Text::new("second"));
    
    // container.add_new_children("text1", text1);
    // container.add_new_children("text2", text2);

    println!("{}", container.render());
}
