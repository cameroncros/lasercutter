use crate::components::connection_controls::ConnectionControls;
use crate::components::machine_controls::MachineControls;
use crate::style::RIGHT_BAR_CLASSES;
use dioxus::prelude::*;

#[component]
pub fn RightBar() -> Element {
    rsx! {
        div { class: RIGHT_BAR_CLASSES,
            ConnectionControls {}
            MachineControls {}
        }
    }
}
