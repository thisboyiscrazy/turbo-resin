// SPDX-License-Identifier: GPL-3.0-or-later

use futures::Future;
use lvgl::{
    style::State,
    widgets::*,
    prelude::*,
};
use alloc::format;

use lvgl::cstr_core::{CStr,CString};
use crate::{
    TaskRunner,
    drivers::zaxis::{
        self,
        prelude::*,
    }, util::CancellableTask,
};
use lvgl::core::Display;
use lvgl::core::Event;
use lvgl::core::InputDevice;
use lvgl::core::InputDeviceState;
use lvgl::core::Lvgl;
use lvgl::core::ObjExt;
use lvgl::core::Screen;
use lvgl::core::TouchPad;
use lvgl::style::Style;
use lvgl::{style::Align, style::Flag, style::GridAlign};
use lvgl::style;

pub struct MoveZ {
    style: Style,
    col_dsc: Box<[i16; 4]>,
    row_dsc: Box<[i16; 5]>,
    btn_0_1mm: Btn<MoveZ>,
    btn_1mm: Btn<MoveZ>,
    btn_10mm: Btn<MoveZ>,
    btn_up: Btn<MoveZ>,
    btn_home: Btn<MoveZ>,
    btn_down: Btn<MoveZ>,
    current_pos: Label<MoveZ>,

    task_runner: &'static TaskRunner<Task>,
    zaxis: &'static zaxis::MotionControlAsync,

}

use alloc::boxed::Box;

impl MoveZ {
    pub fn new(
        screen: &mut Screen<Self>,
        task_runner: &'static mut TaskRunner<Task>,
        zaxis: &'static zaxis::MotionControlAsync,
    ) -> Self {
        let mut style = Style::new();
        style.set_pad_all(10);

        screen.add_style(&mut style, 0);

        let mut col_dsc = Box::new([
            style::grid_free(1),
            style::grid_free(1),
            style::grid_free(1),
            style::grid_last(),
        ]);
        let mut row_dsc = Box::new([
            style::grid_free(1),
            style::grid_free(1),
            style::grid_free(1),
            style::grid_free(1),
            style::grid_last(),
        ]);

        screen.set_grid_dsc_array(col_dsc.as_mut_ptr(), row_dsc.as_mut_ptr());

        let btn_0_1mm = Btn::new(screen).apply(|obj| {
            obj.on_event(Event::Clicked, |context| {
                //println!("Clicked on: 0.1mm");
            })
            .set_grid_cell(
                GridAlign::Stretch,
                0,
                1,
                GridAlign::Stretch,
                0,
                1,
            );

            let mut btn_lbl = Label::new(obj);
            btn_lbl.set_text(CString::new("0.1mm").unwrap().as_c_str());
            btn_lbl.align_to(obj, Align::Center, 0, 0);
        });

        let btn_1mm = Btn::new(screen).apply(|obj| {
            obj.on_event(Event::Clicked, |context| {
                //println!("Clicked on: 1mm");
            })
            .set_grid_cell(
                GridAlign::Stretch,
                1,
                1,
                GridAlign::Stretch,
                0,
                1,
            );

            let mut btn_lbl = Label::new(obj);
            btn_lbl.set_text(CString::new("1mm").unwrap().as_c_str());
            btn_lbl.align_to(obj, Align::Center, 0, 0);
        });

        let btn_10mm = Btn::new(screen).apply(|obj| {
            obj.on_event(Event::Clicked, |context| {
                //println!("Clicked on: 10mm");
            })
            .set_grid_cell(
                GridAlign::Stretch,
                2,
                1,
                GridAlign::Stretch,
                0,
                1,
            );

            let mut btn_lbl = Label::new(obj);
            btn_lbl.set_text(CString::new("10mm").unwrap().as_c_str());
            btn_lbl.align_to(obj, Align::Center, 0, 0);
        });

        let btn_up = Btn::new(screen).apply(|obj| {
            obj.on_event(Event::Clicked, |context| {
                //println!("Clicked on: up");
            })
            .set_grid_cell(
                GridAlign::Stretch,
                0,
                1,
                GridAlign::Stretch,
                1,
                1,
            );

            let mut btn_lbl = Label::new(obj);
            btn_lbl.set_text(CString::new("UP").unwrap().as_c_str());
            btn_lbl.align_to(obj, Align::Center, 0, 0);
        });

        let btn_home = Btn::new(screen).apply(|obj| {
            obj.on_event(Event::Clicked, |context| {
                //println!("Clicked on: home");
            })
            .set_grid_cell(
                GridAlign::Stretch,
                1,
                1,
                GridAlign::Stretch,
                1,
                1,
            );

            let mut btn_lbl = Label::new(obj);
            btn_lbl.set_text(CString::new("HOME").unwrap().as_c_str());
            btn_lbl.align_to(obj, Align::Center, 0, 0);
        });

        let btn_down = Btn::new(screen).apply(|obj| {
            obj.on_event(Event::Clicked, |context| {
                //println!("Clicked on: down");
            })
            .set_grid_cell(
                GridAlign::Stretch,
                2,
                1,
                GridAlign::Stretch,
                1,
                1,
            );

            let mut btn_lbl = Label::new(obj);
            btn_lbl.set_text(CString::new("DOWN").unwrap().as_c_str());
            btn_lbl.align_to(obj, Align::Center, 0, 0);
        });

        let btn_stop = Btn::new(screen).apply(|obj| {
            obj.on_event(Event::Clicked, |context| {
                //println!("Clicked on: stop");
            })
            .set_grid_cell(
                GridAlign::Stretch,
                0,
                3,
                GridAlign::Stretch,
                2,
                1,
            );

            let mut btn_lbl = Label::new(obj);
            btn_lbl.set_text(CString::new("STOP").unwrap().as_c_str());
            btn_lbl.align_to(obj, Align::Center, 0, 0);
        });

        let mut current_pos = Label::new(screen).apply(|obj| {
            obj.set_text(CString::new("0.0").unwrap().as_c_str());
            obj.set_grid_cell(
                GridAlign::Center,
                0,
                3,
                GridAlign::Center,
                3,
                1,
            );
        });

        Self {
            style,
            col_dsc,
            row_dsc,
            btn_0_1mm,
            btn_1mm,
            btn_10mm,
            btn_up,
            btn_home,
            btn_down,
            current_pos,
            task_runner,
            zaxis,
        }
    }
    pub fn refresh(&mut self) {}
}

#[derive(Debug, Clone, Copy)]
pub enum Task {
    MoveUp,
    MoveDown,
    MoveZero,
}

impl CancellableTask for Task {
    type Context = zaxis::MotionControlAsync;

    type RunFuture<'a> = impl Future<Output = ()> + 'a where Self: 'a;
    type CancelFuture<'a> = impl Future<Output = ()> + 'a where Self: 'a;

    fn run<'a>(&'a self, mc: &'a mut zaxis::MotionControlAsync) -> Self::RunFuture<'a> {
        async move {
            match self {
                Self::MoveUp => mc.set_target_relative(40.0.mm()),
                Self::MoveDown => mc.set_target_relative((-40.0).mm()),
                Self::MoveZero => {
                    let s = mc.get_max_speed();
                    zaxis::calibrate_origin(mc, None).await;
                    // FIXME we don't restore the original speed when the task is cancelled.
                    mc.set_max_speed(s);
                    mc.set_target(0.0.mm());
                }
            };
            mc.wait(zaxis::Event::Idle).await;
        }
    }

    fn cancel<'a>(&'a self, mc: &'a mut zaxis::MotionControlAsync) -> Self::CancelFuture<'a> {
        async move {
            // The task was cancelled
            mc.stop();
            mc.wait(zaxis::Event::Idle).await;
        }
    }
}
