use crate::api::{get_prices, Logout, SetLang};
use crate::components::*;
use crate::i18n::{use_lang, use_t, Lang};
use crate::model::*;
use leptos::prelude::*;
use leptos::server_fn::ServerFn;

#[component]
pub fn MorePage() -> impl IntoView {
    let t = use_t();
    let lang = use_lang();
    let user = use_user();
    let prices = Resource::new(|| (), |_| get_prices());

    view! {
        <PageTitle title=t.nav_more/>

        <Suspense fallback=Loading>
            {move || Suspend::new(async move {
                current_user(user).await.ok().map(|user| view! {
                    <section class="as-card">
                        <p class="as-eyebrow">{t.account}</p>
                        <h2 class="as-h3">{user.name.clone()}</h2>
                        <p class="as-caption">{user.role.label(lang)}" · "{user.organisation_name.clone()}" · "{user.phone.clone()}</p>
                    </section>
                    {(user.role == Role::Admin).then(|| view! {
                        <a class="as-list__link" href="/admin">
                            <span>{t.admin}</span>
                            <Icon name="next"/>
                        </a>
                    })}
                })
            })}
        </Suspense>

        <section class="as-section">
            <h2 class="as-h2">{t.language}</h2>
            <LanguageSwitch/>
        </section>

        <section class="as-section">
            <h2 class="as-h2">{t.prices_title}</h2>
            <p class="as-caption">{t.prices_intro}</p>
            <Suspense fallback=Loading>
                {move || Suspend::new(async move {
                    match prices.await {
                        Ok(list) => view! { <PriceTable list=list/> }.into_any(),
                        Err(e) => view! { <ErrorNote error=e/> }.into_any(),
                    }
                })}
            </Suspense>
        </section>

        // A plain form: logging out works the same with or without JavaScript.
        <form method="post" action=Logout::url() class="as-section">
            <button class="as-btn as-btn--secondary as-btn--block" type="submit">{t.log_out}</button>
        </form>
    }
}

#[component]
pub fn PriceTable(list: Vec<PricedAsset>) -> impl IntoView {
    let t = use_t();
    let lang = use_lang();
    let (products, materials): (Vec<_>, Vec<_>) =
        list.into_iter().partition(|p| p.asset.kind == "product");
    let rows = move |items: Vec<PricedAsset>| {
        items
            .into_iter()
            .map(|p| {
                let price = p
                    .price_per_unit()
                    .map(fmt_tzs)
                    .unwrap_or_else(|| "–".into());
                view! {
                    <tr>
                        <th scope="row">{p.asset.name(lang).to_string()}</th>
                        <td>{price}" / "{p.asset.unit(lang).to_string()}</td>
                    </tr>
                }
            })
            .collect_view()
    };
    view! {
        <table class="as-table">
            <caption>{t.sale_prices}</caption>
            <tbody>{rows(products)}</tbody>
        </table>
        <table class="as-table">
            <caption>{t.material_prices}</caption>
            <tbody>{rows(materials)}</tbody>
        </table>
    }
}

/// Two buttons in a plain form. The server stores the choice in a cookie and
/// sends the browser back to the same page, now in the chosen language.
#[component]
pub fn LanguageSwitch() -> impl IntoView {
    let current = use_lang();
    let button = move |lang: Lang, label: &'static str| {
        view! {
            <button class="as-seg__btn" type="submit" name="lang" value=lang.code()
                aria-pressed=if lang == current { "true" } else { "false" }>
                {label}
            </button>
        }
    };
    view! {
        <form method="post" action=SetLang::url() class="as-seg">
            {button(Lang::En, "English")}
            {button(Lang::Sw, "Kiswahili")}
        </form>
    }
}
