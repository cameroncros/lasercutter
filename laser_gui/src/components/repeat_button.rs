use std::sync::atomic::{AtomicBool, AtomicU8};
use std::sync::Arc;
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
    let mut task = use_signal(|| None::<Task>);

    rsx! {
        button {
            class: BUTTON_CLASSES,
            onpointerdown: move |_| {
                if let Some(t) = task.write().take() {
                    t.cancel();
                }
                let t = spawn(async move {
                    props.repeat_fn.call(());
                    tokio::time::sleep(Duration::from_millis(300)).await;
                    loop {
                        props.repeat_fn.call(());
                        tokio::time::sleep(Duration::from_millis(100)).await;
                    }
                });
                task.set(Some(t));
            },

            onpointerup: move |_| {
                if let Some(t) = task.write().take() {
                    t.cancel();
                }
            },

            onpointerleave: move |_| {
                if let Some(t) = task.write().take() {
                    t.cancel();
                }
            },
            {props.children}
        }
    }
}
