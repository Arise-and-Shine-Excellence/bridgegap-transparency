use crate::api::Login;
use crate::components::ErrorNote;
use crate::i18n::use_t;
use crate::pages::LanguageSwitch;
use leptos::prelude::*;

#[component]
pub fn LoginPage() -> impl IntoView {
    let t = use_t();
    let login = ServerAction::<Login>::new();
    let error = move || login.value().get().and_then(|r| r.err());

    view! {
        <div class="as-login">
            <div class="as-login__hero" aria-hidden="true"></div>
            <main class="as-login__card">
                <h1 class="as-display">"Arise and Shine"</h1>
                <p class="as-muted">{t.login_intro}</p>
                <ActionForm action=login attr:class="as-form">
                    <label class="as-field">
                        <span class="as-field__label">{t.phone}</span>
                        <input class="as-field__input" type="tel" name="phone" inputmode="tel"
                            autocomplete="tel" placeholder="07xx xxx xxx" required/>
                    </label>
                    <label class="as-field">
                        <span class="as-field__label">{t.pin}</span>
                        <input class="as-field__input" type="password" name="pin" inputmode="numeric"
                            autocomplete="current-password" pattern="[0-9]{4,6}" maxlength="6" required/>
                    </label>
                    {move || error().map(|e| view! { <ErrorNote error=e/> })}
                    <button class="as-btn as-btn--primary as-btn--block" type="submit"
                        disabled=move || login.pending().get()>
                        {move || if login.pending().get() { t.saving } else { t.log_in }}
                    </button>
                </ActionForm>
                <LanguageSwitch/>
            </main>
        </div>
    }
}
