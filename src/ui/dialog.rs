use fltk::{prelude::*, *};
use std::{cell::Cell, rc::Rc};

pub fn choice(text: &str, ok: &str, cancel: &str) -> bool {
    let choice = Rc::new(Cell::new(false));

    let mut win = window::Window::default()
        .with_size(420, 128)
        .center_screen();
    win.make_modal(true);
    win.end();

    let mut content_group = group::Flex::default()
        .column()
        .size_of(&win)
        .center_of(&win);
    win.add_resizable(&content_group);

    let label = frame::Frame::default_fill().with_label(text);
    content_group.add_resizable(&label);

    let mut button_group = group::Flex::default_fill().row();
    button_group.set_margin(8);
    content_group.fixed(&button_group, 42);

    let mut ok_button = button::Button::default().with_label(ok);
    ok_button.set_callback({
        let choice = choice.clone();
        let mut win = win.clone();
        move |_| {
            choice.replace(true);
            win.hide();
        }
    });
    button_group.add_resizable(&ok_button);

    let mut cancel_button = button::Button::default().with_label(cancel);
    cancel_button.set_callback({
        let choice = choice.clone();
        let mut win = win.clone();
        move |_| {
            choice.replace(false);
            win.hide();
        }
    });
    button_group.add_resizable(&cancel_button);

    win.make_resizable(false);
    win.show();

    // Wait for the window to be hidden.
    while win.shown() {
        app::wait();
    }

    choice.get()
}

pub fn message(text: &str, close: &str) {
    let mut win = window::Window::default()
        .with_size(420, 128)
        .center_screen();
    win.make_modal(true);
    win.end();

    let mut content_group = group::Flex::default()
        .column()
        .size_of(&win)
        .center_of(&win);
    win.add_resizable(&content_group);

    let label = frame::Frame::default_fill().with_label(text);
    content_group.add_resizable(&label);

    let mut button_group = group::Flex::default_fill().row();
    button_group.set_margin(8);
    content_group.fixed(&button_group, 42);

    let mut close_button = button::Button::default().with_label(close);
    close_button.set_callback({
        let mut win = win.clone();
        move |_| {
            win.hide();
        }
    });
    button_group.add_resizable(&close_button);


    win.make_resizable(false);
    win.show();

    // Wait for the window to be hidden.
    while win.shown() {
        app::wait();
    }
}