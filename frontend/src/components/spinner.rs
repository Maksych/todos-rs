use yew::prelude::*;

#[function_component(SpinnerLarge)]
pub fn spinner_large() -> Html {
    html!(
        <div class="absolute inset-0 m-auto w-full h-full text-center bg-base-100 z-40">
            <div class="absolute w-min h-min inset-0 m-auto">
                <div class="w-20 h-20 rounded-full animate-spin border-8 border-solid border-primary border-t-transparent shadow-md"></div>
                <span class="sr-only">{ "Loading..." }</span>
            </div>
        </div>
    )
}

#[function_component(SpinnerMedium)]
pub fn spinner_medium() -> Html {
    html!(
        <div class="absolute inset-0 m-auto w-full h-full text-center bg-base-100 z-40">
            <div class="absolute w-min h-min inset-0 m-auto">
                <div class="w-16 h-16 rounded-full animate-spin border-4 border-solid border-primary border-t-transparent shadow-md"></div>
                <span class="sr-only">{ "Loading..." }</span>
            </div>
        </div>
    )
}
