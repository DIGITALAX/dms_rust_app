mod animation;
mod database;
mod error;
mod helpers;
mod messages;
mod schemas;
mod widgets;
use animation::{start_animation, stop_animation};
use database::connect;
use error::MyError;
use fltk::{
    app::{channel, redraw, set_background_color, set_color, set_font, App},
    enums::{Color, Event, Font},
    group::{Scroll, ScrollType},
    prelude::*,
    window::Window,
};
use helpers::get_collections;
use messages::Message;
use mongodb::Database;
use widgets::droptypes::create_droptypes_table;
use crate::widgets::{sidebar::MenuButton, animation::AnimationProgress};
use widgets::MainTitle;

type MyResult<T> = Result<T, MyError>;
const BAR_SPEED: f64 = 0.00018;

#[tokio::main]
async fn main() -> MyResult<()> {
    let (tx, rx) = channel::<Message>();
    let database: Option<Database> = None;
    let app = App::default();
    set_font(Font::Screen);
    set_background_color(17, 19, 19);
    set_color(Color::White, 231, 242, 251);
    set_color(Color::Cyan, 187, 238, 255);
    set_color(Color::DarkYellow, 201, 216, 228);
    let mut app_window = Window::default()
        .with_size(1200, 900)
        .with_label("Drop Management System");
    let mut main_text = MainTitle::new(600, 400, 0, 0, "Drop Management System", 20);
    let mut anim_bar = AnimationProgress::new();
    let mut animation = false;

    // sidebar window
    let mut sidebar_window = Window::new(0, 100, 120, 900, None);
    sidebar_window.begin();
    let mut drafts_button = MenuButton::new(40, 100, 60, 60, "Drafts", true);
    let mut drop_button = MenuButton::new(40, 200, 60, 60, "Drop Types", false);
    let mut product_button = MenuButton::new(40, 300, 60, 60, "Products", false);
    let mut pricing_button = MenuButton::new(40, 400, 60, 60, "Pricing", false);
    let mut admin_button = MenuButton::new(40, 500, 60, 60, "Admin", false);
    sidebar_window.end();

    // drop types window
    let mut droptypes_scroll = Scroll::new(130, 130, 1050, 750, None);
    droptypes_scroll.set_type(ScrollType::Vertical);
    droptypes_scroll.begin();
    let number_of_cols = 4;
    let x_pos = 130;
    let y_pos = 0;
    let row_height = 200;
    let col_width = 260;
    let drop_frame_height = 180;
    let drop_frame_width = 240;
    droptypes_scroll.end();

    // hide all widgets for starting animation
    sidebar_window.hide();
    droptypes_scroll.hide();

    app_window.make_resizable(true);
    app_window.end();
    app_window.show();

    app_window.set_callback(move |_| match fltk::app::event() {
        Event::Close => app.quit(),
        _ => {}
    });

    // sidebar buttons
    let drop_button_clone = drop_button.clone();
    let product_button_clone = product_button.clone();
    let admin_button_clone = admin_button.clone();
    let drafts_button_clone = drafts_button.clone();
    let pricing_button_clone = pricing_button.clone();
    let drop_button_clone_one = drop_button.clone();
    let product_button_clone_one = product_button.clone();
    let admin_button_clone_one = admin_button.clone();
    let drafts_button_clone_one = drafts_button.clone();
    let pricing_button_clone_one = pricing_button.clone();
    let drop_button_clone_two = drop_button.clone();
    let product_button_clone_two = product_button.clone();
    let admin_button_clone_two = admin_button.clone();
    let drafts_button_clone_two = drafts_button.clone();
    let pricing_button_clone_two = pricing_button.clone();
    let drop_button_clone_three = drop_button.clone();
    let product_button_clone_three = product_button.clone();
    let admin_button_clone_three = admin_button.clone();
    let drafts_button_clone_three = drafts_button.clone();
    let pricing_button_clone_three = pricing_button.clone();
    let drop_button_clone_four = drop_button.clone();
    let product_button_clone_four = product_button.clone();
    let admin_button_clone_four = admin_button.clone();
    let drafts_button_clone_four = drafts_button.clone();
    let pricing_button_clone_four = pricing_button.clone();

    drop_button.emit(
        tx.clone(),
        Message::MenuSelect(
            drop_button_clone,
            product_button_clone,
            admin_button_clone,
            drafts_button_clone,
            pricing_button_clone,
            database.clone(),
        ),
    );

    product_button.emit(
        tx.clone(),
        Message::MenuSelect(
            product_button_clone_one,
            drop_button_clone_one,
            admin_button_clone_one,
            drafts_button_clone_one,
            pricing_button_clone_one,
            database.clone(),
        ),
    );
    admin_button.emit(
        tx.clone(),
        Message::MenuSelect(
            admin_button_clone_two,
            drop_button_clone_two,
            product_button_clone_two,
            drafts_button_clone_two,
            pricing_button_clone_two,
            database.clone(),
        ),
    );
    drafts_button.emit(
        tx.clone(),
        Message::MenuSelect(
            drafts_button_clone_three,
            drop_button_clone_three,
            product_button_clone_three,
            admin_button_clone_three,
            pricing_button_clone_three,
            database.clone(),
        ),
    );
    pricing_button.emit(
        tx.clone(),
        Message::MenuSelect(
            pricing_button_clone_four,
            drop_button_clone_four,
            product_button_clone_four,
            admin_button_clone_four,
            drafts_button_clone_four,
            database.clone(),
        ),
    );

    tx.send(Message::Start);

    while app.wait() {
        match rx.recv() {
            Some(Message::Start) => {
                start_animation(&mut animation, &mut anim_bar);
                let db = connect().await?;
                tx.send(Message::Ready(Some(db)))
            }
            Some(Message::Ready(db)) => {
                stop_animation(&mut animation, &mut anim_bar);
                app_window.remove(&*anim_bar);
                main_text.set_pos(200, 50);
                sidebar_window.show();
                redraw();
                tx.send(Message::DropTypes(db));
            }
            Some(Message::MenuSelect(mut b, mut b1, mut b2, mut b3, mut b4, db)) => {
                if b.color() == Color::White {
                    b.set_color(Color::Cyan);
                    b.set_label_color(Color::Cyan);
                    b1.set_color(Color::White);
                    b1.set_label_color(Color::White);
                    b2.set_color(Color::White);
                    b2.set_label_color(Color::White);
                    b3.set_color(Color::White);
                    b3.set_label_color(Color::White);
                    b4.set_color(Color::White);
                    b4.set_label_color(Color::White);
                }
                if b.label() == "Drop Types" {
                    tx.send(Message::DropTypes(db));
                } else {
                    droptypes_scroll.hide();
                }
                redraw();
            }
            Some(Message::DropTypes(db)) => {
                droptypes_scroll.show();
                match get_collections(db).await {
                    Ok(droptypes) => {
                        create_droptypes_table(
                            &mut droptypes_scroll,
                            number_of_cols,
                            droptypes.len() as i32,
                            drop_frame_height,
                            drop_frame_width,
                            row_height,
                            col_width,
                            x_pos,
                            y_pos,
                            &droptypes,
                            tx.clone()
                        );
                        droptypes_scroll.redraw();
                    }
                    Err(_) => {}
                }
            }
            Some(Message::DropTypeModify(frame)) => {

            }
            Some(Message::Error) => {
                // todo!()
            }
            None => {
                if animation {
                    let mut val = anim_bar.value();
                    if val < anim_bar.maximum() {
                        val += BAR_SPEED;
                        anim_bar.set_value(val);
                    }
                }
            }
        }
    }
    Ok(())
}
