use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ModalProps {
    pub toggle: UseToggleHandle<bool>,
    pub children: Children,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let toggle = {
        let toggle = props.toggle.clone();

        move |_| {
            toggle.toggle();
        }
    };

    html!(
        <div class="modal modal-open ">
            <div class="modal-box relative">
                <button onclick={ toggle } class="btn btn-sm btn-circle absolute right-2 top-2">{ "✕" }</button>
                { for props.children.iter() }
            </div>
        </div>
    )
}
