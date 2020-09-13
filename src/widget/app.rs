use druid::widget::{CrossAxisAlignment, Flex, ViewSwitcher, Widget, WidgetExt};
use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Point,
    Rect, Selector, Size, UpdateCtx, WidgetPod,
};

use crate::data::{AppData, AppView};
use crate::theme;
use crate::widget::{Menu, View1, View2};

pub struct App {
    child: WidgetPod<AppData, Box<dyn Widget<AppData>>>,
}

impl App {
    pub const APP_VIEW_CHANGED: Selector<AppView> = Selector::new("druid-example.app.view-changed");

    pub fn new() -> App {
        let child = Flex::<AppData>::row()
            .must_fill_main_axis(true)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .with_child(Menu::new().lens(AppData::VIEW))
            .with_flex_child(
                ViewSwitcher::new(
                    |data: &AppData, _env| data.view(),
                    |view, _data, _env| match view {
                        AppView::View1 => Box::new(View1::new().lens(AppData::VIEW_DATA)),
                        AppView::View2 => Box::new(View2::new().lens(AppData::VIEW_DATA)),
                    },
                ),
                1.0,
            )
            .padding(10.0)
            .background(druid::theme::WINDOW_BACKGROUND_COLOR)
            .env_scope(|env: &mut Env, _data: &AppData| theme::init(env));
        App {
            child: WidgetPod::new(child).boxed(),
        }
    }
}

impl Widget<AppData> for App {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppData, env: &Env) {
        match event {
            Event::Command(command) => {
                if let Some(view) = command.get(App::APP_VIEW_CHANGED) {
                    data.set_view(*view);
                    ctx.set_handled();
                    return;
                }
            }
            _ => {}
        }

        self.child.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppData, env: &Env) {
        self.child.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &AppData, data: &AppData, env: &Env) {
        self.child.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppData,
        env: &Env,
    ) -> Size {
        let size = self.child.layout(ctx, bc, data, env);
        self.child
            .set_layout_rect(ctx, data, env, Rect::from_origin_size(Point::ORIGIN, size));
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppData, env: &Env) {
        self.child.paint(ctx, data, env);
    }
}
