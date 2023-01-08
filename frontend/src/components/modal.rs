use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct ModalProps {
    pub title: String,
    pub is_show: UseStateHandle<bool>,
    pub children: Children,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let close = {
        let is_show = props.is_show.clone();

        move |_| {
            is_show.set(false);
        }
    };

    html!(
        <>
            <div class="modal-backdrop fade in show"></div>

            <div class="modal fade d-block show">
                <div class="modal-dialog modal-dialog-centered">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h5 class="modal-title">{ props.title.clone() }</h5>

                            <button type="button" class="btn-close" onclick={close.clone()}></button>
                        </div>
                        <div class="modal-body">
                            { for props.children.iter() }
                        </div>
                    </div>
                </div>
            </div>
        </>
    )
}
