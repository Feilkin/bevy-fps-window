//! Debug UI stuff
use std::collections::VecDeque;

use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::egui::plot::{HLine, Line, Plot, PlotPoints};
use bevy_egui::egui::Id;
use bevy_egui::{egui, EguiContexts};

pub struct FpsWindowPlugin;

impl Plugin for FpsWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(show_fps_window);
    }
}

fn show_fps_window(
    diagnostics: Res<Diagnostics>,
    mut egui_contexts: EguiContexts,
    mut averages: Local<VecDeque<f64>>,
    mut frame_times: Local<VecDeque<f64>>,
) {
    let egui_context = egui_contexts.ctx_mut();
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        egui::Window::new(format!("FPS: {:.2}", fps.average().unwrap_or(0.)))
            .id(Id::new("bevy-fps-window"))
            .default_height(400.)
            .default_pos([0., 0.])
            .show(egui_context, |ui| {
                if let Some(value) = fps.value() {
                    averages.push_front(value);
                    averages.truncate(2000);

                    let points: PlotPoints = averages
                        .iter()
                        .rev()
                        .enumerate()
                        .map(|(i, v)| [i as f64, *v])
                        .collect();
                    let line = Line::new(points);

                    Plot::new("averages")
                        .height(200.)
                        .show(ui, |plot_ui| plot_ui.line(line));
                }

                if let Some(frame_time) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME) {
                    if let Some(value) = frame_time.value() {
                        // value is in seconds, make it more reasonable ms
                        frame_times.push_front(value * 1000.);
                        frame_times.truncate(2000);

                        let points: PlotPoints = frame_times
                            .iter()
                            .rev()
                            .enumerate()
                            .map(|(i, v)| [i as f64, *v])
                            .collect();
                        let line = Line::new(points);

                        let average = frame_times.iter().sum::<f64>() / frame_times.len() as f64;
                        let average_line = HLine::new(average);

                        Plot::new("frame times").height(200.).show(ui, |plot_ui| {
                            plot_ui.line(line);
                            plot_ui.hline(average_line)
                        });
                    }
                }
            });
    }
}
