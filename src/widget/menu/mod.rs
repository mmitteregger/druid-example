pub use menu_group::*;
pub use menu_item::*;

use druid::widget::{CrossAxisAlignment, Flex, Padding};
use druid::{
    BoxConstraints, Env, Event, EventCtx, Insets, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    Point, Rect, Selector, Size, UpdateCtx, Widget, WidgetId, WidgetPod,
};

use crate::data::AppView;

mod menu_group;
mod menu_item;

pub struct Menu {
    id: WidgetId,
    child: WidgetPod<AppView, Box<dyn Widget<AppView>>>,
}

impl Menu {
    pub const MENU_ITEM_SELECTED: Selector<WidgetId> =
        Selector::new("druid-example.menu.menu-item-selected");

    pub fn new() -> Menu {
        let id = WidgetId::next();
        let child = Padding::new(
            Insets::new(0.0, 0.0, 15.0, 0.0),
            Flex::column()
                .must_fill_main_axis(true)
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(
                    MenuGroup::new("Group 1")
                        .with_item(MenuItem::new(id, "View 1", AppView::View1))
                        .with_item(MenuItem::new(id, "View 2", AppView::View2)),
                ),
        );
        Menu {
            id,
            child: WidgetPod::new(child).boxed(),
        }
    }
}

impl Widget<AppView> for Menu {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppView, env: &Env) {
        self.child.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppView, env: &Env) {
        self.child.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &AppView, data: &AppView, env: &Env) {
        self.child.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppView,
        env: &Env,
    ) -> Size {
        let size = self.child.layout(ctx, bc, data, env);
        self.child
            .set_layout_rect(ctx, data, env, Rect::from_origin_size(Point::ORIGIN, size));
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppView, env: &Env) {
        self.child.paint(ctx, data, env);
    }

    fn id(&self) -> Option<WidgetId> {
        Some(self.id)
    }
}
