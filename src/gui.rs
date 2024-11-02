use gtk::{prelude::*, Align::*, Application, ApplicationWindow, Button, Label};
use std::cell::Cell;
use std::rc::Rc;

pub fn build_ui(application: &Application) {
    // Create a label to display the number.
    let label = Label::builder()
        .label("0")
        .margin_top(20)
        .margin_bottom(20)
        .valign(Center)
        .halign(Center)
        .build();

    // Create a button to increase the number.
    let button_increase = Button::builder()
        .label("Up!")
        .valign(Center)
        .halign(Center)
        .build();

    // Create a button to decrease the number.
    let button_decrease = Button::builder()
        .label("Down!")
        .valign(Center)
        .halign(Center)
        .build();

    // Shared number that both buttons will modify.
    let number = Rc::new(Cell::new(0));

    // Connect the "Increase" button to increment the number and update the label.
    let inc_number = Rc::clone(&number);
    let inc_label = label.clone();
    button_increase.connect_clicked(move |_| {
        inc_number.set(inc_number.get() + 1);
        inc_label.set_text(&format!("{}", inc_number.get()));
    });

    // Connect the "Decrease" button to decrement the number and update the label.
    let dec_number = Rc::clone(&number);
    let dec_label = label.clone();
    button_decrease.connect_clicked(move |_| {
        dec_number.set(dec_number.get() - 1);
        dec_label.set_text(&format!("{}", dec_number.get()));
    });

    // Create the main application window.
    let window = ApplicationWindow::builder()
        .application(application)
        .title("Counter App")
        .build();

    // Create a vertical layout container and add the label and buttons.
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    vbox.set_valign(Center);
    vbox.set_halign(Center);
    
    vbox.append(&button_increase);
    vbox.append(&label);
    vbox.append(&button_decrease);

    window.set_child(Some(&vbox));
    window.present();
}
