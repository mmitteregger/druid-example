use druid::widget::{CrossAxisAlignment, Flex, Label};
use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size,
    UpdateCtx, Widget,
};

use crate::data::AppView;
use crate::widget::menu::MenuItem;

pub struct MenuGroup {
    group: Flex<AppView>,
}

impl MenuGroup {
    pub fn new(name: &str) -> MenuGroup {
        let group = Flex::column()
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .with_child(Label::new(name).with_text_size(20.0));

        MenuGroup { group }
    }

    pub fn with_item(mut self, menu_item: MenuItem) -> MenuGroup {
        self.group.add_child(menu_item);
        self
    }
}

impl Widget<AppView> for MenuGroup {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppView, env: &Env) {
        self.group.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppView, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                self.group.add_spacer(10.0);
            }
            _ => {}
        }
        self.group.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppView, data: &AppView, env: &Env) {
        self.group.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppView,
        env: &Env,
    ) -> Size {
        self.group.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppView, env: &Env) {
        self.group.paint(ctx, data, env);
    }
}
