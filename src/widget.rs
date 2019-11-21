use druid::{
    BaseState, BoxConstraints, Env, Event, EventCtx, LayoutCtx, PaintCtx, UpdateCtx, Widget,
    KeyCode, KeyEvent,
};

use druid::widget::TextBox;
use druid::kurbo::Size;

pub struct TextEntry<T> {
    child: Box<dyn Widget<String>>,
    cache: String,
    action: Box<dyn Fn(&mut EventCtx, &mut T, &Env, String) + 'static>,
}

impl<T> TextEntry<T> {
    pub fn new(action: impl Fn(&mut EventCtx, &mut T, &Env, String) + 'static) -> Self {
        Self {
            child: Box::new(TextBox::new()),
            cache: "".into(),
            action: Box::new(action),
        }
    }
}


impl<T> Widget<T> for TextEntry<T> {
    fn paint(&mut self, paint_ctx: &mut PaintCtx, base_state: &BaseState, _data: &T, env: &Env) {
        self.child.paint(paint_ctx, base_state, &self.cache, env);
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &T,
        env: &Env,
    ) -> Size {

        self.child.layout(layout_ctx, bc, &self.cache, env)
    }

    fn event(&mut self, event: &Event, ctx: &mut EventCtx, data: &mut T, env: &Env) {
        match event {
            Event::KeyUp(KeyEvent { key_code, .. } ) if *key_code == KeyCode::Return => {
                // send contents of cache upstream
                (self.action)(ctx, data, env, self.cache.trim().to_owned());
                self.cache.clear();
            },
            _ => self.child.event(event, ctx, &mut self.cache, env),
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: Option<&T>, data: &T, env: &Env) {
    }
}
