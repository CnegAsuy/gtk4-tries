use gtk::{prelude::*, Align::*, Button, Entry, Label};

pub fn build_ui() -> gtk::Box {
    // defining the GObjects for use
    let label = Label::builder()
        .label("Result will be here!")
        .margin_top(20)
        .margin_bottom(20)
        .valign(Center)
        .halign(Center)
        .build();

    let substract = Button::builder()
        .label("-")
        .valign(Center)
        .halign(Center)
        .width_request(100)
        .build();

    let add = Button::builder()
        .label("+")
        .valign(Center)
        .halign(Center)
        .width_request(100)
        .build();

    let multiplicate = Button::builder()
        .label("*")
        .valign(Center)
        .halign(Center)
        .width_request(100)
        .build();
    let divise = Button::builder()
        .label("/")
        .valign(Center)
        .halign(Center)
        .width_request(100)
        .build();
    let second_number_box = Entry::builder()
        .placeholder_text("Enter the number")
        .valign(Center)
        .halign(Center)
        .width_request(200)
        .build();

    let first_number_box = Entry::builder()
        .placeholder_text("Enter the number")
        .valign(Center)
        .halign(Center)
        .width_request(200)
        .build();


    // add 2 number each other and write it on label
    let box_1_cln = first_number_box.clone();
    let box_2_cln = second_number_box.clone();
    let label_cln = label.clone();
    add.connect_clicked(move |_| {
        match (box_1_cln.text().parse::<f32>(), box_2_cln.clone().text().parse::<f32>()) {
            (Ok(value), Ok(value2)) => {
                let result = value + value2;
                label_cln.set_label(&result.to_string());
            }
            _ => {
                label_cln.set_label("Invalid Number");
            }
        }
    });

    // divisor
    let box_1_cln = first_number_box.clone();
    let box_2_cln = second_number_box.clone();
    let label_cln = label.clone();
    divise.connect_clicked(move |_| {
        match (box_1_cln.text().parse::<f32>(), box_2_cln.clone().text().parse::<f32>()) {
            (Ok(value), Ok(value2)) => {
                if value2 != 0.0 {
                    let result = value / value2;
                    label_cln.set_label(&result.to_string());
                }else {
                    label_cln.set_label("Cannot divide by 0");
                }
            }
            _ => {
                label_cln.set_label("Invalid Number");
            }
        }
    });

    // value multiplicater 
    let box_1_cln = first_number_box.clone();
    let box_2_cln = second_number_box.clone();
    let label_cln = label.clone();
    multiplicate.connect_clicked(move |_| {
        match (box_1_cln.text().parse::<f32>(), box_2_cln.clone().text().parse::<f32>()) {
            (Ok(value), Ok(value2)) => {
                let result = value * value2;
                label_cln.set_label(&result.to_string());
            }
            _ => {
                label_cln.set_label("Invalid Number");
            }
        }
    });


    // substract the number.
    let box_1_cln = first_number_box.clone();
    let box_2_cln = second_number_box.clone();
    let label_cln = label.clone();
    substract.connect_clicked(move |_| {
        match (box_1_cln.text().parse::<f32>(), box_2_cln.clone().text().parse::<f32>()) {
            (Ok(value), Ok(value2)) => {
                let result = value - value2;
                label_cln.set_label(&result.to_string());
            }
            _ => {
                label_cln.set_label("Invalid Number");
            }
        }
    });

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    let vbox2 = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    let vbox3 = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    vbox.set_valign(Center);
    vbox.set_halign(Center);
    vbox2.set_valign(Center);
    vbox2.set_halign(Center);
    vbox3.set_valign(Center);
    vbox3.set_halign(Center);

    vbox.append(&label);
    vbox.append(&first_number_box);
    vbox.append(&second_number_box);
    vbox2.append(&add);
    vbox2.append(&substract);
    vbox3.append(&multiplicate);
    vbox3.append(&divise);
    vbox.append(&vbox2);
    vbox.append(&vbox3);

    vbox
}
