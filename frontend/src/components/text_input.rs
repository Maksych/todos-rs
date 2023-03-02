use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct TextInputProps {
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub required: bool,
    pub value: String,
    pub placeholder: String,
    pub errors: Vec<String>,
    pub onchange: Callback<Event, ()>,
}

#[function_component(TextInput)]
pub fn text_input(props: &TextInputProps) -> Html {
    html!(
        <>
            <label for={ props.name.clone() } class="sr-only">{ props.placeholder.clone() }</label>

            <input
                id={ props.id.clone() }
                name={ props.name.clone() }
                type={ props.r#type.clone() }
                required={ props.required }
                placeholder={ props.placeholder.clone() }
                class={classes!(
                    "input",
                    "input-bordered",
                    if !props.errors.is_empty() {"input-error"} else {""},
                    "w-full"
                )}
                onchange={ props.onchange.clone() }
                value={ props.value.clone() }
            />

            if !props.errors.is_empty() {
                <label class="label">
                    {
                        props.errors
                            .iter()
                            .map(|text| html!(<span class="label-text-alt text-error">{ text.clone() }</span>))
                            .collect::<Html>()
                    }
                </label>
            }
        </>
    )
}
