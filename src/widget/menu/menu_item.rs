use druid::piet::{FontFamily, Text, TextLayout, TextLayoutBuilder};
use druid::widget::Label;
use druid::{
    BoxConstraints, Color, Command, Env, Event, EventCtx, Key, LayoutCtx, LifeCycle, LifeCycleCtx,
    MouseButton, PaintCtx, Point, Rect, RenderContext, Size, Target, UpdateCtx, Widget, WidgetId,
    WidgetPod,
};

use crate::data::AppView;
use crate::theme;
use crate::widget::{App, Menu};

const COLOR: Key<Color> = Key::new("druid-example.menu-item.color");

pub struct MenuItem {
    menu_id: WidgetId,
    label: WidgetPod<AppView, Label<AppView>>,
    view: AppView,
    color: Color,
}

impl MenuItem {
    pub fn new(menu_id: WidgetId, menu_name: &str, view: AppView) -> MenuItem {
        let label = Label::new(menu_name)
            .with_text_color(COLOR)
            .with_text_size(theme::MENU_ITEM_TEXT_SIZE);

        MenuItem {
            menu_id,
            label: WidgetPod::new(label),
            view,
            color: Color::BLACK,
        }
    }

    fn is_selected(&self, data: &AppView) -> bool {
        &self.view == data
    }

    fn compute_color(&mut self, is_hot: bool, data: &AppView, env: &Env) {
        self.color = if self.is_selected(data) {
            env.get(theme::MENU_ITEM_SELECTED_COLOR)
        } else if is_hot {
            env.get(theme::MENU_ITEM_HOT_COLOR)
        } else {
            env.get(theme::MENU_ITEM_DEFAULT_COLOR)
        };
    }
}

impl Widget<AppView> for MenuItem {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppView, env: &Env) {
        match event {
            Event::Command(command) => {
                if let Some(menu_item_id) = command.get(crate::widget::Menu::MENU_ITEM_SELECTED) {
                    let prev_selected = self.is_selected(data);
                    let new_selected = *menu_item_id == ctx.widget_id();

                    if new_selected {
                        ctx.submit_command(Command::new(
                            App::APP_VIEW_CHANGED,
                            self.view,
                            Target::Global,
                        ));
                    }
                    if prev_selected != new_selected {
                        ctx.request_layout();
                    }

                    return;
                }
            }
            Event::MouseMove(_mouse_event) => {
                #[cfg(target_os = "windows")]
                ctx.set_cursor(&druid::Cursor::OpenHand);
            }
            Event::MouseUp(mouse_event) => {
                if mouse_event.button == MouseButton::Left {
                    ctx.submit_command(Command::new(
                        Menu::MENU_ITEM_SELECTED,
                        ctx.widget_id(),
                        self.menu_id,
                    ));
                }
            }
            _ => {}
        }
        self.label.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppView, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                self.compute_color(ctx.is_hot(), data, env);
            }
            LifeCycle::HotChanged(is_hot) => {
                self.compute_color(*is_hot, data, env);
                ctx.request_layout();
            }
            _ => {}
        }
        self.label.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &AppView, data: &AppView, env: &Env) {
        self.compute_color(ctx.is_hot(), data, env);
        let mut new_env = env.clone();
        new_env.set(COLOR, self.color.clone());
        self.label.update(ctx, data, &new_env);
        // self.label.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppView,
        env: &Env,
    ) -> Size {
        let label_text_inset_x = 15.0;
        let label_text_inset_y = 0.0;

        // TODO: Uncommenting these lines fixes the panic, but this certainly "feels" wrong here.
        // // force rebuild to change the color
        // self.label.widget_mut().set_text_color(self.color.clone());

        let mut new_env = env.clone();
        new_env.set(COLOR, self.color.clone());

        let label_bc = bc.shrink(Size::new(label_text_inset_x, label_text_inset_y));
        let size = self.label.layout(ctx, &label_bc, data, &new_env);
        let origin = Point::new(label_text_inset_x, label_text_inset_y);
        self.label
            .set_layout_rect(ctx, data, &new_env, Rect::from_origin_size(origin, size));

        let my_size = Size::new(
            size.width + label_text_inset_x,
            size.height + label_text_inset_y,
        );
        let my_insets = self.label.compute_parent_paint_insets(my_size);
        ctx.set_paint_insets(my_insets);
        my_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppView, env: &Env) {
        let mut new_env = env.clone();
        new_env.set(COLOR, self.color.clone());
        self.label.paint(ctx, data, &new_env);

        if self.is_selected(data) {
            // added padding between the edges of the widget and the text.
            const LABEL_X_PADDING: f64 = 2.0;

            let font_size = 14.0;

            let arrow_text = "➤"; // \u{27A4}
            let text_layout_builder = ctx.text();
            let font = text_layout_builder
                .font_family(FontFamily::SANS_SERIF.name())
                .unwrap_or(FontFamily::SYSTEM_UI);
            let text_layout = text_layout_builder
                .new_text_layout(arrow_text)
                .font(font, font_size)
                .text_color(self.color.clone())
                .build()
                .unwrap();

            // Find the origin for the text: position the arrow in the middle
            let arrow_height = (ctx.size().height / 2.0) - (text_layout.size().height / 2.0);
            let origin = Point::new(LABEL_X_PADDING, arrow_height);

            ctx.draw_text(&text_layout, origin);
        }
    }
}
