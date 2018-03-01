//  This file is part of Sulis, a turn based RPG written in Rust.
//  Copyright 2018 Jared Stephen
//
//  Sulis is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  Sulis is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with Sulis.  If not, see <http://www.gnu.org/licenses/>

use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;

use sulis_core::ui::{Callback, Widget, WidgetKind};
use {Button};

const NAME: &str = "scrollbar";

pub struct Scrollbar {
    widget: Rc<RefCell<Widget>>,
    delta: u32,

    min_y: i32,
    cur_y: i32,
    max_y: i32,
}

impl Scrollbar {
    pub fn new(widget_to_scroll: &Rc<RefCell<Widget>>) -> Rc<RefCell<Scrollbar>> {
        Rc::new(RefCell::new(Scrollbar {
            widget: Rc::clone(widget_to_scroll),
            delta: 1,
            cur_y: 0,
            min_y: 0,
            max_y: 0,
        }))
    }

    fn update_children_position(&mut self, parent: &Rc<RefCell<Widget>>, dir: i32) {
        self.compute_min_max_y(parent);

        let old_y = self.cur_y;
        self.cur_y += dir * self.delta as i32;

        if self.cur_y < self.min_y { self.cur_y = self.min_y }
        else if self.cur_y > self.max_y { self.cur_y = self.max_y }

        update_children_recursive(parent, old_y - self.cur_y);
    }

    fn compute_min_max_y(&mut self, widget: &Rc<RefCell<Widget>>) {
        let mut max_y = 0;

        let len = widget.borrow().children.len();
        for i in 0..len {
            let child = Rc::clone(&widget.borrow().children[i]);

            max_y = cmp::max(max_y, child.borrow().state.position.y + child.borrow().state.size.height);
        }

        self.max_y = max_y + self.cur_y - widget.borrow().state.inner_size.height;

        if self.max_y < self.min_y { self.max_y = self.min_y }
    }
}

fn update_children_recursive(parent: &Rc<RefCell<Widget>>, delta: i32) {
    let len = parent.borrow().children.len();
    for i in 0..len {
        let child = Rc::clone(&parent.borrow().children[i]);
        let cur_x = child.borrow().state.position.x;
        let cur_y = child.borrow().state.position.y;
        child.borrow_mut().state.set_position(cur_x, cur_y + delta);

        update_children_recursive(&child, delta);
    }
}

impl WidgetKind for Scrollbar {
    fn get_name(&self) -> &str { NAME }

    fn as_any(&self) -> &Any { self }

    fn as_any_mut(&mut self) -> &mut Any { self }

    fn layout(&mut self, widget: &mut Widget) {
        widget.do_base_layout();

        if let Some(ref theme) = widget.theme {
            self.delta = theme.get_custom_or_default("scroll_delta", 1);
        }
    }

    fn on_add(&mut self, _widget: &Rc<RefCell<Widget>>) -> Vec<Rc<RefCell<Widget>>> {
        let up = Widget::with_theme(Button::empty(), "up");

        let widget_ref = Rc::clone(&self.widget);
        up.borrow_mut().state.add_callback(Callback::new(Rc::new(move |widget, _| {
            let parent = Widget::get_parent(&widget);
            let kind = Widget::downcast_kind_mut::<Scrollbar>(&parent);
            kind.update_children_position(&widget_ref, -1);
        })));

        let down = Widget::with_theme(Button::empty(), "down");

        let widget_ref = Rc::clone(&self.widget);
        down.borrow_mut().state.add_callback(Callback::new(Rc::new(move |widget, _| {
            let parent = Widget::get_parent(&widget);
            let kind = Widget::downcast_kind_mut::<Scrollbar>(&parent);
            kind.update_children_position(&widget_ref, 1);
        })));

        vec![up, down]
    }
}
