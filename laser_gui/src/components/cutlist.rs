use std::time::{Duration, Instant};

use dioxus::{core::Task, prelude::*};
use laser_cutter::gcode_generator::{cut::Cut, workspace::Workspace};
use lucide_dioxus::{
    Minus,
    MoveDown,
    MoveLeft,
    MoveRight,
    MoveUp,
    OctagonMinus,
    Pencil,
    Plus,
    RefreshCcw,
    RefreshCw,
};

#[derive(Props, PartialEq, Clone)]
struct RepeatButtonProps {
    repeat_fn: EventHandler<()>,
    children: Element,
}

fn repeat_button(props: RepeatButtonProps) -> Element {
    let mut running = use_signal(|| false);

    let mut task = use_signal(|| None::<Task>);

    rsx! {
        button {
            class: "flex flex-row justify-center items-center bg-gray-700 hover:bg-gray-800 text-white text-xs font-semibold px-2 py-1 rounded w-full",
            onpointerdown: move |_| {
                running.set(true);

                let t = spawn(async move {
                    props.repeat_fn.call(());
                    let start = Instant::now();
                    while *running.read() {
                        if start.elapsed().as_millis() > 300 {
                            break;
                        }
                        tokio::time::sleep(Duration::from_millis(5)).await;
                    }

                    while *running.read() {
                        props.repeat_fn.call(());
                        let start = Instant::now();
                        while *running.read() {
                            if start.elapsed().as_millis() > 20 {
                                break;
                            }
                            tokio::time::sleep(Duration::from_millis(5)).await;
                        }
                    }
                });

                task.set(Some(t));
            },

            onpointerup: move |_| {
                running.set(false);
                task.write().take();
            },

            onpointerleave: move |_| {
                running.set(false);
                task.write().take();
            },
            {props.children}
        }
    }
}

#[component]
pub fn CutElem(cut: Cut, index: usize, is_last: bool, workspace: Signal<Workspace>) -> Element {
    let mut sc1 = use_signal(|| false);
    let scale_step = 1.1f32;
    let rotate_step = 5.0f32;
    let translate_step = 5.0f32;
    rsx! {
        div { class: "w-full",
            div { class: "flex items-center justify-between bg-gray-800 text-white px-4 py-3",
                div { class: "flex items-baseline gap-2",
                    span { class: "text-gray-300", "{cut}" }
                }
                div { class: "flex gap-2",
                    button {
                        class: "bg-gray-700 hover:bg-gray-800 text-white text-xs font-semibold px-2 py-1 rounded w-full",
                        visibility: if index == 0 { "hidden" } else { "visible" },
                        onclick: move |_| {
                            let mut workspace = workspace.write();
                            workspace.items.swap(index, index - 1);
                        },
                        MoveUp {}
                    }
                    button {
                        class: "bg-gray-700 hover:bg-gray-800 text-white text-xs font-semibold px-2 py-1 rounded w-full",
                        visibility: if is_last { "hidden" } else { "visible" },
                        onclick: move |_| {
                            let mut workspace = workspace.write();
                            workspace.items.swap(index, index + 1);
                        },
                        MoveDown {}
                    }
                    button {
                        class: "bg-gray-700 hover:bg-gray-800 text-white text-xs font-semibold px-2 py-1 rounded w-full",
                        onclick: move |_| {
                            let show = *sc1.read();
                            sc1.set(!show);
                        },
                        Pencil {}
                    }
                }
            }
        }
        if *sc1.read() {
            // Controls section
            div { class: "px-2",
                div { class: "flex -mx-2",
                    div { class: "w-1/3 px-2",
                        repeat_button {
                            repeat_fn: move || {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    cut.transform.scale(1.0 / scale_step);
                                }
                            },

                            Minus {}
                        }
                    }
                    div { class: "w-1/3 px-2",
                        repeat_button {
                            repeat_fn: move |_| {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    cut.transform.translate(0.0, -translate_step);
                                }

                            },
                            MoveUp {}
                        }
                    }
                    div { class: "w-1/3 px-2",
                        repeat_button {
                            repeat_fn: move |_| {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    cut.transform.scale(scale_step);
                                }
                            },
                            Plus {}
                        }
                    }
                }
                div { class: "flex -mx-2",
                    div { class: "w-1/3 px-2",
                        repeat_button {
                            repeat_fn: move |_| {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    cut.transform.translate(-translate_step, 0.0);
                                }
                            },
                            MoveLeft {}
                        }
                    }
                    div { class: "w-1/3 px-2",
                        repeat_button {
                            repeat_fn: move |_| {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    cut.transform.reset();
                                }
                            },
                            OctagonMinus {}
                        }
                    }
                    div { class: "w-1/3 px-2",
                        repeat_button {
                            repeat_fn: move |_| {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    cut.transform.translate(translate_step, 0.0);
                                }
                            },
                            MoveRight {}
                        }
                    }
                }
                div { class: "flex -mx-2",
                    div { class: "w-1/3 px-2",
                        repeat_button {
                            repeat_fn: move |_| {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    cut.transform.rotate(-rotate_step);
                                }
                            },
                            RefreshCw {}
                        }
                    }
                    div { class: "w-1/3 px-2",
                        repeat_button {
                            repeat_fn: move |_| {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    cut.transform.translate(0.0, translate_step);
                                }
                            },
                            MoveDown {}
                        }
                    }
                    div { class: "w-1/3 px-2",
                        repeat_button {
                            repeat_fn: move |_| {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    cut.transform.rotate(rotate_step);
                                }
                            },
                            RefreshCcw {}
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn CutList(workspace: Signal<Workspace>) -> Element {
    let workspace_read = workspace.read();

    rsx! {
        for (index , cut) in workspace_read.items().iter().enumerate() {
            CutElem {
                cut: cut.clone(),
                index,
                is_last: index == workspace_read.items().len() - 1,
                workspace,
            }
        }
    }
}
