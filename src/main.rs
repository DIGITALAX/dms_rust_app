mod animation;
mod database;
mod droptypes;
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
    prelude::*,
    window::Window,
};
use messages::MainMessage;
use widgets::{AnimationProgress, MainTitle, MenuButton};

type MyResult<T> = Result<T, MyError>;
const BAR_SPEED: f64 = 0.00018;

#[tokio::main]
async fn main() -> MyResult<()> {
    let (tx, rx) = channel::<MainMessage>();
    let app = App::default();
    set_font(Font::Screen);
    set_background_color(17, 19, 19);
    set_color(Color::White, 231, 242, 251);
    set_color(Color::Cyan, 187, 238, 255);
    let mut window = Window::default()
        .with_size(1200, 900)
        .with_label("Drop Management System");
    let mut main_text = MainTitle::new(600, 400, 0, 0, "Drop Management System", 30);
    let mut drafts_button = MenuButton::new(40, 200, 60, 60, "Drafts", true);
    let mut drop_button = MenuButton::new(40, 300, 60, 60, "Drop Types", false);
    let mut product_button = MenuButton::new(40, 400, 60, 60, "Products", false);
    let mut pricing_button = MenuButton::new(40, 500, 60, 60, "Pricing", false);
    let mut admin_button = MenuButton::new(40, 600, 60, 60, "Admin", false);

    let mut anim_bar = AnimationProgress::new();
    let mut animation = false;

    drafts_button.hide();
    drop_button.hide();
    product_button.hide();
    pricing_button.hide();
    admin_button.hide();

    window.make_resizable(true);
    window.end();
    window.show();

    window.set_callback(move |_| match fltk::app::event() {
        Event::Close => app.quit(),
        _ => {}
    });

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
        MainMessage::MenuSelect(
            drop_button_clone,
            product_button_clone,
            admin_button_clone,
            drafts_button_clone,
            pricing_button_clone,
        ),
    );
    product_button.emit(
        tx.clone(),
        MainMessage::MenuSelect(
            product_button_clone_one,
            drop_button_clone_one,
            admin_button_clone_one,
            drafts_button_clone_one,
            pricing_button_clone_one,
        ),
    );
    admin_button.emit(
        tx.clone(),
        MainMessage::MenuSelect(
            admin_button_clone_two,
            drop_button_clone_two,
            product_button_clone_two,
            drafts_button_clone_two,
            pricing_button_clone_two,
        ),
    );
    drafts_button.emit(
        tx.clone(),
        MainMessage::MenuSelect(
            drafts_button_clone_three,
            drop_button_clone_three,
            product_button_clone_three,
            admin_button_clone_three,
            pricing_button_clone_three,
        ),
    );
    pricing_button.emit(
        tx.clone(),
        MainMessage::MenuSelect(
            pricing_button_clone_four,
            drop_button_clone_four,
            product_button_clone_four,
            admin_button_clone_four,
            drafts_button_clone_four,
        ),
    );

    tx.send(MainMessage::Start);

    while app.wait() {
        match rx.recv() {
            Some(MainMessage::Start) => {
                start_animation(&mut animation, &mut anim_bar);
                let db = connect().await?;
                tx.send(MainMessage::Ready(Some(db)))
            }
            Some(MainMessage::Ready(db)) => {
                stop_animation(&mut animation, &mut anim_bar);
                window.remove(&*anim_bar);
                main_text.set_pos(600, 50);
                drafts_button.show();
                drop_button.show();
                product_button.show();
                pricing_button.show();
                admin_button.show();

                redraw();
            }

            Some(MainMessage::MenuSelect(mut b, mut b1, mut b2, mut b3, mut b4)) => {
                if b.color() == Color::White {
                    b.set_color(Color::Cyan);
                    b1.set_color(Color::White);
                    b2.set_color(Color::White);
                    b3.set_color(Color::White);
                    b4.set_color(Color::White);
                }
                redraw();
            }

            Some(MainMessage::Error) => {
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
