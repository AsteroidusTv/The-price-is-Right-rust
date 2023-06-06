use rand::random;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Box, Orientation};

fn main() {
    let app = Application::builder()
        .application_id("Test")
        .build();

    app.connect_activate(buil_ui);
    app.run();
}
fn buil_ui(app : &Application) {
    let label = Label::builder()
        .label("Press flip coin to begin")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let button = Button::builder()
        .label("Flip coin")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let content = Box::new(Orientation::Vertical, 0);
    content.append(&label);
    content.append(&button);

    let window = ApplicationWindow::builder()
        .title("Test")
        .application(app)
        .child(&content)
        .build();

    button.connect_clicked(move |_| flip_coin(&label));

    window.show();
}

fn flip_coin(label: &Label) {
    if random() {
        label.set_text("Heads")
    }
    else {
       label.set_text("Tails")
    }
}