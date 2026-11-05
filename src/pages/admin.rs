use crate::api::{get_admin_data, CreateGroup, CreateMember, SetPrice};
use crate::components::*;
use crate::i18n::{use_lang, use_t};
use crate::model::*;
use crate::pages::more::PriceTable;
use leptos::prelude::*;

#[component]
pub fn AdminPage() -> impl IntoView {
    let t = use_t();
    let lang = use_lang();
    let add_group = ServerAction::<CreateGroup>::new();
    let add_member = ServerAction::<CreateMember>::new();
    let set_price = ServerAction::<SetPrice>::new();
    let data = Resource::new(
        move || {
            (
                add_group.version().get(),
                add_member.version().get(),
                set_price.version().get(),
            )
        },
        |_| get_admin_data(),
    );

    view! {
        <a class="as-backlink" href="/more"><Icon name="back"/>{t.back}</a>
        <PageTitle title=t.admin_title/>
        <Transition fallback=Loading>
            {move || Suspend::new(async move {
                let d = match data.await {
                    Ok(d) => d,
                    Err(e) => return view! { <ErrorNote error=e/> }.into_any(),
                };
                let groups_for_select = d.groups.iter().map(|g| g.group.clone()).collect::<Vec<_>>();
                view! {
                    <section class="as-section">
                        <h2 class="as-h2">{t.groups}</h2>
                        <ul class="as-stack">
                            {d.groups.iter().map(|g| view! {
                                <li class="as-card">
                                    <div class="as-row">
                                        <h3 class="as-h3">{g.group.name.clone()}</h3>
                                        <a class="as-link" href=with_group("/", Some(g.group.id))>{t.nav_home}</a>
                                    </div>
                                    <MemberList members=g.members.clone()/>
                                </li>
                            }).collect_view()}
                            <li class="as-card">
                                <h3 class="as-h3">"Arise and Shine Excellence"</h3>
                                <MemberList members=d.ase_members.clone()/>
                            </li>
                        </ul>
                    </section>

                    <section class="as-card as-form-card">
                        <h2 class="as-h2">{t.add_group}</h2>
                        <ActionForm action=add_group attr:class="as-form">
                            <label class="as-field">
                                <span class="as-field__label">{t.group_name}</span>
                                <input class="as-field__input" name="name" required autocomplete="off"/>
                            </label>
                            <ActionResult result=add_group.value() ok=t.created/>
                            <button class="as-btn as-btn--primary" type="submit">{t.create}</button>
                        </ActionForm>
                    </section>

                    <section class="as-card as-form-card">
                        <h2 class="as-h2">{t.add_member}</h2>
                        <ActionForm action=add_member attr:class="as-form">
                            <label class="as-field">
                                <span class="as-field__label">{t.group}</span>
                                <select class="as-field__input" name="group_id">
                                    {groups_for_select.into_iter().map(|g| view! {
                                        <option value=g.id.to_string()>{g.name}</option>
                                    }).collect_view()}
                                    <option value="0">"Arise and Shine Excellence (ASE)"</option>
                                </select>
                            </label>
                            <label class="as-field">
                                <span class="as-field__label">{t.name}</span>
                                <input class="as-field__input" name="name" required autocomplete="off"/>
                            </label>
                            <div class="as-grid-2">
                                <label class="as-field">
                                    <span class="as-field__label">{t.phone}</span>
                                    <input class="as-field__input" type="tel" name="phone" inputmode="tel" required
                                        placeholder="07xx xxx xxx" autocomplete="off"/>
                                </label>
                                <label class="as-field">
                                    <span class="as-field__label">{t.pin}</span>
                                    <input class="as-field__input" name="pin" inputmode="numeric" pattern="[0-9]{4,6}"
                                        maxlength="6" required autocomplete="off"/>
                                </label>
                            </div>
                            <p class="as-field__help">{t.pin_hint}</p>
                            <label class="as-field">
                                <span class="as-field__label">{t.role}</span>
                                <select class="as-field__input" name="role">
                                    {[Role::Member, Role::Leader, Role::Admin, Role::Viewer].into_iter().map(|r| view! {
                                        <option value=r.code()>{r.label(lang)}</option>
                                    }).collect_view()}
                                </select>
                            </label>
                            <ActionResult result=add_member.value() ok=t.created/>
                            <button class="as-btn as-btn--primary" type="submit">{t.create}</button>
                        </ActionForm>
                    </section>

                    <section class="as-section">
                        <h2 class="as-h2">{t.prices_title}</h2>
                        <PriceTable list=d.priced.clone()/>
                    </section>

                    <section class="as-card as-form-card">
                        <h2 class="as-h2">{t.change_price}</h2>
                        <ActionForm action=set_price attr:class="as-form">
                            <label class="as-field">
                                <span class="as-field__label">{t.product}</span>
                                <select class="as-field__input" name="asset_id">
                                    {d.priced.iter().map(|p| view! {
                                        <option value=p.asset.id.to_string()>
                                            {format!("{} ({} {})", p.asset.name(lang), t.per, p.asset.unit(lang))}
                                        </option>
                                    }).collect_view()}
                                </select>
                            </label>
                            <label class="as-field">
                                <span class="as-field__label">{t.new_price_tzs}</span>
                                <input class="as-field__input" name="amount_tzs" inputmode="numeric" required autocomplete="off"/>
                            </label>
                            <ActionResult result=set_price.value() ok=t.price_changed/>
                            <button class="as-btn as-btn--primary" type="submit">{t.save}</button>
                        </ActionForm>
                    </section>
                }.into_any()
            })}
        </Transition>
    }
}

#[component]
fn MemberList(members: Vec<MemberRow>) -> impl IntoView {
    let t = use_t();
    let lang = use_lang();
    if members.is_empty() {
        return view! { <p class="as-caption">{t.no_records}</p> }.into_any();
    }
    view! {
        <ul class="as-members">
            {members.into_iter().map(|m| view! {
                <li>
                    <span>{m.name}</span>
                    <span class="as-caption">{m.role.label(lang)}" · "{m.phone}</span>
                </li>
            }).collect_view()}
        </ul>
    }
    .into_any()
}

#[component]
fn ActionResult(
    #[prop(into)] result: Signal<Option<Result<(), crate::error::AppError>>>,
    ok: &'static str,
) -> impl IntoView {
    move || match result.get() {
        Some(Ok(())) => Some(view! { <SuccessNote>{ok}</SuccessNote> }.into_any()),
        Some(Err(e)) => Some(view! { <ErrorNote error=e/> }.into_any()),
        None => None,
    }
}
