use leptos::prelude::*;

const OPENERS: [&str; 8] = [
    "You walk into a noodle bar. The chef is a fortune teller who only tells the truth on Tuesdays.",
    "The elevator doors open. Everyone inside is dressed as you, but younger.",
    "You've just been elected mayor of a city that only exists in dreams.",
    "The vending machine on this floor dispenses regrets instead of snacks.",
    "Your reflection waves first.",
    "You're the last human at a robot open-mic night, and you're up next.",
    "The fortune cookie has your home address in it.",
    "Somebody left a stranger's diary open to today's date.",
];

const YES_ANDS: [&str; 12] = [
    "the fortune teller owes you money from a past life.",
    "one of them is already crying about a breakup you haven't had yet.",
    "your first executive order is mandatory nap time.",
    "it's out of regrets, so it gives you a coupon for one free bad decision.",
    "it winks.",
    "the robot in the front row heckles you in binary.",
    "the fortune cookie also lists your wifi password.",
    "the diary's last entry is a five-star review of this exact moment.",
    "somewhere, a kazoo starts playing the national anthem.",
    "you realize you've been rehearsing this apology to a houseplant.",
    "the houseplant accepts the apology, but wants it in writing.",
    "everyone in the room starts slow-clapping for no discernible reason.",
];

fn rand_index(len: usize) -> usize {
    ((js_sys::Math::random() * len as f64) as usize).min(len - 1)
}

fn rand_pick(pool: &[&'static str], avoid: Option<&'static str>) -> &'static str {
    if pool.len() == 1 {
        return pool[0];
    }
    loop {
        let candidate = pool[rand_index(pool.len())];
        if Some(candidate) != avoid {
            return candidate;
        }
    }
}

const MAX_BEATS: usize = 8;

#[component]
pub fn App() -> impl IntoView {
    let (opening, set_opening) = signal(rand_pick(&OPENERS, None));
    let (lines, set_lines) = signal::<Vec<&'static str>>(Vec::new());

    let beats = move || lines.get().len();

    let yes_and = move |_| {
        set_lines.update(|l| {
            let avoid = l.last().copied();
            let next = rand_pick(&YES_ANDS, avoid);
            l.push(next);
        });
    };

    let new_scene = move |_| {
        let current = opening.get_untracked();
        set_opening.set(rand_pick(&OPENERS, Some(current)));
        set_lines.set(Vec::new());
    };

    view! {
        <div class="stage">
            <div class="scanlines"></div>

            <header class="hud">
                <span class="hud-tag">"IMPROV.SYS"</span>
                <span class="hud-tag hud-tag--right">"NO SCRIPT // NO NET"</span>
            </header>

            <main class="console">
                <h1 class="glitch" data-text="YES, AND">"YES, AND"</h1>
                <p class="subtitle">"the improv equivalent of hello world"</p>

                <div class="scene-card">
                    <div class="line">
                        <span class="tag tag--stranger">"STRANGER"</span>
                        <p>{move || opening.get()}</p>
                    </div>

                    <For
                        each=move || lines.get().into_iter().enumerate()
                        key=|(i, _)| *i
                        children=move |(i, text)| {
                            let (tag_text, tag_class) = if i % 2 == 0 {
                                ("YOU", "tag tag--you")
                            } else {
                                ("SCENE PARTNER", "tag tag--partner")
                            };
                            view! {
                                <div class="line">
                                    <span class=tag_class>{tag_text}</span>
                                    <p>{format!("...and {}", text)}</p>
                                </div>
                            }
                        }
                    />
                </div>

                <div class="meter">
                    <div class="meter-label">
                        <span>"SCENE ENERGY"</span>
                        <span>{move || format!("{}/{}", beats(), MAX_BEATS)}</span>
                    </div>
                    <div class="meter-track">
                        <div
                            class="meter-fill"
                            style:width=move || format!("{}%", (beats() * 100 / MAX_BEATS).min(100))
                        ></div>
                    </div>
                </div>

                <div class="controls">
                    <button class="btn btn--primary" on:click=yes_and>"YES, AND →"</button>
                    <button class="btn btn--ghost" on:click=new_scene>"SCENE! (reset)"</button>
                </div>
            </main>

            <footer class="hud hud--footer">
                <span>"built with leptos + trunk, deployed on github pages"</span>
            </footer>
        </div>
    }
}
