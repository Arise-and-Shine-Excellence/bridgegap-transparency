use crate::api::{get_dashboard, get_group_summaries};
use crate::components::*;
use crate::error::AppError;
use crate::i18n::{fill, use_lang, use_t};
use crate::model::*;
use leptos::prelude::*;

/// Members and leaders see their group's dashboard. ASE staff see every group
/// until they pick one (`?group=<id>`).
#[component]
pub fn HomePage() -> impl IntoView {
    let user = use_user();
    let group = use_group();
    let dashboard = Resource::new(move || group.get(), get_dashboard);
    // Only ASE staff may list all groups; for everyone else this is a no-op.
    let summaries = Resource::new(
        move || group.get(),
        |g| async move {
            match g {
                Some(_) => Ok(None),
                None => match get_group_summaries().await {
                    Ok(list) => Ok(Some(list)),
                    Err(AppError::Forbidden) => Ok(None),
                    Err(e) => Err(e),
                },
            }
        },
    );

    view! {
        <Suspense fallback=Loading>
            {move || Suspend::new(async move {
                let user = match current_user(user).await {
                    Ok(u) => u,
                    Err(e) => return view! { <ErrorNote error=e/> }.into_any(),
                };
                if user.role.sees_all_groups() && group.get_untracked().is_none() {
                    return match summaries.await {
                        Ok(list) => view! { <GroupOverview user=user list=list.unwrap_or_default()/> }.into_any(),
                        Err(e) => view! { <ErrorNote error=e/> }.into_any(),
                    };
                }
                match dashboard.await {
                    Ok(d) => view! { <GroupDashboard user=user d=d group=group.get_untracked()/> }.into_any(),
                    Err(e) => error_or_picker(e),
                }
            })}
        </Suspense>
    }
}

/// ASE staff: every group at a glance.
#[component]
fn GroupOverview(user: User, list: Vec<GroupSummary>) -> impl IntoView {
    let t = use_t();
    view! {
        <header class="as-page-title">
            <p class="as-eyebrow">{t.welcome}</p>
            <h1>{user.name}</h1>
        </header>
        <section class="as-section">
            <h2 class="as-h2">{t.all_groups}" · "{t.this_month}</h2>
            {list.is_empty().then(|| view! { <p class="as-muted">{t.no_records}</p> })}
            <ul class="as-stack">
                {list.into_iter().map(|s| {
                    let profit = s.revenue - s.material_cost;
                    view! {
                        <li>
                            <a class="as-card as-card--link" href=with_group("/", Some(s.group.id))>
                                <div class="as-row">
                                    <h3 class="as-h3">{s.group.name}</h3>
                                    {(s.alerts > 0).then(|| view! {
                                        <span class="as-tag as-tag--warning"><Icon name="alert"/>{s.alerts}</span>
                                    })}
                                </div>
                                <dl class="as-kv">
                                    <dt>{t.money_in}</dt><dd>{fmt_tzs(s.revenue)}</dd>
                                    <dt>{t.money_out}</dt><dd>{fmt_tzs(s.material_cost)}</dd>
                                    <dt>{if profit < 0 { t.loss } else { t.profit }}</dt>
                                    <dd class=if profit < 0 { "as-danger" } else { "as-success" }>{fmt_tzs(profit.abs())}</dd>
                                    <dt>{t.members}</dt><dd>{s.members}</dd>
                                </dl>
                            </a>
                        </li>
                    }
                }).collect_view()}
            </ul>
        </section>
    }
}

#[component]
fn GroupDashboard(user: User, d: Dashboard, group: Option<i64>) -> impl IntoView {
    let t = use_t();
    let lang = use_lang();
    let profit = d.profit();
    let staff = user.role.sees_all_groups();
    view! {
        <header class="as-page-title">
            {staff.then(|| view! {
                <a class="as-backlink" href="/"><Icon name="back"/>{t.all_groups}</a>
            })}
            <p class="as-eyebrow">{if staff { t.group.to_string() } else { t.welcome.to_string() }}</p>
            <h1>{if staff { d.group.name.clone() } else { user.name.clone() }}</h1>
        </header>

        <Alerts alerts=d.alerts.clone()/>

        <section class="as-card as-summary">
            <p class="as-eyebrow">{t.this_month}" · "{lang.month(d.month)}" "{d.year}</p>
            <dl class="as-kv as-kv--lg">
                <dt>{t.money_in}</dt><dd>{fmt_tzs(d.revenue)}</dd>
                <dt>{t.money_out}</dt><dd>{fmt_tzs(d.material_cost)}</dd>
            </dl>
            <div class="as-summary__result">
                <span class="as-label">{if profit < 0 { t.loss } else { t.profit }}</span>
                <span class=if profit < 0 { "as-figure as-danger" } else { "as-figure as-success" }>
                    {fmt_tzs(profit.abs())}
                </span>
            </div>
        </section>

        {user.role.can_record().then(|| view! {
            <div class="as-actions">
                <a class="as-btn as-btn--primary" href=with_group("/sales", group)>
                    <Icon name="plus"/>{t.record_sale}
                </a>
                <a class="as-btn as-btn--secondary" href=with_group("/production/new", group)>
                    <Icon name="plus"/>{t.new_batch}
                </a>
            </div>
        })}

        <section class="as-section">
            <h2 class="as-h2">{t.stock_title}</h2>
            <ul class="as-stack">
                {d.products.into_iter().map(|p| view! { <ProductCard line=p/> }).collect_view()}
            </ul>
        </section>
    }
}

#[component]
fn Alerts(alerts: Vec<Alert>) -> impl IntoView {
    let t = use_t();
    let lang = use_lang();
    if alerts.is_empty() {
        return view! {
            <p class="as-note as-note--success" role="status"><Icon name="check"/><span>{t.all_good}</span></p>
        }
        .into_any();
    }
    view! {
        <section class="as-alerts" aria-label=t.alerts_title>
            <h2 class="as-label">{t.alerts_title}</h2>
            {alerts.into_iter().map(|a| {
                let (danger, text) = match &a {
                    Alert::UnitLoss { product, unit_cost, price } => (true, fill(t.alert_unit_loss, &[
                        product.name(lang), &fmt_tzs(*unit_cost), product.unit(lang), &fmt_tzs(*price),
                    ])),
                    Alert::MonthLoss { loss, unsold_value } => {
                        let mut s = fill(t.alert_month_loss, &[&fmt_tzs(*loss)]);
                        if *unsold_value > 0 {
                            s.push(' ');
                            s.push_str(&fill(t.alert_month_loss_stock, &[&fmt_tzs(*unsold_value)]));
                        }
                        (false, s)
                    }
                    Alert::NegativeStock { product } => (true, fill(t.alert_negative_stock, &[product.name(lang)])),
                };
                view! {
                    <p class=if danger { "as-note as-note--danger" } else { "as-note as-note--warning" } role="alert">
                        <Icon name="alert"/><span>{text}</span>
                    </p>
                }
            }).collect_view()}
        </section>
    }
    .into_any()
}

#[component]
fn ProductCard(line: ProductLine) -> impl IntoView {
    let t = use_t();
    let lang = use_lang();
    let a = line.product.asset.clone();
    let unit = a.unit(lang).to_string();
    let price = line.product.price_per_unit();
    view! {
        <li class="as-card as-product">
            <div class="as-row">
                <span class=format!("as-tag as-tag--{}", a.line())>{a.name(lang).to_string()}</span>
                <span class="as-figure">{a.fmt_qty_unit(line.in_stock, lang)}</span>
            </div>
            <dl class="as-kv">
                <dt>{t.sold_this_month}</dt><dd>{a.fmt_qty_unit(line.sold_month, lang)}</dd>
                <dt>{t.produced_this_month}</dt><dd>{a.fmt_qty_unit(line.produced_month, lang)}</dd>
                {price.map(|p| view! { <dt>{t.sells_for}</dt><dd>{fmt_tzs(p)}" / "{unit.clone()}</dd> })}
                {line.last_unit_cost.map(|c| view! {
                    <dt>{t.cost_per_unit}" "{unit.clone()}</dt><dd>{fmt_tzs(c)}</dd>
                })}
            </dl>
        </li>
    }
}
