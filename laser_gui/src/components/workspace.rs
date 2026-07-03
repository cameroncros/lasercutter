use dioxus::html::geometry::WheelDelta;
use dioxus::prelude::*;
use laser_cutter::gcode_emulator::GCodeEmulator;
use laser_cutter::gcode_generator::workspace::Workspace;
use std::time::Duration;

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
    refresh: Signal<i32>,
) -> Element {
    let mut zoom = use_signal(|| 1.0f32);

    let preview = use_resource(move || {
        let _ = refresh();
        async move {
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
        }
    });

    rsx! {
        div { class: "flex-1 bg-gray-300 flex flex-col overflow-hidden",
            div { class: "flex items-center justify-between px-3 py-2 bg-gray-200 border-b border-gray-400",
                span { class: "text-sm text-gray-700", "Preview" }
                div { class: "flex items-center gap-2",
                    button {
                        class: "bg-gray-700 hover:bg-gray-800 text-white text-xs font-semibold px-2 py-1 rounded",
                        onclick: move |_| zoom.set((zoom() - 0.1).max(0.2)),
                        "-"
                    }
                    span { class: "text-xs text-gray-700 w-12 text-center",
                        "{(zoom() * 100.0).round()}%"
                    }
                    button {
                        class: "bg-gray-700 hover:bg-gray-800 text-white text-xs font-semibold px-2 py-1 rounded",
                        onclick: move |_| zoom.set((zoom() + 0.1).min(5.0)),
                        "+"
                    }
                    button {
                        class: "bg-gray-700 hover:bg-gray-800 text-white text-xs font-semibold px-2 py-1 rounded",
                        onclick: move |_| zoom.set(1.0),
                        "Reset"
                    }
                }
            }
            div {
                class: "flex-1 overflow-auto",
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
                                class: "max-w-none max-h-none",
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
