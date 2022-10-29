use baseview::{Size, WindowHandle, WindowOpenOptions, WindowScalePolicy};
use egui::Context;
use std::sync::Arc;
use vst::{editor::Editor, plugin::PluginParameters};

use log::*;

use egui_baseview::{EguiWindow, Queue};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use crate::plugin_state::PluginState;

pub struct WindowParent(pub WindowHandle);
unsafe impl Send for WindowParent {}

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

struct VstPadre {
    parent: *mut ::std::ffi::c_void,
}
unsafe impl Send for VstPadre {}

#[cfg(target_os = "linux")]
unsafe impl HasRawWindowHandle for VstPadre {
    fn raw_window_handle(&self) -> RawWindowHandle {
        use raw_window_handle::XcbHandle;

        let mut window_handle = XcbHandle::empty();
        window_handle.window = self.parent as u32;

        RawWindowHandle::Xcb(window_handle)
    }
}

const WINDOW_WIDTH: usize = 500;
const WINDOW_HEIGHT: usize = 600;

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
        info!("Opening Editor");
        if self.is_open {
            return false;
        }
        self.is_open = true;

        info!("Creating Settings");
        let settings = WindowOpenOptions {
            title: String::from("SynthOne"),
            size: Size::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64),
            scale: WindowScalePolicy::SystemScaleFactor,
            gl_config: Some(Default::default()),
        };

        info!("Creating Window Handle");
        let vst_parent = VstPadre { parent };
        let window_handle = EguiWindow::open_parented(
            &vst_parent,
            settings,
            self.params.clone(),
            |_egui_ctx, _queue, _state| {},
            |egui_ctx: &Context, _, state: &mut Arc<PluginState>| {
                draw_ui(egui_ctx, state);
            },
        );
        info!("Window handle II");

        self.window_handle = Some(WindowParent(window_handle));
        info!("Done with open");

        true
    }
}

#[inline(always)]
fn draw_ui(ctx: &Context, params: &mut Arc<PluginState>) -> egui::Response {
    info!("Draw UI");
    egui::CentralPanel::default()
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label("Editor");

                for i in 0..12 {
                    let mut val = params.get_parameter(i);
                    let parameter_name = params.get_parameter_label(i);
                    let slider =
                        ui.add(egui::Slider::new(&mut val, 0.0..=1.0).text(parameter_name));

                    if slider.changed() {
                        params.set_parameter(i, val);
                    }
                }
            })
        })
        .response
}
