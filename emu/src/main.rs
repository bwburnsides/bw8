mod emu;

use egui_wgpu::winit::Painter;
use std::sync::Arc;
use winit::window::Window;

use emu::{Bw8, EmulatorState};

struct AppState {
    window: Arc<Window>,
    ui_context: egui::Context,
    ui_state: egui_winit::State,
    ui_painter: Painter,
    system: Bw8,
    emu_state: EmulatorState,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use egui_wgpu::WgpuConfiguration;
    use std::num::NonZeroU32;
    use winit::dpi::PhysicalSize;
    use winit::event::{Event, WindowEvent};
    use winit::event_loop::{ControlFlow, EventLoop};
    use winit::window::WindowBuilder;

    let event_loop = EventLoop::new()?;
    let mut app_state = None;

    event_loop.run(move |event, window_target| {
        window_target.set_control_flow(ControlFlow::Poll);

        match event {
            Event::Resumed => {
                let window = Arc::new(
                    WindowBuilder::new()
                        .with_title("bw8 Emulator")
                        .with_inner_size(PhysicalSize::new(2800, 1400))
                        .build(window_target)
                        .unwrap(),
                );

                let ui_context = egui::Context::default();
                let ui_state = egui_winit::State::new(
                    ui_context.clone(),
                    ui_context.viewport_id(),
                    window_target,
                    None,
                    None,
                );
                let gpu_config = WgpuConfiguration::default();
                let mut ui_painter = Painter::new(gpu_config, 1, None, false);

                const FONT: &[u8] = include_bytes!("../res/SourceCodePro-Regular.ttf");
                const FONT_NAME: &str = "SourceCodePro";
                let mut fonts = egui::FontDefinitions::empty();
                fonts
                    .font_data
                    .insert(FONT_NAME.to_owned(), egui::FontData::from_static(FONT));
                fonts
                    .families
                    .entry(egui::FontFamily::Proportional)
                    .or_default()
                    .insert(0, FONT_NAME.to_owned());
                ui_context.set_fonts(fonts);
                ui_context.set_style(egui::Style {
                    override_font_id: Some(egui::FontId::proportional(14.0)),
                    ..Default::default()
                });

                let binary_path = std::env::args().nth(1).unwrap();
                let mut system = Bw8::new(binary_path);
                system.reset();

                // if let Some(program) = args.run.as_deref() {
                //     system
                //         .load_program(0, &std::fs::read(program).unwrap())
                //         .expect("binary is too big");
                //     system.execute_program();
                // }

                pollster::block_on(
                    ui_painter.set_window(ui_context.viewport_id(), Some(Arc::clone(&window))),
                )
                .unwrap();

                let emu_state = EmulatorState::new(&ui_context);

                app_state = Some(AppState {
                    window,
                    ui_context,
                    ui_state,
                    ui_painter,
                    system,
                    emu_state,
                });
            }
            Event::WindowEvent { window_id, event } => {
                if let Some(app_state) = app_state.as_mut() {
                    if window_id == app_state.window.id() {
                        if !app_state
                            .ui_state
                            .on_window_event(&app_state.window, &event)
                            .consumed
                        {
                            match event {
                                WindowEvent::CloseRequested => {
                                    window_target.exit();
                                    app_state.emu_state.quit(&mut app_state.system);
                                }
                                WindowEvent::Resized(size) => {
                                    let width =
                                        NonZeroU32::new(size.width).unwrap_or(NonZeroU32::MIN);
                                    let height =
                                        NonZeroU32::new(size.height).unwrap_or(NonZeroU32::MIN);
                                    app_state.ui_painter.on_window_resized(
                                        app_state.ui_context.viewport_id(),
                                        width,
                                        height,
                                    )
                                }
                                WindowEvent::RedrawRequested => {
                                    app_state.emu_state.update(&mut app_state.system);
                                    let ui_input =
                                        app_state.ui_state.take_egui_input(&app_state.window);
                                    let ui_output = app_state.ui_context.run(ui_input, |ctx| {
                                        egui::CentralPanel::default().show(ctx, |ui| {
                                            app_state.emu_state.draw(&mut app_state.system, ui);
                                        });
                                    });

                                    app_state.ui_state.handle_platform_output(
                                        &app_state.window,
                                        ui_output.platform_output,
                                    );

                                    let ui_primitives = app_state.ui_context.tessellate(
                                        ui_output.shapes,
                                        app_state.ui_context.pixels_per_point(),
                                    );
                                    app_state.ui_painter.paint_and_update_textures(
                                        app_state.ui_context.viewport_id(),
                                        app_state.ui_context.pixels_per_point(),
                                        egui::Rgba::BLACK.to_array(),
                                        &ui_primitives,
                                        &ui_output.textures_delta,
                                        false,
                                    );

                                    app_state.window.request_redraw();
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    })?;

    Ok(())
}
