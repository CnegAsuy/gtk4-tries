use gtk::{prelude::{ButtonExt, GtkWindowExt}, Align::*, Application, ApplicationWindow, Button};
use std::cell::Cell;

pub fn build_ui(application: &Application) {
    // Create a button for increase the number.
    let button = Button::builder()
        .label("Increase")
        //Vertical and horizontal center align.
        .valign(Center)
        .halign(Center)
        .build();



    // number for increase.
    let number = Cell::new(0);

    // Increase the number if user click the buttton.
    button.connect_clicked(move |button| {
        number.set(number.get() + 1);
        button.set_label(format!("{}", number.get()).as_str());
    });

    //
    let window = ApplicationWindow::builder()
        .application(application)
        .title("My GTK App")
        .child(&button)
        .build();

    window.present();
}