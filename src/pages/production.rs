use crate::api::{get_form_context, list_batches, RecordBatch, UndoBatch};
use crate::components::*;
use crate::i18n::{use_lang, use_t};
use crate::model::*;
use leptos::prelude::*;

#[component]
pub fn ProductionPage() -> impl IntoView {
    let t = use_t();
    let user = use_user();
    let group = use_group();
    let undo = ServerAction::<UndoBatch>::new();
    let batches = Resource::new(
        move || (group.get(), undo.version().get()),
        |(g, _)| list_batches(g),
    );

    view! {
        <PageTitle title=t.production_title/>
        {move || undo.value().get().and_then(|r| r.err()).map(|e| view! { <ErrorNote error=e/> })}
        <Transition fallback=Loading>
            {move || Suspend::new(async move {
                let user = match current_user(user).await {
                    Ok(u) => u,
                    Err(e) => return view! { <ErrorNote error=e/> }.into_any(),
                };
                let list = match batches.await {
                    Ok(list) => list,
                    Err(e) => return error_or_picker(e),
                };
                let g = group.get_untracked();
                view! {
                    {user.role.can_record().then(|| view! {
                        <div class="as-actions">
                            <a class="as-btn as-btn--primary" href=with_group("/production/new", g)>
                                <Icon name="plus"/>{t.new_batch}
                            </a>
                        </div>
                    })}
                    {list.is_empty().then(|| view! { <p class="as-empty">{t.no_records}</p> })}
                    {(!list.is_empty()).then(|| view! {
                        <section class="as-section">
                            <h2 class="as-h2">{t.recent_batches}</h2>
                            <ul class="as-stack">
                                {list.into_iter().map(|b| view! {
                                    <BatchCard row=b undo=undo group=g can_undo=user.role.can_undo()/>
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
fn BatchCard(
    row: BatchRow,
    undo: ServerAction<UndoBatch>,
    group: Option<i64>,
    can_undo: bool,
) -> impl IntoView {
    let t = use_t();
    let lang = use_lang();
    let p = row.product.clone();
    let unit_cost = (row.material_cost > 0).then(|| {
        Price {
            amount_tzs: row.material_cost,
            per_quantity: row.quantity,
        }
        .value_of(p.factor)
    });
    view! {
        <li class="as-card" class:as-card--reversed=row.reversed>
            <div class="as-row">
                <span class=format!("as-tag as-tag--{}", p.line())>{p.name(lang).to_string()}</span>
                <span class="as-figure">{p.fmt_qty_unit(row.quantity, lang)}</span>
            </div>
            <p class="as-caption">
                {row.produced_on.clone()}
                {(!row.site.is_empty()).then(|| format!(" · {}", row.site))}
                " · "{t.recorded_by}" "{row.recorded_by.clone()}
            </p>
            {row.reversed.then(|| view! { <span class="as-tag as-tag--outline">{t.reversed}</span> })}
            <dl class="as-kv">
                <dt>{t.estimated_cost}</dt><dd>{fmt_tzs(row.material_cost)}</dd>
                {unit_cost.map(|c| view! { <dt>{t.cost_per_unit}" "{p.unit(lang).to_string()}</dt><dd>{fmt_tzs(c)}</dd> })}
            </dl>
            {(!row.ingredients.is_empty()).then(|| view! {
                <details class="as-details">
                    <summary>{t.ingredients}" ("{row.ingredients.len()}")"</summary>
                    <ul class="as-ingredients">
                        {row.ingredients.iter().map(|i| view! {
                            <li><span>{i.asset.name(lang).to_string()}</span><span>{i.asset.fmt_qty_unit(i.amount, lang)}</span></li>
                        }).collect_view()}
                    </ul>
                </details>
            })}
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

#[component]
pub fn NewBatchPage() -> impl IntoView {
    let t = use_t();
    let group = use_group();
    let save = ServerAction::<RecordBatch>::new();
    let ctx = Resource::new(move || group.get(), get_form_context);
    view! {
        <a class="as-backlink" href=move || with_group("/production", group.get())><Icon name="back"/>{t.back}</a>
        <PageTitle title=t.new_batch/>
        <Suspense fallback=Loading>
            {move || Suspend::new(async move {
                match ctx.await {
                    Ok(c) => view! { <BatchForm ctx=c save=save group=group.get_untracked()/> }.into_any(),
                    Err(e) => error_or_picker(e),
                }
            })}
        </Suspense>
    }
}

#[component]
fn BatchForm(
    ctx: FormContext,
    save: ServerAction<RecordBatch>,
    group: Option<i64>,
) -> impl IntoView {
    let t = use_t();
    let lang = use_lang();
    let products = StoredValue::new(ctx.products.clone());
    let product_id = RwSignal::new(ctx.products.first().map(|p| p.asset.id).unwrap_or_default());
    let quantity = RwSignal::new(String::new());
    let materials: Vec<(PricedAsset, RwSignal<String>)> = ctx
        .materials
        .iter()
        .cloned()
        .map(|m| (m, RwSignal::new(String::new())))
        .collect();
    let mats = StoredValue::new(materials.clone());

    let selected = move || {
        products.with_value(|ps| ps.iter().find(|p| p.asset.id == product_id.get()).cloned())
    };
    let cost = Memo::new(move |_| {
        mats.with_value(|ms| {
            ms.iter()
                .filter_map(|(m, s)| Some(m.price?.value_of(m.asset.parse_qty(&s.get())?)))
                .sum::<i64>()
        })
    });
    let qty_base = move || {
        selected()
            .and_then(|p| p.asset.parse_qty(&quantity.get()))
            .filter(|q| *q > 0)
    };
    let unit_cost = move || {
        let p = selected()?;
        let q = qty_base()?;
        (cost.get() > 0).then(|| {
            Price {
                amount_tzs: cost.get(),
                per_quantity: q,
            }
            .value_of(p.asset.factor)
        })
    };
    let income = move || Some(selected()?.price?.value_of(qty_base()?));
    let losing = move || match (unit_cost(), selected().and_then(|p| p.price_per_unit())) {
        (Some(c), Some(p)) => c >= p,
        _ => false,
    };
    let unit = move || {
        selected()
            .map(|p| p.asset.unit(lang).to_string())
            .unwrap_or_default()
    };

    view! {
        <ActionForm action=save attr:class="as-form">
            <input type="hidden" name="key" value=ctx.key.clone()/>
            <GroupHidden group=group/>
            <fieldset class="as-field">
                <legend class="as-field__label">{t.product}</legend>
                <div class="as-choices">
                    {ctx.products.iter().enumerate().map(|(i, p)| {
                        let id = p.asset.id;
                        view! {
                            <label class=format!("as-choice as-choice--{}", p.asset.line())>
                                <input type="radio" name="product_id" value=id.to_string() checked=i == 0
                                    on:change=move |_| product_id.set(id)/>
                                <span>{p.asset.name(lang).to_string()}</span>
                            </label>
                        }
                    }).collect_view()}
                </div>
            </fieldset>
            <label class="as-field">
                <span class="as-field__label">{t.quantity_made}" ("{unit}")"</span>
                <input class="as-field__input as-field__input--lg" name="quantity" inputmode="decimal"
                    autocomplete="off" required placeholder="0"
                    on:input=move |ev| quantity.set(event_target_value(&ev))/>
            </label>
            <div class="as-grid-2">
                <label class="as-field">
                    <span class="as-field__label">{t.date}</span>
                    <input class="as-field__input" type="date" name="produced_on" value=ctx.today.clone()
                        max=ctx.today.clone() required/>
                </label>
                <label class="as-field">
                    <span class="as-field__label">{t.site}</span>
                    <input class="as-field__input" name="site" value=ctx.last_site.clone() autocomplete="off"/>
                </label>
            </div>

            <fieldset class="as-field as-materials">
                <legend class="as-field__label">{t.materials_used}</legend>
                <p class="as-field__help">{t.materials_hint}</p>
                {materials.into_iter().map(|(m, signal)| {
                    let a = m.asset.clone();
                    let price_hint = m.price_per_unit().map(|p| format!("{} / {}", fmt_tzs(p), a.unit(lang)));
                    view! {
                        <label class="as-material">
                            <span class="as-material__name">
                                {a.name(lang).to_string()}
                                {price_hint.map(|h| view! { <small>{h}</small> })}
                            </span>
                            <span class="as-material__input">
                                <input class="as-field__input" name=format!("amounts[{}]", a.code) inputmode="decimal"
                                    autocomplete="off" placeholder="0"
                                    on:input=move |ev| signal.set(event_target_value(&ev))/>
                                <span class="as-unit">{a.unit(lang).to_string()}</span>
                            </span>
                        </label>
                    }
                }).collect_view()}
            </fieldset>

            <section class="as-estimate" aria-live="polite">
                <dl class="as-kv">
                    <dt>{t.estimated_cost}</dt><dd>{move || fmt_tzs(cost.get())}</dd>
                    <dt>{t.cost_per_unit}" "{unit}</dt><dd>{move || unit_cost().map(fmt_tzs).unwrap_or_else(|| "–".into())}</dd>
                    <dt>{t.expected_income}</dt><dd>{move || income().map(fmt_tzs).unwrap_or_else(|| "–".into())}</dd>
                    <dt>{t.expected_profit}</dt>
                    <dd>{move || income().map(|i| fmt_tzs(i - cost.get())).unwrap_or_else(|| "–".into())}</dd>
                </dl>
                {move || losing().then(|| view! {
                    <p class="as-note as-note--danger" role="alert"><Icon name="alert"/><span>{t.warn_cost_over_price}</span></p>
                })}
            </section>

            <label class="as-field">
                <span class="as-field__label">{t.comment}" ("{t.optional}")"</span>
                <textarea class="as-field__input" name="comment" rows="2"></textarea>
            </label>
            {move || save.value().get().and_then(|r| r.err()).map(|e| view! { <ErrorNote error=e/> })}
            <button class="as-btn as-btn--primary as-btn--block" type="submit" disabled=move || save.pending().get()>
                {move || if save.pending().get() { t.saving } else { t.record_batch }}
            </button>
        </ActionForm>
    }
}
