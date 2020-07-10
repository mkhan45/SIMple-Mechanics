use imgui::*;
use std::fs;
use std::path::Path;

use crate::gui::imgui_wrapper::*;
use crate::{
    resources::{CreateMass, ShapeInfo, CreateElasticity, CreateFriction},
    Vector,
};

use specs::prelude::*;

macro_rules! signal_button {
    ( $label:expr, $signal:expr, $ui:expr, $signals:expr) => {
        if $ui.small_button(im_str!($label)) {
            $signals.push($signal);
        }
    };
}

macro_rules! int_slider {
    ( $ui:expr, $label:expr, $num:expr, $min:expr, $max:expr ) => {
        let mut num_i32 = *$num as i32;
        $ui.drag_int(im_str!($label), &mut num_i32)
            .min($min)
            .speed(0.05 * (*$num as f32).powf(1.0 / 3.0))
            .max($max)
            .build();
        *$num = (num_i32 as usize).min($max).max($min);
    };
}

pub fn make_menu_bar(ui: &mut imgui::Ui, signals: &mut Vec<UiSignal>, world: &mut World) {
    ui.main_menu_bar(|| {
        ui.menu(im_str!("Create Shape"), true, || {
            signal_button!(
                "Rectangle",
                UiSignal::AddShape(ShapeInfo::Rectangle(None)),
                ui,
                signals
            );
            signal_button!(
                "Circle",
                UiSignal::AddShape(ShapeInfo::Circle(None)),
                ui,
                signals
            );
            signal_button!(
                "Polygon",
                UiSignal::AddShape(ShapeInfo::Polygon(Some(Vec::with_capacity(3)))),
                ui,
                signals
            );
        });

        ui.menu(im_str!("Shape Info"), true, || {
            ui.drag_float(im_str!("Mass"), &mut world.fetch_mut::<CreateMass>().0)
                .min(0.001)
                .max(250.0)
                .speed(0.25)
                .build();

            ui.drag_float(im_str!("Elasticity"), &mut world.fetch_mut::<CreateElasticity>().0)
                .min(0.00)
                .max(1.0)
                .speed(0.1)
                .build();

            ui.drag_float(im_str!("Friction"), &mut world.fetch_mut::<CreateFriction>().0)
                .min(0.00)
                .max(1.0)
                .speed(0.1)
                .build();
            
        });
    });
}

pub fn make_default_ui(ui: &mut imgui::Ui) {
    // Window
    imgui::Window::new(im_str!("Hello world"))
        .position([100.0, 100.0], imgui::Condition::Appearing)
        .build(ui, || {
            ui.text(im_str!("Hello world!"));
            ui.separator();

            if ui.small_button(im_str!("small button")) {
                println!("Small button clicked");
            }
        });
}
