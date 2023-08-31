use crate::gui::{
    theme::{Element, Renderer},
    Message, Tab,
};

use iced::widget::{column, text_input, Column};
use iced_aw::{TabBar, TabLabel};
use generational_array::{GenerationalIndex, GenerationalArray, GenerationalArrayResult};

pub fn tab_bar<'a>(active: usize, data: &[GenerationalIndex], gen_array: &GenerationalArray<Tab>) -> Column<'a, Message, Renderer> {
    if data.is_empty() {
        column!()
    } else {
        column!(head(active, data, gen_array), body(active, data, gen_array))
    }
}

fn head(active: usize, data: &[GenerationalIndex], gen_array: &GenerationalArray<Tab>) -> TabBar<Message, usize, Renderer> {
    let mut tab_bar = TabBar::new(Message::TabSelected).on_close(Message::TabClosed);

    for (i, geneartional_index) in data.iter().enumerate() {
        let resul = gen_array.get(geneartional_index);
        match resul {
            GenerationalArrayResult::None => todo!(),
            GenerationalArrayResult::OutDated => todo!(),
            GenerationalArrayResult::OutOfBounds => todo!(),
            GenerationalArrayResult::Some(tab) =>         match tab {
                Tab::File(file_tab) => {
                    let filename = file_tab
                        .path
                        .file_name() // this already checks the "empty" case
                        .and_then(|filename| filename.to_str())
                        .unwrap_or("New Tab");
    
                    tab_bar = tab_bar.push(i, TabLabel::Text(String::from(filename)));
                }
            },
        }

    }

    tab_bar.set_active_tab(&active)
}

fn body<'a>(active: usize, data: &[GenerationalIndex], gen_array: &GenerationalArray<Tab>) -> Element<'a, Message> {
    //let resul = gen_array.get(data.get(active).unwrap());
    match gen_array.get(data.get(active).unwrap()) {
        GenerationalArrayResult::None => todo!(),
        GenerationalArrayResult::OutDated => todo!(),
        GenerationalArrayResult::OutOfBounds => todo!(),
        GenerationalArrayResult::Some(active_tab) =>     match active_tab {
            Tab::File(file_tab) => text_input("placeholder", &file_tab.text)
                .on_input(Message::TextUpdate)
                .into(),
        },
    }


}
