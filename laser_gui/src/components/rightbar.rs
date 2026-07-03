use crate::components::connection_controls::ConnectionControls;
use crate::components::machine_controls::MachineControls;
use dioxus::prelude::*;

#[component]
pub fn RightBar() -> Element {
    rsx! {
        div { class: "w-64 bg-gray-600 p-3 text-white",
            ConnectionControls {}
            MachineControls {}
        }
    }
}
