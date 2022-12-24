mod animation;
mod database;
mod error;
mod helpers;
mod messages;
mod schemas;
mod widgets;
mod droptypes;
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
use widgets::{
    create_collection_table, ActionButton, AnimationProgress, CollectionFrame, MainTitle,
    MenuButton, TableInput, TableMultInput,
};

type MyResult<T> = Result<T, MyError>;
const BAR_SPEED: f64 = 0.00018;

#[tokio::main]
async fn main() -> MyResult<()> {
    let (tx, rx) = channel::<Message>();
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
    let mut drop_button = MenuButton::new(40, 300, 60, 60, "Drop Types", true);
    let mut product_button = MenuButton::new(40, 400, 60, 60, "Products", false);
    let mut pricing_button = MenuButton::new(40, 500, 60, 60, "Pricing", false);
    let mut admin_button = MenuButton::new(40, 600, 60, 60, "Admin", false);
    let mut find_button = ActionButton::new(1000, 160, 100, 30, "Find");
    let mut reset_button = ActionButton::new(1000, 160, 100, 30, "Reset");
    let mut add_button = ActionButton::new(450, 310, 100, 30, "Add");
    let mut title_input = TableInput::new(250, 120, 300, 30, "Drop Title");
    let mut description_input = TableMultInput::new(250, 190, 300, 100, "Drop Description");
    let mut search_title_input = TableInput::new(800, 120, 300, 30, "Search By Title");
    let mut all_drops_text = MainTitle::new(350, 460, 0, 0, "All Drop Types", 25);

    let number_of_cols = 4;
    let col_one_width = 250;
    let col_two_width = 400;
    let col_three_width = 100;
    let col_four_width = 100;
    let mut x_pos = 200;
    let mut y_pos = 0;
    let row_height = 130;
    let mut table_title_col: Vec<CollectionFrame> = Vec::new();
    let mut table_description_col: Vec<CollectionFrame> = Vec::new();
    let mut table_update_col: Vec<ActionButton> = Vec::new();
    let mut table_delete_col: Vec<ActionButton> = Vec::new();
    let mut anim_bar = AnimationProgress::new();
    let mut animation = false;

    let mut collection_scroll = Scroll::new(200, 480, 900, 350, None);
    collection_scroll.set_type(ScrollType::Vertical);

    drafts_button.hide();
    drop_button.hide();
    product_button.hide();
    pricing_button.hide();
    admin_button.hide();
    find_button.hide();
    reset_button.hide();
    add_button.hide();
    title_input.hide();
    description_input.hide();
    search_title_input.hide();
    all_drops_text.hide();
    collection_scroll.hide();

    window.make_resizable(true);
    window.end();
    window.show();

    window.set_callback(move |_| match fltk::app::event() {
        Event::Close => app.quit(),
        _ => {}
    });

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
                window.remove(&*anim_bar);
                main_text.set_pos(600, 50);
                drafts_button.show();
                drop_button.show();
                product_button.show();
                pricing_button.show();
                admin_button.show();
                find_button.show();
                reset_button.show();
                add_button.show();
                title_input.show();
                description_input.show();
                search_title_input.show();
                all_drops_text.show();
                collection_scroll.show();
                match get_collections(db).await {
                    Ok(collections) => {
                        create_collection_table(
                            &mut collection_scroll,
                            number_of_cols,
                            collections.len() as i32,
                            col_one_width,
                            col_two_width,
                            col_three_width,
                            col_four_width,
                            x_pos,
                            y_pos,
                            &collections,
                            row_height,
                            &mut table_title_col,
                            &mut table_description_col,
                            &mut table_update_col,
                            &mut table_delete_col,
                        );
                        redraw();
                    }
                    Err(_) => {}
                }
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
