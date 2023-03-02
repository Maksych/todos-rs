use yew::prelude::*;

use crate::components::navbar::Navbar;

#[function_component(Header)]
pub fn header() -> Html {
    html!(
        <header class="max-w-screen-md mx-auto">
            <Navbar />
        </header>
    )
}
