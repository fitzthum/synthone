use baseview::{Size, WindowHandle, WindowOpenOptions, WindowScalePolicy};
use egui::{Context, Ui};
use egui::plot::{Line, Plot, PlotPoints};
use std::sync::Arc;
use vst::{editor::Editor, plugin::PluginParameters};

use egui_baseview::EguiWindow;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use crate::plugin_state::PluginState;
use crate::dsp::envelope::{Envelope, ADSR};
use crate::dsp::oscillator::{Oscillator, WaveTableOscillator};

pub struct WindowParent(pub WindowHandle);
unsafe impl Send for WindowParent {}

struct VstParent {
    parent: *mut ::std::ffi::c_void,
}

unsafe impl Send for VstParent {}

pub struct PluginEditor {
    pub params: Arc<PluginState>,
    pub window_handle: Option<WindowParent>,
    pub is_open: bool,
}

#[cfg(target_os = "macos")]
unsafe impl HasRawWindowHandle for VstParent {
    fn raw_window_handle(&self) -> RawWindowHandle {
        use raw_window_handle::macos::MacOSHandle;

        RawWindowHandle::MacOS(MacOSHandle {
            ns_view: self.0 as *mut ::std::ffi::c_void,
            ..MacOSHandle::empty()
        })
    }
}

#[cfg(target_os = "windows")]
unsafe impl HasRawWindowHandle for VstParent {
    fn raw_window_handle(&self) -> RawWindowHandle {
        use raw_window_handle::windows::WindowsHandle;

        RawWindowHandle::Windows(WindowsHandle {
            hwnd: self.0,
            ..WindowsHandle::empty()
        })
    }
}

#[cfg(target_os = "linux")]
unsafe impl HasRawWindowHandle for VstParent {
    fn raw_window_handle(&self) -> RawWindowHandle {
        use raw_window_handle::XcbHandle;

        let mut window_handle = XcbHandle::empty();
        window_handle.window = self.parent as u32;

        RawWindowHandle::Xcb(window_handle)
    }
}

const WINDOW_WIDTH: usize = 900;
const WINDOW_HEIGHT: usize = 1200;

impl Editor for PluginEditor {
    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn size(&self) -> (i32, i32) {
        (WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
    }

    fn is_open(&mut self) -> bool {
        self.is_open
    }

    fn close(&mut self) {
        self.is_open = false;
        if let Some(mut window_handle) = self.window_handle.take() {
            (window_handle.0).close();
        }
    }

    fn open(&mut self, parent: *mut ::std::ffi::c_void) -> bool {
        if self.is_open {
            return false;
        }
        self.is_open = true;

        let settings = WindowOpenOptions {
            title: String::from("SynthOne"),
            size: Size::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64),
            scale: WindowScalePolicy::SystemScaleFactor,
            gl_config: Some(Default::default()),
        };

        let vst_parent = VstParent { parent };
        let window_handle = EguiWindow::open_parented(
            &vst_parent,
            settings,
            self.params.clone(),
            |_egui_ctx, _queue, _state| {},
            |egui_ctx: &Context, _, state: &mut Arc<PluginState>| {
                draw_ui(egui_ctx, state);
            },
        );
        self.window_handle = Some(WindowParent(window_handle));

        true
    }
}

#[inline(always)]
fn draw_ui(ctx: &Context, params: &mut Arc<PluginState>) -> egui::Response {
    egui::CentralPanel::default()
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label("Editor");

                // Sliders for volume and envelope
                for i in 0..5 {
                    draw_slider(ui, &params, i);
                }

                // Draw envelope 
                let a = params.attack.get();
                let d = params.decay.get();
                let s = params.sustain.get();
                let r = params.release.get();

                draw_envelope(ui, a, d, s, r, "env1");
                
                // Sliders for filter and wave warp
                for i in 5..11 {
                    draw_slider(ui, &params, i);
                }

                let a = params.warp_attack.get();
                let d = params.warp_decay.get();
                let s = params.warp_sustain.get();
                let r = params.warp_release.get();

                // Wave warp envelope
                draw_envelope(ui, a, d, s, r, "env2");

                // Warp ratio
                draw_slider(ui, &params, 11);

                // Oscillator
                draw_oscillator(ui, params.wave_warp.get(), params.sample_rate.get());

            })
        })
    .response
}

fn draw_slider(ui: &mut Ui, params: &PluginState, i: i32) {
    let mut val = params.get_parameter(i);
    let parameter_name = params.get_parameter_label(i);
    let slider =
        ui.add(
            egui::Slider::new(&mut val, 0.0..=1.0)
                .text(parameter_name)
                .show_value(false)
        );

    if slider.changed() {
        params.set_parameter(i, val);
    }
}

fn draw_envelope(ui: &mut Ui, a: f32, d: f32, s: f32, r: f32, id: &str) {
    const STEP_X: f32 = 0.01;
    const OFF_INDEX: i32 = 200;
    const TOTAL_STEPS: i32 = 300;
    const HEIGHT: f32 = 30.0;
    const WIDTH: f32 = 90.0;

    let mut envelope = ADSR::new(a, d, s, r);
    let points: PlotPoints = (0..TOTAL_STEPS).map(|i| {
        let x = i as f32 * STEP_X;
        let on = i <= OFF_INDEX;
        let y = envelope.process(x, on, OFF_INDEX as f32 * STEP_X);

        [x as f64, y as f64]
    }).collect();

    let line = Line::new(points);
    draw_plot(ui, line, (HEIGHT, WIDTH), (-0.1, 1.1), (-0.1, 3.1), id);

}

fn draw_oscillator(ui: &mut Ui, wave_warp: f32, sample_rate: f32) {
    const STEP_X: f32 = 0.01;
    const TOTAL_STEPS: i32 = 300;
    const HEIGHT: f32 = 60.0;
    const WIDTH: f32 = 180.0;
    const ID: &str = "Oscillator";

    let oscillator = WaveTableOscillator::new(1.0, sample_rate, wave_warp);
    let points: PlotPoints = (0..TOTAL_STEPS).map(|i| {
        let x = i as f32 * STEP_X;
        let y = oscillator.process(x);

        [x as f64, y as f64]

    }).collect();

    let line = Line::new(points);
    draw_plot(ui, line, (HEIGHT, WIDTH), (-0.1, 1.1), (-0.1, 3.1), ID);

}

fn draw_plot(
    ui: &mut Ui,
    line: Line,
    dimensions: (f32, f32),
    bounds_x: (f32, f32),
    bounds_y: (f32, f32),
    id: &str) {

    let plot = Plot::new(id)
        .height(dimensions.0)
        .width(dimensions.1)
        .allow_scroll(false)
        .allow_zoom(false)
        .allow_boxed_zoom(false)
        .allow_drag(false)
        .show_axes([false, false])
        .include_y(bounds_x.0)
        .include_y(bounds_x.1)
        .include_x(bounds_y.0)
        .include_x(bounds_y.1)
        .label_formatter(|_name, _value| { "".to_owned() });

    plot.show(ui, |plot_ui| plot_ui.line(line));

}
