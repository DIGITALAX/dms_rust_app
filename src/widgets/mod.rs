use fltk::{
    app::redraw,
    button::Button,
    draw::set_cursor,
    enums::{Align, Color, Cursor, Event, Font},
    input::{Input, MultilineInput},
    prelude::*,
    text::TextDisplay,
    widget_extends,
};
pub mod droptypes;
pub mod sidebar;
pub mod animation;

widget_extends!(ActionButton, Button, act_btn);
widget_extends!(MainTitle, TextDisplay, mn_title);
widget_extends!(TableInput, Input, tbl_input);
widget_extends!(TableMultInput, MultilineInput, tbl_multi_input);

#[derive(Clone)]
pub struct ActionButton {
    act_btn: Button,
}

impl ActionButton {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let mut act_btn = Button::new(x, y, w, h, None).with_label(label);
        act_btn.clear_visible_focus();
        act_btn.set_label_color(Color::White);
        act_btn.set_selection_color(Color::Cyan);
        act_btn.handle(move |b, ev| match ev {
            Event::Enter => {
                set_cursor(Cursor::Hand);
                redraw();
                true
            }
            Event::Leave => {
                set_cursor(Cursor::Default);
                redraw();
                true
            }
            _ => false,
        });
        Self { act_btn }
    }
}

#[derive(Clone)]
pub struct MainTitle {
    mn_title: TextDisplay,
}

impl MainTitle {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str, text_size: i32) -> Self {
        let mut mn_title = TextDisplay::new(x, y, w, h, None).with_label(&label.to_uppercase());
        mn_title.set_label_font(Font::ScreenBold);
        mn_title.set_label_color(Color::White);
        mn_title.set_label_size(text_size);
        Self { mn_title }
    }
}
#[derive(Clone)]
pub struct TableInput {
    tbl_input: Input,
}

impl TableInput {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let mut tbl_input = Input::new(x, y, w, h, None)
            .with_label(label)
            .with_align(Align::TopLeft);
        tbl_input.set_label_color(Color::White);
        tbl_input.set_color(Color::White);
        tbl_input.set_selection_color(Color::Cyan);
        Self { tbl_input }
    }
}

#[derive(Clone)]
pub struct TableMultInput {
    tbl_multi_input: MultilineInput,
}

impl TableMultInput {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let mut tbl_multi_input = MultilineInput::new(x, y, w, h, None)
            .with_label(label)
            .with_align(Align::TopLeft);
        tbl_multi_input.set_label_color(Color::White);
        tbl_multi_input.set_color(Color::White);
        tbl_multi_input.set_selection_color(Color::Cyan);
        Self { tbl_multi_input }
    }
}

// pub fn update_delete_collection_table(
//     deleted_row: i32,
//     number_of_cols: i32,
//     number_of_rows: i32,
//     row_height: i32,
//     mut x_pos: i32,
//     mut y_pos: i32,
//     col_one_width: i32,
//     col_two_width: i32,
//     col_three_width: i32,
//     col_four_width: i32,
//     col_one: &mut Vec<Frame>,
//     col_two: &mut Vec<Frame>,
//     col_three: &mut Vec<ActionButton>,
//     col_four: &mut Vec<ActionButton>,
// ) {
//     for col in 0..number_of_cols {
//         let col_clone_one = col_one.clone();
//         let col_clone_two = col_two.clone();
//         let col_clone_three = col_three.clone();
//         let col_clone_four = col_four.clone();
//         if col == 0 {
//             for row in deleted_row..number_of_rows {
//                 col_one[row as usize].set_pos(
//                     col_clone_one[row as usize].x(),
//                     col_clone_one[row as usize].y() - row_height,
//                 );
//             }
//             x_pos += col_one_width;
//             y_pos = 0;
//         } else if col == 1 {
//             for row in deleted_row..number_of_rows {
//                 col_two[row as usize].set_pos(
//                     col_clone_two[row as usize].x(),
//                     col_clone_two[row as usize].y() - row_height,
//                 );
//             }
//             x_pos += col_two_width;
//             y_pos = col_three_width / 2;
//         } else if col == 2 {
//             for row in deleted_row..number_of_rows {
//                 col_three[row as usize].set_pos(
//                     col_clone_three[row as usize].x(),
//                     col_clone_three[row as usize].y() - row_height,
//                 );
//             }
//             x_pos += col_three_width + 10;
//             y_pos = col_four_width / 2;
//         } else {
//             for row in deleted_row..number_of_rows {
//                 col_four[row as usize].set_pos(
//                     col_clone_four[row as usize].x(),
//                     col_clone_four[row as usize].y() - row_height,
//                 );
//             }
//             x_pos += col_four_width;
//         }
//     }
// }

// pub fn update_add_collection_table(
//     number_of_cols: i32,
//     mut x_pos: i32,
//     col_one_width: i32,
//     col_two_width: i32,
//     col_three_width: i32,
//     col_four_width: i32,
//     col_one: &mut Vec<CollectionFrame>,
//     col_two: &mut Vec<CollectionFrame>,
//     col_three: &mut Vec<ActionButton>,
//     col_four: &mut Vec<ActionButton>,
//     collection: &DropType,
//     row_height: i32,
// ) {
//     for col in 0..number_of_cols {
//         if col == 0 {
//             let result =
//                 CollectionFrame::new(x_pos, 0, col_one_width, row_height, &collection.title);
//             col_one.push(result);
//             x_pos += col_one_width;
//         } else if col == 1 {
//             let result = CollectionFrame::new(
//                 x_pos,
//                 0, //total,
//                 col_two_width,
//                 row_height,
//                 &collection.description,
//             );
//             col_two.push(result);

//             x_pos += col_two_width;
//         } else if col == 2 {
//             let result =
//                 ActionButton::new(x_pos, col_three_width / 2, col_three_width, 30, "Update");
//             col_three.push(result);

//             x_pos += col_three_width + 10;
//         } else {
//             let result = ActionButton::new(x_pos, col_four_width / 2, col_four_width, 30, "Delete");
//             col_four.push(result);
//         }
//     }
// }

// pub fn create_find_table(
//     number_of_cols: i32,
//     number_of_rows: i32,
//     found_row: i32,
//     mut x_pos: i32,
//     col_one_width: i32,
//     col_two_width: i32,
//     col_three_width: i32,
//     col_four_width: i32,
//     row_height: i32,
//     collection: DropType,
//     collections: &Vec<DropType>,
//     col_one: &mut Vec<Frame>,
//     col_two: &mut Vec<Frame>,
//     col_three: &mut Vec<ActionButton>,
//     col_four: &mut Vec<ActionButton>,
// ) {
//     for col in 0..number_of_cols {
//         if col == 0 {
//             for row in 0..number_of_rows {
//                 if row == found_row {
//                     col_one[row as usize].set_pos(x_pos, 0)
//                 } else {
//                     col_one[row as usize].hide();
//                 }
//             }
//             x_pos += col_one_width;
//         } else if col == 1 {
//             for row in 0..number_of_rows {
//                 if row == found_row {
//                     col_two[row as usize].set_pos(x_pos, 0)
//                 } else {
//                     col_two[row as usize].hide();
//                 }
//             }
//             x_pos += col_two_width;
//         } else if col == 2 {
//             for row in 0..number_of_rows {
//                 if row == found_row {
//                     col_three[row as usize].set_pos(x_pos, col_three_width / 2)
//                 } else {
//                     col_three[row as usize].hide();
//                 }
//             }
//             x_pos += col_three_width + 10;
//         } else if col == 3 {
//             for row in 0..number_of_rows {
//                 if row == found_row {
//                     col_four[row as usize].set_pos(x_pos, col_four_width / 2)
//                 } else {
//                     col_four[row as usize].hide();
//                 }
//             }
//         }
//     }
// }

// pub fn get_row(
//     number_of_cols: i32,
//     number_of_rows: i32,
//     collections: &Vec<DropType>,
//     collection: &DropType,
// ) -> Result<i32, ()> {
//     let mut row_value = 0;
//     for col in 0..number_of_cols {
//         if col == 0 {
//             for row in 0..number_of_rows {
//                 if collection.title == collections[row as usize].title {
//                     row_value = row;
//                 }
//             }
//         }
//     }
//     Ok(row_value)
// }

// pub fn create_collection_table(
//     collection_scroll: &mut Scroll,
//     number_of_cols: i32,
//     number_of_rows: i32,
//     col_one_width: i32,
//     col_two_width: i32,
//     col_three_width: i32,
//     col_four_width: i32,
//     mut x_pos: i32,
//     mut y_pos: i32,
//     collections: &Vec<DropType>,
//     row_height: i32,
//     col_one: &mut Vec<CollectionFrame>,
//     col_two: &mut Vec<CollectionFrame>,
//     col_three: &mut Vec<ActionButton>,
//     col_four: &mut Vec<ActionButton>,
// ) {
//     for col in 0..number_of_cols {
//         if col == 0 {
//             for row in 0..number_of_rows {
//                 let result = CollectionFrame::new(
//                     x_pos,
//                     y_pos,
//                     col_one_width,
//                     row_height,
//                     &collections[row as usize].title,
//                 );
//                 collection_scroll.add(&*result);
//                 col_one.push(result);
//                 y_pos += row_height;
//             }
//             x_pos += col_one_width;
//             y_pos = 0;
//         } else if col == 1 {
//             for row in 0..number_of_rows {
//                 let result = CollectionFrame::new(
//                     x_pos,
//                     y_pos,
//                     col_two_width,
//                     row_height,
//                     &collections[row as usize].description,
//                 );
//                 collection_scroll.add(&*result);
//                 col_two.push(result);
//                 y_pos += row_height;
//             }
//             x_pos += col_two_width;
//             y_pos = col_three_width / 2;
//         } else if col == 2 {
//             for row in 0..number_of_rows {
//                 let result = ActionButton::new(x_pos, y_pos, col_three_width, 30, "Update");
//                 collection_scroll.add(&*result);
//                 col_three.push(result);
//                 y_pos += row_height;
//             }
//             x_pos += col_three_width + 10;
//             y_pos = col_four_width / 2;
//         } else {
//             for row in 0..number_of_rows {
//                 let result = ActionButton::new(x_pos, y_pos, col_four_width, 30, "Delete");
//                 collection_scroll.add(&*result);
//                 col_four.push(result);
//                 y_pos += row_height;
//             }
//             x_pos += col_four_width;
//         }
//     }
// }

// pub fn create_product_table(
//     product_scroll: &mut Scroll,
//     number_of_cols: i32,
//     number_of_rows: i32,
//     col_one_width: i32,
//     col_two_width: i32,
//     col_three_width: i32,
//     col_four_width: i32,
//     mut x_pos: i32,
//     mut y_pos: i32,
//     products: &Vec<Product>,
//     row_height: i32,
//     col_one: &mut Vec<CollectionFrame>,
//     col_two: &mut Vec<CollectionFrame>,
//     col_three: &mut Vec<ActionButton>,
//     col_four: &mut Vec<ActionButton>,
// ) {
//     for col in 0..number_of_cols {
//         if col == 0 {
//             for row in 0..number_of_rows {
//                 let result = CollectionFrame::new(
//                     x_pos,
//                     y_pos,
//                     col_one_width,
//                     row_height,
//                     &products[row as usize].name,
//                 );
//                 product_scroll.add(&*result);
//                 col_one.push(result);
//                 y_pos += row_height;
//             }
//             x_pos += col_one_width;
//             y_pos = 0;
//         } else if col == 1 {
//             for row in 0..number_of_rows {
//                 let result = CollectionFrame::new(
//                     x_pos,
//                     y_pos,
//                     col_two_width,
//                     row_height,
//                     &products[row as usize].description,
//                 );
//                 product_scroll.add(&*result);
//                 col_two.push(result);
//                 y_pos += row_height;
//             }
//             x_pos += col_two_width;
//             y_pos = col_three_width / 2;
//         } else if col == 2 {
//             for row in 0..number_of_rows {
//                 let result = ActionButton::new(x_pos, y_pos, col_three_width, 30, "Update");
//                 product_scroll.add(&*result);
//                 col_three.push(result);
//                 y_pos += row_height;
//             }
//             x_pos += col_three_width + 10;
//             y_pos = col_four_width / 2;
//         } else {
//             for row in 0..number_of_rows {
//                 let result = ActionButton::new(x_pos, y_pos, col_four_width, 30, "Delete");
//                 product_scroll.add(&*result);
//                 col_four.push(result);
//                 y_pos += row_height;
//             }
//             x_pos += col_four_width;
//         }
//     }
// }

