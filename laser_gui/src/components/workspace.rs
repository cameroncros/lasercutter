use dioxus::html::geometry::WheelDelta;
use dioxus::prelude::*;
use laser_cutter::gcode_emulator::GCodeEmulator;
use laser_cutter::gcode_generator::workspace::Workspace;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use crate::style::*;

fn render(workspace: &Workspace) -> anyhow::Result<(String, Duration)> {
    let start = std::time::Instant::now();

    let t0 = std::time::Instant::now();
    let gcode = workspace.gen_gcode()?;
    let gcode_time = t0.elapsed();

    let t1 = std::time::Instant::now();
    let mut emu = GCodeEmulator::from_gcode(gcode)?;
    emu.run()?;
    let emu_time = t1.elapsed();

    let t2 = std::time::Instant::now();
    let img_url = emu.to_img_url()?;
    let img_time = t2.elapsed();

    // eprintln!(
    //     "Timing: gcode={:?}, emu={:?}, img={:?}, total={:?}",
    //     gcode_time,
    //     emu_time,
    //     img_time,
    //     start.elapsed()
    // );

    Ok((img_url, start.elapsed()))
}

#[component]
pub fn WorkspaceView(
    workspace: Signal<Workspace>,
    rendertime: Signal<String>,
    errormsg: Signal<String>,
) -> Element {
    let mut zoom = use_signal(|| 1.0f32);

    let preview = use_resource(move || async move {
        let str = match render(&workspace.read()) {
            Ok((svg, duration)) => {
                rendertime.set(format!("{duration:?}"));
                svg
            }
            Err(e) => {
                errormsg.set(e.to_string());
                rendertime.set("Failed".into());
                "".into()
            }
        };
        str
    });

    rsx! {
        div { class: WORKSPACE_VIEW_CLASSES,
            div { class: PREVIEW_HEADER_CLASSES,
                span { class: PREVIEW_TEXT_CLASSES, "Preview" }
                div { class: PREVIEW_ZOOM_CONTAINER_CLASSES,
                    button {
                        class: SMALL_BUTTON_CLASSES,
                        onclick: move |_| {
                            zoom.set((zoom() - 0.1).max(0.2));
                        },
                        "-"
                    }
                    span { class: PREVIEW_ZOOM_VALUE_CLASSES,
                        "{(zoom() * 100.0).round()}%"
                    }
                    button {
                        class: SMALL_BUTTON_CLASSES,
                        onclick: move |_| {
                            zoom.set((zoom() + 0.1).min(5.0));
                        },
                        "+"
                    }
                    button {
                        class: SMALL_BUTTON_CLASSES,
                        onclick: move |_| {
                            zoom.set(1.0);
                        },
                        "Reset"
                    }
                }
            }
            div {
                class: PREVIEW_VIEWPORT_CLASSES,
                onwheel: move |e| {
                    let delta = e.delta();
                    match delta {
                        WheelDelta::Pixels(delta) => {
                            if delta.y < 0.0 {
                                zoom.set((zoom() + 0.1).min(5.0));
                            } else if delta.y > 0.0 {
                                zoom.set((zoom() - 0.1).max(0.2));
                            }
                        }
                        WheelDelta::Lines(delta) => {
                            if delta.y < 0.0 {
                                zoom.set((zoom() + 0.1).min(5.0));
                            } else if delta.y > 0.0 {
                                zoom.set((zoom() - 0.1).max(0.2));
                            }
                        }
                        WheelDelta::Pages(delta) => {
                            if delta.y < 0.0 {
                                zoom.set((zoom() + 0.1).min(5.0));
                            } else if delta.y > 0.0 {
                                zoom.set((zoom() - 0.1).max(0.2));
                            }
                        }
                    }
                },
                div { class: "min-h-full min-w-full",
                    match &*preview.read() {
                        Some(data_url) => rsx! {
                            img {
                                class: PREVIEW_IMAGE_CLASSES,
                                style: "transform: scale({zoom}); transform-origin: 0 0;",
                                src: "{data_url}",
                            }
                        },
                        None => rsx! {
                            p { "Generating..." }
                        },
                    }

                }
            }
        }
    }
}
