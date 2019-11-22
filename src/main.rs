mod widget;

use druid::widget::{Align, Button, Column, DynLabel, Padding};
use druid::{AppLauncher, Data, Lens, Widget, WindowDesc};

use crate::widget::TextEntry;

fn main() {
    let main_window = WindowDesc::new(ui_builder);
    let data = Model {
        count: 0u32,
        response: "".into(),
    };
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}

fn ui_builder() -> impl Widget<Model> {
    let label = DynLabel::new(|data: &Model, _env| format!("count: {}", data.count));
    let button = Button::new("increment", |_ctx, data: &mut Model, _env| {
        data.increment();
    });

    let response = DynLabel::new(|data: &Model, _env| format!("{}", data.response));

    let text_entry = TextEntry::new(|_ctx, data: &mut Model, _env, text| {
        match text.parse::<u32>() {
            Ok(n) => data.set(n),
            Err(_err) => data.response = format!("{:?} is not a number", text),
        }
    });

    let mut col = Column::new();
    col.add_child(Align::centered(Padding::new(5.0, label)), 1.0);
    col.add_child(Align::centered(Padding::new(5.0, response)), 1.0);
    col.add_child(Padding::new(5.0, button), 1.0);
    col.add_child(Padding::new(5.0, text_entry), 1.0);
    col
}

#[derive(Data, Clone, Lens)]
struct Model {
    count: u32,
    response: String,
}

impl Model {
    fn set(&mut self, n: u32) {
        if n > 10 {
            self.response = "Count above 10 not allowed".into();
        } else {
            self.count = n;
            self.response = "".into();
        }
    }

    fn increment(&mut self) {
        self.set(self.count + 1);
    }
}

