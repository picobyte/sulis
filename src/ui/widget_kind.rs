use state::GameState;
use io::{InputAction, TextRenderer};
use io::event::ClickKind;
use resource::Point;
use ui::{Widget, WidgetState};

pub struct EmptyWidget { }
impl<'a> WidgetKind<'a> for EmptyWidget {
    fn get_name(&self) -> &str {
        "Empty"
    }
}

//// Trait for implementations of different Widgets.  This is held by a 'WidgetState'
//// object which contains the common functionality across all Widgets.
pub trait WidgetKind<'a> {
    fn super_draw_text_mode(&self, _widget: &Widget<'a>) {
        // TODO verify the state of the widget in the tree somehow
    }

    fn draw_text_mode(&self, _renderer: &mut TextRenderer, widget: &Widget<'a>) {
        self.super_draw_text_mode(widget);
    }

    fn get_name(&self) -> &str;

    fn on_add(&self, _widget_state: &mut WidgetState) { }

    fn on_mouse_click(&self, _state: &mut GameState, _widget: &mut Widget<'a>,
                      _kind: ClickKind, _mouse_pos: Point) -> bool {
        false
    }

    fn on_mouse_move(&self, _state: &mut GameState, _widget: &mut Widget<'a>,
                      _mouse_pos: Point) -> bool {
        false
    }

    fn on_mouse_enter(&self, _state: &mut GameState, widget: &mut Widget<'a>,
                      _mouse_pos: Point) -> bool {
        self.super_on_mouse_enter(widget);
        false
    }

    fn on_mouse_exit(&self, _state: &mut GameState, widget: &mut Widget<'a>,
                     _mouse_pos: Point) -> bool {
        self.super_on_mouse_exit(widget);
        false
    }

    fn on_mouse_scroll(&self, _state: &mut GameState, _widget: &mut Widget<'a>,
                         _scroll: i32, _mouse_pos: Point) -> bool {
        false
    }

    fn on_key_press(&self, _state: &mut GameState, _widget: &mut Widget<'a>,
                    _key: InputAction, _mouse_pos: Point) -> bool {
        false
    }

    fn super_on_mouse_enter(&self, widget: &mut Widget<'a>) {
        widget.state.set_mouse_inside(true);
    }

    fn super_on_mouse_exit(&self, widget: &mut Widget<'a>) {
        widget.state.set_mouse_inside(false);
    }
}
