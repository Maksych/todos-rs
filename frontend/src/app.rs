use yew::prelude::*;
use yew_router::prelude::*;

use crate::{router, store::StoreProvider};

#[function_component(App)]
pub fn app() -> Html {
    html!(
        <BrowserRouter>
            <StoreProvider>
                <Switch<router::Route> render={router::switch} />
            </StoreProvider>
        </BrowserRouter>
    )
}
