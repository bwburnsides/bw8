mod bus;
mod uart;
mod vga;

use arch::{self, Architectural16, Architectural8, ReachedBreakpoint};
use spin_sleep_util::{Interval, RateReporter};
use std::time::Duration;

const WIDTH: usize = 1280;
const HEIGHT: usize = 960;

pub struct Bw8 {
    cpu: arch::CpuState,
    bus: bus::Bw8Bus,
    vga: vga::Vga,
}

impl Bw8 {
    pub fn new(binary_path: String) -> Self {
        Self {
            cpu: arch::CpuState::new(),
            bus: bus::Bw8Bus::new(binary_path),
            vga: vga::Vga::new(),
        }
    }

    pub fn run(&mut self, cycles: usize) -> (arch::trace::Trace, ReachedBreakpoint) {
        let rv = self.cpu.run(&mut self.bus, cycles);
        self.vga.clock(&self.bus);
        rv
    }

    pub fn reset(&mut self) {
        self.bus.set_reset(true);
        self.cpu.run(&mut self.bus, 1);
        self.bus.set_reset(false);
        self.bus.reset();
        self.vga.reset();
    }

    pub fn inject_irq(&mut self) {
        self.bus.set_irq(true);
    }

    pub fn inject_nmi(&mut self) {
        self.bus.set_nmi(true);
    }

    pub fn vga_frame(&self) -> &[u8] {
        &self.vga.pixel_data()
    }
}

pub struct EmulatorState {
    running: bool,
    fps: f64,
    loop_interval: Interval,
    loop_reporter: RateReporter,
    vga_texture: egui::TextureHandle,
}

impl EmulatorState {
    pub fn new(ui_context: &egui::Context) -> Self {
        let loop_interval =
            spin_sleep_util::interval(Duration::from_secs_f64(1.0 / vga::FRAMERATE));
        let loop_reporter = RateReporter::new(Duration::from_secs_f64(0.5));

        let vga_image = egui::ColorImage::new([WIDTH, HEIGHT], egui::Color32::WHITE);
        let vga_texture =
            ui_context.load_texture("VGA Framebuffer", vga_image, egui::TextureOptions::NEAREST);

        Self {
            running: false,
            fps: 0.0,
            loop_interval,
            loop_reporter,
            vga_texture,
        }
    }

    pub fn update(&mut self, system: &mut Bw8) {
        self.loop_interval.tick();

        // Need to write microcode so that either:
        //  1. Know cycle counts and can add wait-cycles between frames
        //  2. Implement cycle-accurate microcode emulation in `arch`.
        // Assuming 4 MHz clock rate and 4 cycles per instruction, we can
        // expect 16,667 instructions to be executed in 1/60th of a second.
        if self.running {
            let (_trace, bp) = system.run(16_650);
            self.running = bp == ReachedBreakpoint::DidNot;
        };

        if let Some(fps) = self.loop_reporter.increment_and_report() {
            self.fps = fps;
        }

        let vga_image = egui::ColorImage::from_rgba_unmultiplied(
            [vga::COLUMN_COUNT, vga::ROW_COUNT],
            system.vga_frame(),
        );
        self.vga_texture
            .set(vga_image, egui::TextureOptions::NEAREST);
    }

    pub fn draw(&mut self, system: &mut Bw8, ui: &mut egui::Ui) {
        use egui::panel::*;
        use egui::style::*;
        use egui::*;

        SidePanel::new(Side::Right, "ctrl")
            .show_separator_line(false)
            .resizable(false)
            .show_inside(ui, |ui| {
                ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);

                TopBottomPanel::new(TopBottomSide::Top, "Emulator Control").show_inside(ui, |ui| {
                    if self.running {
                        ui.label(format!("{:.2} fps", self.fps,));
                    } else {
                        ui.label("- fps");
                    }

                    ui.with_layout(
                        Layout {
                            main_dir: Direction::LeftToRight,
                            ..*ui.layout()
                        },
                        |ui| {
                            if ui
                                .button(if self.running { "Pause" } else { "Run" })
                                .clicked()
                            {
                                self.running = !self.running;
                            }

                            if ui
                                .add_enabled(!self.running, Button::new("Single Step"))
                                .clicked()
                            {
                                system.run(1);
                            }

                            if ui.button("Reset").clicked() {
                                self.running = false;
                                system.reset();
                            }

                            if ui.add_enabled(self.running, Button::new("IRQ")).clicked() {
                                system.inject_irq();
                            }

                            if ui.add_enabled(self.running, Button::new("NMI")).clicked() {
                                system.inject_nmi();
                            }
                        },
                    );

                    ui.with_layout(
                        Layout {
                            main_dir: Direction::LeftToRight,
                            ..*ui.layout()
                        },
                        |_ui| {
                            // if ui
                            //     .add_enabled(
                            //         self.running && (system.clock_rate() > 1_000.0),
                            //         Button::new("-- Clock Speed"),
                            //     )
                            //     .clicked()
                            // {
                            //     system.set_clock_rate(system.clock_rate() * 0.5);
                            // }

                            // if ui
                            //     .add_enabled(
                            //         self.running && (system.clock_rate() < 16_000_000.0),
                            //         Button::new("++ Clock Speed"),
                            //     )
                            //     .clicked()
                            // {
                            //     system.set_clock_rate(system.clock_rate() * 2.0);
                            // }
                        },
                    );

                    ui.add_space(10.0);
                });

                TopBottomPanel::new(TopBottomSide::Top, "Register View").show_inside(ui, |ui| {
                    SidePanel::new(Side::Left, "regs16")
                        .show_separator_line(false)
                        .resizable(false)
                        .frame(Frame {
                            inner_margin: Margin::symmetric(35.0, 10.0),
                            ..Default::default()
                        })
                        .show_inside(ui, |ui| {
                            ui.with_layout(ui.layout().with_cross_align(Align::LEFT), |ui| {
                                ui.label(RichText::new("16 Bit Regs").color(Color32::WHITE));

                                ui.label(format!("PC: {:0>4X}", system.cpu[Architectural16::PC]));
                                ui.label(format!("SP: {:0>4X}", system.cpu[Architectural16::SP]));
                                ui.label(format!("X:  {:0>4X}", system.cpu[Architectural16::X]));
                                ui.label(format!("Y:  {:0>4X}", system.cpu[Architectural16::Y]));
                            });
                        });

                    SidePanel::new(Side::Left, "regs8")
                        .show_separator_line(false)
                        .resizable(false)
                        .frame(Frame {
                            inner_margin: Margin::symmetric(35.0, 10.0),
                            ..Default::default()
                        })
                        .show_inside(ui, |ui| {
                            ui.with_layout(ui.layout().with_cross_align(Align::LEFT), |ui| {
                                ui.label(RichText::new("8 Bit Regs").color(Color32::WHITE));

                                ui.label(format!("A: {:0>2X}", system.cpu[Architectural8::A]));
                                ui.label(format!("B: {:0>2X}", system.cpu[Architectural8::B]));
                                ui.label(format!("C: {:0>2X}", system.cpu[Architectural8::C]));
                                ui.label(format!("D: {:0>2X}", system.cpu[Architectural8::D]));
                            });
                        });

                    SidePanel::new(Side::Right, "flags")
                        .show_separator_line(false)
                        .resizable(false)
                        .frame(Frame {
                            inner_margin: Margin::symmetric(35.0, 10.0),
                            ..Default::default()
                        })
                        .show_inside(ui, |ui| {
                            ui.with_layout(ui.layout().with_cross_align(Align::LEFT), |ui| {
                                ui.label(RichText::new("Flags").color(Color32::WHITE));

                                let status = system.cpu.status();

                                let carry: u8 = status.carry.into();
                                let zero: u8 = status.zero.into();
                                let negative: u8 = status.negative.into();
                                let overflow: u8 = status.overflow.into();
                                let irq_enable: u8 = status.irq_enable.into();
                                let bank_enable: u8 = status.bank_enable.into();
                                let nmi_active: u8 = status.nmi_active.into();

                                ui.label("C  Z  S  O  I  B  A");
                                ui.label(format!(
                                    "{}  {}  {}  {}  {}  {}  {}",
                                    carry,
                                    zero,
                                    negative,
                                    overflow,
                                    irq_enable,
                                    bank_enable,
                                    nmi_active,
                                ));

                                ui.label(format!("Privilege: {:?}", status.privilege_level,));
                                ui.label(format!(
                                    "Bank Register: {:0>1X}",
                                    system.cpu.br().as_inner()
                                ));
                            });
                        });
                });

                CentralPanel::default().show_inside(ui, |ui| {
                    ui.with_layout(ui.layout().with_cross_align(Align::Center), |ui| {
                        ui.label(RichText::new("Memory").color(Color32::WHITE))
                    });

                    ui.label("ADDR | 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F");
                    ui.separator();

                    ScrollArea::new([false, true]).show(ui, |ui| {
                        for addr in (u16::MIN..=0x4000).step_by(16) {
                            use std::fmt::Write;

                            // Length of one line is 6 characters for `ADDR |` + 3 characters for each byte.
                            let mut line = String::with_capacity(6 + 16 * 3);
                            write!(line, "{:0>4X} |", addr).unwrap();
                            for i in 0..16 {
                                write!(line, " {:0>2X}", system.bus.inspect_memory(addr + i),)
                                    .unwrap();
                            }

                            ui.label(line);
                        }
                    });
                });
            });

        CentralPanel::default()
            .frame(Frame::side_top_panel(&Style {
                visuals: Visuals {
                    panel_fill: Color32::BLACK,
                    ..Visuals::dark()
                },
                ..Default::default()
            }))
            .show_inside(ui, |ui| {
                const SCREEN_SIZE: Vec2 = Vec2::new(WIDTH as f32, HEIGHT as f32);

                let xf = ui.available_width() / SCREEN_SIZE.x;
                let yf = ui.available_height() / SCREEN_SIZE.y;
                let f = f32::min(xf, yf);
                let size = SCREEN_SIZE * f;

                ui.centered_and_justified(|ui| {
                    ui.image((self.vga_texture.id(), size));
                })
            });
    }

    #[inline]
    pub fn quit(&mut self, _system: &mut Bw8) {
        // system.terminal().quit().unwrap();
    }
}
