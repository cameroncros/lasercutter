use std::time::{Duration, Instant};

use dioxus::{
    core::{Element, EventHandler, Task},
    core_macro::{rsx, Props},
    hooks::use_signal,
    prelude::*,
};

use crate::style::BUTTON_CLASSES;

#[derive(Props, PartialEq, Clone)]
pub(crate) struct RepeatButtonProps {
    repeat_fn: EventHandler<()>,
    children: Element,
}

#[component]
pub(crate) fn RepeatButton(props: RepeatButtonProps) -> Element {
    let mut running = use_signal(|| false);

    let mut task = use_signal(|| None::<Task>);

    rsx! {
        button {
            class: BUTTON_CLASSES,
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
