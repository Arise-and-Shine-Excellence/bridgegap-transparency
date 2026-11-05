use crate::api::{get_form_context, list_sales, RecordSale, UndoSale};
use crate::components::*;
use crate::i18n::{use_lang, use_t};
use crate::model::*;
use leptos::prelude::*;

#[component]
pub fn SalesPage() -> impl IntoView {
    let t = use_t();
    let user = use_user();
    let group = use_group();
    let save = ServerAction::<RecordSale>::new();
    let undo = ServerAction::<UndoSale>::new();

    // Count successful saves: a new count fetches a fresh form (new
    // idempotency key, empty inputs) and the updated list. A failed save keeps
    // the form as it is so nothing typed is lost.
    let saved = RwSignal::new(0usize);
    Effect::new(move |_| {
        if let Some(Ok(_)) = save.value().get() {
            saved.update(|n| *n += 1);
        }
    });
    let ctx = Resource::new(
        move || (group.get(), saved.get(), undo.version().get()),
        |(g, _, _)| get_form_context(g),
    );
    let sales = Resource::new(
        move || (group.get(), saved.get(), undo.version().get()),
        |(g, _, _)| list_sales(g),
    );

    view! {
        <PageTitle title=t.sales_title/>
        {move || undo.value().get().and_then(|r| r.err()).map(|e| view! { <ErrorNote error=e/> })}
        <Transition fallback=Loading>
            {move || Suspend::new(async move {
                let user = match current_user(user).await {
                    Ok(u) => u,
                    Err(e) => return view! { <ErrorNote error=e/> }.into_any(),
                };
                let g = group.get_untracked();
                let form = if user.role.can_record() {
                    match ctx.await {
                        Ok(c) => view! { <SaleForm ctx=c save=save group=g/> }.into_any(),
                        Err(e) => return error_or_picker(e),
                    }
                } else {
                    ().into_any()
                };
                let list = match sales.await {
                    Ok(list) => list,
                    Err(e) => return error_or_picker(e),
                };
                view! {
                    {form}
                    {list.is_empty().then(|| view! { <p class="as-empty">{t.no_records}</p> })}
                    {(!list.is_empty()).then(|| view! {
                        <section class="as-section">
                            <h2 class="as-h2">{t.recent_sales}</h2>
                            <ul class="as-stack">
                                {list.into_iter().map(|s| view! {
                                    <SaleCard row=s undo=undo group=g can_undo=user.role.can_undo()/>
                                }).collect_view()}
                            </ul>
                        </section>
                    })}
                }.into_any()
            })}
        </Transition>
    }
}

#[component]
fn SaleForm(ctx: FormContext, save: ServerAction<RecordSale>, group: Option<i64>) -> impl IntoView {
    let t = use_t();
    let lang = use_lang();
    let products = StoredValue::new(ctx.products.clone());
    let stock = StoredValue::new(ctx.stock.clone());
    let first_id = ctx
        .products
        .iter()
        .find(|p| ctx.stock_of(p.asset.id) > 0)
        .or(ctx.products.first())
        .map(|p| p.asset.id);
    let product_id = RwSignal::new(first_id.unwrap_or_default());
    let quantity = RwSignal::new(String::new());

    let selected = move || {
        products.with_value(|ps| ps.iter().find(|p| p.asset.id == product_id.get()).cloned())
    };
    let in_stock = move || {
        stock.with_value(|s| {
            s.iter()
                .find(|(id, _)| *id == product_id.get())
                .map(|(_, q)| *q)
                .unwrap_or(0)
        })
    };
    let total = move || {
        let p = selected()?;
        Some(p.price?.value_of(p.asset.parse_qty(&quantity.get())?))
    };
    let unit = move || {
        selected()
            .map(|p| p.asset.unit(lang).to_string())
            .unwrap_or_default()
    };
    let last_amount = move || save.value().get().and_then(|r| r.ok());

    view! {
        <section class="as-card as-form-card">
            <h2 class="as-h2">{t.record_sale}</h2>
            {move || last_amount().map(|a| view! { <SuccessNote>{t.sale_saved}" "{fmt_tzs(a)}</SuccessNote> })}
            <ActionForm action=save attr:class="as-form">
                <input type="hidden" name="key" value=ctx.key.clone()/>
                <GroupHidden group=group/>
                <fieldset class="as-field">
                    <legend class="as-field__label">{t.product}</legend>
                    <div class="as-choices">
                        {ctx.products.iter().map(|p| {
                            let id = p.asset.id;
                            let checked = first_id == Some(id);
                            let available = p.asset.fmt_qty_unit(ctx.stock_of(id), lang);
                            view! {
                                <label class=format!("as-choice as-choice--{}", p.asset.line())>
                                    <input type="radio" name="product_id" value=id.to_string() checked=checked
                                        on:change=move |_| product_id.set(id)/>
                                    <span>
                                        {p.asset.name(lang).to_string()}
                                        <small>{available}" "{t.available}</small>
                                    </span>
                                </label>
                            }
                        }).collect_view()}
                    </div>
                </fieldset>
                <div class="as-grid-2">
                    <label class="as-field">
                        <span class="as-field__label">{t.quantity_sold}" ("{unit}")"</span>
                        <input class="as-field__input as-field__input--lg" name="quantity" inputmode="decimal"
                            autocomplete="off" required placeholder="0" prop:value=move || quantity.get()
                            on:input=move |ev| quantity.set(event_target_value(&ev))/>
                    </label>
                    <label class="as-field">
                        <span class="as-field__label">{t.date}</span>
                        <input class="as-field__input" type="date" name="sold_on" value=ctx.today.clone()
                            max=ctx.today.clone() required/>
                    </label>
                </div>
                <div class="as-total" aria-live="polite">
                    <span class="as-label">{t.total}</span>
                    <span class="as-figure">{move || total().map(fmt_tzs).unwrap_or_else(|| "TSh 0".into())}</span>
                    <span class="as-caption">
                        {move || selected().and_then(|p| p.price_per_unit()).map(|p| format!("{} {} {}", fmt_tzs(p), t.per, unit()))}
                        " · "{move || selected().map(|p| p.asset.fmt_qty_unit(in_stock(), lang))}" "{t.available}
                    </span>
                </div>
                <details class="as-details">
                    <summary>{t.comment}" ("{t.optional}")"</summary>
                    <textarea class="as-field__input" name="comment" rows="2" prop:value=""></textarea>
                </details>
                {move || save.value().get().and_then(|r| r.err()).map(|e| view! { <ErrorNote error=e/> })}
                <button class="as-btn as-btn--primary as-btn--block" type="submit" disabled=move || save.pending().get()>
                    {move || if save.pending().get() { t.saving } else { t.save }}
                </button>
            </ActionForm>
        </section>
    }
}

#[component]
fn SaleCard(
    row: SaleRow,
    undo: ServerAction<UndoSale>,
    group: Option<i64>,
    can_undo: bool,
) -> impl IntoView {
    let t = use_t();
    let lang = use_lang();
    let p = row.product.clone();
    view! {
        <li class="as-card" class:as-card--reversed=row.reversed>
            <div class="as-row">
                <span class=format!("as-tag as-tag--{}", p.line())>{p.name(lang).to_string()}</span>
                <span class="as-figure">{fmt_tzs(row.amount_tzs)}</span>
            </div>
            <p class="as-caption">
                {p.fmt_qty_unit(row.quantity, lang)}" · "{row.sold_on.clone()}" · "{t.recorded_by}" "{row.recorded_by.clone()}
            </p>
            {row.reversed.then(|| view! { <span class="as-tag as-tag--outline">{t.reversed}</span> })}
            {(!row.comment.is_empty()).then(|| view! { <p class="as-comment">{row.comment.clone()}</p> })}
            {(can_undo && !row.reversed).then(|| view! {
                <details class="as-undo">
                    <summary>{t.undo}</summary>
                    <p class="as-caption">{t.undo_explain}</p>
                    <ActionForm action=undo>
                        <input type="hidden" name="id" value=row.id.to_string()/>
                        <GroupHidden group=group/>
                        <button class="as-btn as-btn--danger" type="submit">{t.undo_confirm}</button>
                    </ActionForm>
                </details>
            })}
        </li>
    }
}
