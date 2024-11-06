use gtk::{prelude::*, Application, Stack, Orientation, StackSwitcher, ApplicationWindow};
use crate::gui_calc;
use crate::gui_counter;

pub fn build_ui(app: &Application) {
    // Create the main application window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Counter-Calculator")
        .default_width(400)
        .default_height(300)
        .build();

    // Create a Stack and StackSwitcher
    let stack = Stack::builder()
        .transition_type(gtk::StackTransitionType::SlideLeftRight)
        .build();

    let stack_switcher = StackSwitcher::builder()
        .stack(&stack)
        .build();

    // Add each page to the stack by calling separate functions
    stack.add_titled(&gui_calc::build_ui(), Some("calculator"), "Calculator");
    stack.add_titled(&gui_counter::build_ui(), Some("counter"), "Counter");

    // Set up the horizontal layout with StackSwitcher on the left
    let hbox = gtk::Box::new(Orientation::Vertical, 5);
    hbox.append(&stack_switcher);
    hbox.append(&stack);

    // Add everything to the window and show it
    window.set_child(Some(&hbox));
    window.set_visible(true);
}