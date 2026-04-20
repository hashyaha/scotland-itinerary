// scotland-itinerary/src/main.rs
//
// Run:  cargo run
// Out:  index.html
//
// Uses `maud` for compile-time, type-safe HTML templating.
// Every stop, day, and label is a Rust value — no stringly-typed soup.

use maud::{html, Markup, PreEscaped, DOCTYPE};
use std::fs;

// ─────────────────────────────────────────────
// DATA MODEL
// ─────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq)]
enum Dot { Gold, Warn, Moss }

#[derive(Clone, Copy)]
enum Tag { Drive, Hike, Food, Castle, View, Stay, Fly, Warn, Opt }

impl Tag {
    fn css(self) -> &'static str {
        match self {
            Tag::Drive  => "t-drive",
            Tag::Hike   => "t-hike",
            Tag::Food   => "t-food",
            Tag::Castle => "t-castle",
            Tag::View   => "t-view",
            Tag::Stay   => "t-stay",
            Tag::Fly    => "t-fly",
            Tag::Warn   => "t-warn",
            Tag::Opt    => "t-opt",
        }
    }
}

#[derive(Clone)]
struct Stop {
    id:       &'static str,
    time:     &'static str,
    tag:      Tag,
    tag_text: &'static str,
    dot:      Dot,
    name:     &'static str,
    note:     &'static str,
}

#[derive(Clone)]
struct Day {
    id:    u8,
    tab:   &'static str,
    date:  &'static str,
    title: &'static str,
    alert: Option<&'static str>,
    stops: Vec<Stop>,
}

// ─────────────────────────────────────────────
// ITINERARY DATA
// ─────────────────────────────────────────────

fn itinerary() -> Vec<Day> {
    vec![
        Day {
            id: 0, tab: "Night 0", date: "Wed 30 Apr",
            title: "Arrive Edinburgh", alert: None,
            stops: vec![
                Stop { id:"n0", time:"21:00", tag:Tag::Fly,  tag_text:"Inbound · RK 8605", dot:Dot::Gold,
                       name:"Land Edinburgh Airport",
                       note:"Clear baggage claim. Pick up the minibus from the rental desk." },
                Stop { id:"n1", time:"22:00", tag:Tag::Food, tag_text:"Dinner",             dot:Dot::Gold,
                       name:"Quick dinner near airport",
                       note:"Stay near the airport — Prezzo, Frankie & Benny's, or hotel restaurant." },
                Stop { id:"n2", time:"22:30", tag:Tag::Stay, tag_text:"Stay",               dot:Dot::Moss,
                       name:"Hotel near Edinburgh Airport",
                       note:"Premier Inn Edinburgh Airport or Travelodge. Early night — 09:00 departure tomorrow." },
            ],
        },
        Day {
            id: 1, tab: "Day 1", date: "Thu 1 May",
            title: "Edinburgh → Loch Ness → Lochcarron",
            alert: Some("Go inside Urquhart Castle today — you have time. Book tickets online (~£12) to skip the queue."),
            stops: vec![
                Stop { id:"1a", time:"09:00", tag:Tag::Drive,  tag_text:"Drive ~2h45",       dot:Dot::Warn,
                       name:"🚐 Depart Edinburgh",
                       note:"Fuel up. Coffee to go. Route: M9 → A9 → Inverness area." },
                Stop { id:"1b", time:"11:45", tag:Tag::Castle, tag_text:"Castle · 1h15",     dot:Dot::Gold,
                       name:"🏰 Urquhart Castle — go inside!",
                       note:"Film in visitor centre, walk the ruins, climb the tower for loch views. Much more satisfying than a photo stop." },
                Stop { id:"1c", time:"13:00", tag:Tag::Food,   tag_text:"Lunch · 45 min",    dot:Dot::Gold,
                       name:"🍽 Lunch beside Loch Ness",
                       note:"Loch Ness Inn or cafés along the A82. Eat with a loch view if weather allows." },
                Stop { id:"1d", time:"13:45", tag:Tag::View,   tag_text:"Lochside · 30 min", dot:Dot::Gold,
                       name:"🌊 Dores Beach",
                       note:"Wide-open loch views. Peaceful, moody, very Highland. Coffee van often here. Flat shore walk — tourist-free." },
                Stop { id:"1e", time:"14:20", tag:Tag::Hike,   tag_text:"Waterfall · 1h",    dot:Dot::Gold,
                       name:"💧 Falls of Foyers",
                       note:"Dramatic waterfall into a gorge. 45–60 min walk down and back. Small café at the top. Absolutely worth it." },
                Stop { id:"1f", time:"15:30", tag:Tag::Drive,  tag_text:"Drive ~3h via A87", dot:Dot::Gold,
                       name:"🚐 Head west to Lochcarron",
                       note:"Scenic A87 through Glen Shiel. One comfort stop en route is fine." },
                Stop { id:"1g", time:"18:30", tag:Tag::Stay,   tag_text:"Base · 2 nights",   dot:Dot::Moss,
                       name:"🏡 Arrive Lochcarron",
                       note:"Check in, dinner in the village. Still light until 21:30 — optional loch waterfront walk after dinner." },
            ],
        },
        Day {
            id: 2, tab: "Day 2", date: "Fri 2 May",
            title: "Isle of Skye — Northern Loop",
            alert: Some("07:00 departure is firm. Old Man of Storr car park fills by 9am on good days. A slow breakfast costs you the spot."),
            stops: vec![
                Stop { id:"2a", time:"07:00", tag:Tag::Drive, tag_text:"Drive ~1h20",       dot:Dot::Warn,
                       name:"🚐 Depart Lochcarron",
                       note:"Cross Skye Bridge via Kyle of Lochalsh. Head north on A855 towards the Storr." },
                Stop { id:"2b", time:"08:20", tag:Tag::Hike,  tag_text:"Main Hike · 2h40",  dot:Dot::Gold,
                       name:"🏔 Old Man of Storr",
                       note:"Take the LEFT fork — gentler, better views. Parking ~£5. 3.8km return. Walk around the pinnacles." },
                Stop { id:"2c", time:"11:20", tag:Tag::View,  tag_text:"Viewpoint · 25 min", dot:Dot::Gold,
                       name:"🌊 Kilt Rock & Mealt Falls",
                       note:"Basalt cliff columns + waterfall straight into the sea. Steps from car park. Parking £3." },
                Stop { id:"2d", time:"12:10", tag:Tag::View,  tag_text:"Wander · 40 min",   dot:Dot::Gold,
                       name:"🧚 Fairy Glen",
                       note:"Whimsical mini hills near Uig. Unlike anywhere else on Skye. Parking £3." },
                Stop { id:"2e", time:"13:00", tag:Tag::Food,  tag_text:"Lunch · 60 min",    dot:Dot::Gold,
                       name:"🍽 Portree Harbour",
                       note:"Colourful harbour, great seafood. Short walk along the pier after." },
                Stop { id:"2f", time:"14:15", tag:Tag::Drive, tag_text:"Drive ~1h20",       dot:Dot::Gold,
                       name:"🚐 Return to Lochcarron",
                       note:"Back by ~15:35. Six hours of daylight remaining." },
                Stop { id:"2g", time:"19:00", tag:Tag::Opt,   tag_text:"✨ Optional (light till 21:30)", dot:Dot::Moss,
                       name:"🌅 Evening: Attadale walk or loch stroll",
                       note:"Attadale waterfront path (~1h return) is beautiful in evening light. Or walk the shore with a dram." },
            ],
        },
        Day {
            id: 3, tab: "Day 3", date: "Sat 3 May",
            title: "West Skye → Glenfinnan → Glencoe",
            alert: Some("07:30 departure non-negotiable. Longest day — zero slack. Talisker Bay can be cut if weather is bad (saves ~1h)."),
            stops: vec![
                Stop { id:"3a", time:"07:30", tag:Tag::Drive,  tag_text:"Drive ~45 min",       dot:Dot::Warn,
                       name:"🚐 Depart Lochcarron", note:"A890 → A87 towards Dornie." },
                Stop { id:"3b", time:"07:45", tag:Tag::Castle, tag_text:"Photo stop · 25 min",  dot:Dot::Gold,
                       name:"🏰 Eilean Donan Castle",
                       note:"Stop 200m east on A87 for the full shot — castle + bridge + loch. Do NOT go inside. Morning light is perfect." },
                Stop { id:"3c", time:"08:10", tag:Tag::Drive,  tag_text:"Drive ~50 min",        dot:Dot::Gold,
                       name:"🚐 Cross to Skye", note:"Kyle of Lochalsh → Skye Bridge → south to Talisker." },
                Stop { id:"3d", time:"09:00", tag:Tag::Hike,   tag_text:"Coastal walk · 70 min", dot:Dot::Gold,
                       name:"🏖 Talisker Bay",
                       note:"Black volcanic sand. 20 min walk each way. Walk all the way to the waterfall at the far end. Skip if rain/wind is bad." },
                Stop { id:"3e", time:"10:50", tag:Tag::View,   tag_text:"Photo stop · 25 min",  dot:Dot::Gold,
                       name:"🌉 Sligachan Bridge",
                       note:"Classic bridge + Black Cuillin view. Coffee at pub. Dip face in river — 7 years of beauty 😄" },
                Stop { id:"3f", time:"11:20", tag:Tag::Drive,  tag_text:"Drive ~2h20",           dot:Dot::Warn,
                       name:"🚐 Leave Skye — head south",
                       note:"Discipline moment. No extra stops. Lunch in Fort William in ~1h20." },
                Stop { id:"3g", time:"12:30", tag:Tag::Food,   tag_text:"Lunch · 45 min",       dot:Dot::Gold,
                       name:"🍽 Fort William", note:"Fuel up properly. Supermarkets here for supplies." },
                Stop { id:"3h", time:"13:45", tag:Tag::View,   tag_text:"Harry Potter · 75 min", dot:Dot::Gold,
                       name:"🚂 Glenfinnan Viaduct",
                       note:"Walk 10–15 min uphill from car park (go left). Jacobite Train return ≈ 15:10. Parking £3.50." },
                Stop { id:"3i", time:"15:30", tag:Tag::View,   tag_text:"James Bond · 45 min",  dot:Dot::Gold,
                       name:"🏔 Glen Etive — Skyfall Road",
                       note:"Dead-end single-track. Drive in, stop anywhere, back out same way. Go slow with the minibus." },
                Stop { id:"3j", time:"16:30", tag:Tag::View,   tag_text:"Roadside · 30 min",    dot:Dot::Gold,
                       name:"⛰ Buachaille Etive Mòr",
                       note:"Lay-by at Glen Etive junction on A82. Mountain reflects in River Coupall on calm days." },
                Stop { id:"3k", time:"17:00", tag:Tag::Stay,   tag_text:"Base · 1 night",       dot:Dot::Moss,
                       name:"🏡 Check in — Glencoe or Fort William",
                       note:"Still 5 hours of daylight. Optional evening walk below." },
                Stop { id:"3l", time:"18:30", tag:Tag::Opt,    tag_text:"✨ Optional (light till 22:00)", dot:Dot::Moss,
                       name:"🌄 Evening: Glencoe Lochan forest walk",
                       note:"Flat ~1h circuit — magical in evening light. Completely different atmosphere to the big ridges. Signposted from the village." },
            ],
        },
        Day {
            id: 4, tab: "Day 4", date: "Sun 4 May",
            title: "Glencoe Hike → Edinburgh → ✈ Home",
            alert: Some("12:30 is the most critical departure of the trip. Set a group alarm for 12:00. Agree a turnaround time before the hike starts."),
            stops: vec![
                Stop { id:"4a", time:"07:30", tag:Tag::Drive, tag_text:"Drive ~10 min",       dot:Dot::Gold,
                       name:"🚐 Drive to Lost Valley car park",
                       note:"NTS Glencoe car park on A82. Arrive early — fills fast. Toilets here." },
                Stop { id:"4b", time:"07:45", tag:Tag::Hike,  tag_text:"Main Hike · ~3h45",   dot:Dot::Gold,
                       name:"🥾 Lost Valley (Coire Gabhail)",
                       note:"Up past waterfalls into a hidden valley. Rocky + slippy — proper hiking shoes essential. ~1h45 up, ~2h down." },
                Stop { id:"4c", time:"11:30", tag:Tag::Food,  tag_text:"Quick lunch · 45 min", dot:Dot::Gold,
                       name:"🍺 Clachaig Inn",
                       note:"Famous hikers' pub. Quick lunch only — 12:30 departure is firm." },
                Stop { id:"4d", time:"12:30", tag:Tag::Warn,  tag_text:"Non-negotiable 🚨",    dot:Dot::Warn,
                       name:"🚐 Depart Glencoe",
                       note:"~3.5 hrs via A82 → A85 → M8 → M9. One comfort stop only." },
                Stop { id:"4e", time:"16:00", tag:Tag::Drive, tag_text:"Airport",              dot:Dot::Gold,
                       name:"🏙 Arrive Edinburgh",
                       note:"Return minibus. ~4 hrs before flight — check-in, security, airside dinner." },
                Stop { id:"4f", time:"20:10", tag:Tag::Fly,   tag_text:"Outbound · RK 8604",   dot:Dot::Moss,
                       name:"✈ Fly Home",
                       note:"Job done. Scotland delivered. 🏴󠁧󠁢󠁳󠁣󠁴󠁿" },
            ],
        },
    ]
}

// ─────────────────────────────────────────────
// EXTRA PANELS — data + renderers
// ─────────────────────────────────────────────

struct TipRow { icon: &'static str, html: &'static str }
struct CheckItem { text: &'static str, critical: bool }
struct CheckSection { title: &'static str, items: Vec<CheckItem> }

fn tip(icon: &'static str, html: &'static str) -> TipRow { TipRow { icon, html } }
fn check(text: &'static str) -> CheckItem { CheckItem { text, critical: false } }
fn check_crit(text: &'static str) -> CheckItem { CheckItem { text, critical: true } }

fn render_tip_row(t: &TipRow) -> Markup {
    html! {
        div.tip-row {
            div.tip-icon { (t.icon) }
            div contenteditable="true" { (PreEscaped(t.html)) }
        }
    }
}

fn render_alerts() -> Markup {
    let alerts = vec![
        tip("🕘",  "<strong>Day 1 — Urquhart Castle:</strong> Go inside today — you have time. Buy tickets online (~£12) to skip the queue."),
        tip("🅿️", "<strong>Day 2 — Storr parking:</strong> 07:00 departure is firm. Car park fills by 9am on good days — a slow breakfast costs you the spot."),
        tip("🚐",  "<strong>Day 3 — Single-track roads:</strong> Glen Etive and Talisker Bay are narrow. A minibus needs more space for passing places — go slow."),
        tip("⏱",  "<strong>Day 3 — Glen Etive is a dead end:</strong> Drive in and back out the same way. Budget the full 45 min."),
        tip("🚨",  "<strong>Day 4 — 12:30 is non-negotiable.</strong> Set a group alarm for 12:00. Agree a hike turnaround time before you start."),
        tip("🌧",  "<strong>Bad weather cuts:</strong> Talisker Bay (Day 3, saves ~1h) → Fairy Glen (Day 2, ~40 min). Never cut: Storr, Glenfinnan, Lost Valley."),
        tip("🚂",  "<strong>Jacobite Train:</strong> Confirm on West Coast Railways — runs from mid-May. Return crossing ≈ 15:10."),
        tip("🅿️", "<strong>Parking:</strong> Storr (~£5) · Kilt Rock (~£3) · Fairy Glen (~£3) · Glenfinnan (~£3.50). Mostly card, carry some cash."),
    ];
    let pubs = vec![
        tip("🏴󠁧󠁢󠁳󠁣󠁴󠁿", "<strong>Clachaig Inn</strong> — Glencoe. Legendary hikers' pub. Real ales + fireplace. Day 3 evening or Day 4 lunch."),
        tip("🏔",  "<strong>Kingshouse Hotel</strong> — Glencoe. Remote and iconic. Worth a drink even if not staying."),
        tip("🌊",  "<strong>Sligachan Hotel pub</strong> — Day 3 morning coffee stop after the bridge."),
    ];
    html! {
        div.panel id="panel-5" {
            div.emoji-section-title { "⚠️ Schedule Watch Points" }
            @for t in &alerts { (render_tip_row(t)) }
            div.emoji-section-title style="padding-top:28px" { "🍺 Best Pubs" }
            @for t in &pubs { (render_tip_row(t)) }
        }
    }
}

fn render_gear() -> Markup {
    let sections = vec![
        CheckSection { title: "👟 Footwear (critical)", items: vec![
            check_crit("Proper hiking shoes/boots with ankle support — non-negotiable for Lost Valley"),
            check("Trainers fine for non-hiking days"),
            check("Waterproof hiking boots preferred — paths are wet and rocky"),
        ]},
        CheckSection { title: "🧥 Clothing layers", items: vec![
            check("Base layer (moisture-wicking)"),
            check("Mid-layer fleece or thick jumper"),
            check_crit("Genuinely waterproof shell jacket — not just shower-resistant"),
            check("T-shirts and long sleeves"),
            check("Beanie hat + thin gloves (cold at summits even in May)"),
        ]},
        CheckSection { title: "🦟 Midge protection", items: vec![
            check_crit("Smidge or Avon Skin So Soft — most recommended in Scotland"),
            check("DEET-based repellent as backup"),
            check("Midge head net — worst at Sligachan & Talisker Bay at dawn"),
            check("Apply before leaving the minibus, not once surrounded"),
        ]},
        CheckSection { title: "🎒 Day pack (group)", items: vec![
            check("1.5L water per person on hike days"),
            check("High-energy snacks (no café at Lost Valley summit)"),
            check("Basic group first aid kit"),
            check_crit("OS Maps or AllTrails downloaded OFFLINE — no signal in Glencoe"),
            check("Spare dry layer in a dry bag"),
            check("Trekking poles (optional, useful on Lost Valley descent)"),
            check("Portable phone charger / power bank"),
        ]},
        CheckSection { title: "🚐 Minibus", items: vec![
            check("Cash (parking machines, rural cafés)"),
            check("Bin bags for muddy kit"),
            check("Snacks for long drive days"),
            check("Google Maps downloaded offline for all routes"),
        ]},
    ];
    html! {
        div.panel id="panel-6" {
            div.emoji-section-title { "🎒 What to Pack" }
            div.info-cards {
                div.info-card { div.info-card-title{"Main Hikes"} div.info-card-val{"Day 2 + Day 4"} }
                div.info-card { div.info-card-title{"Midge Risk"} div.info-card-val{"Low–Med (early May)"} }
                div.info-card { div.info-card-title{"Daylight"}   div.info-card-val{"Until ~21:30–22:00"} }
                div.info-card { div.info-card-title{"Avg Temp"}   div.info-card-val{"10–16°C / 5–8°C eve"} }
            }
            div.check-section {
                @for section in &sections {
                    div.check-title { (section.title) }
                    ul.checklist {
                        @for item in &section.items {
                            @if item.critical {
                                li.crit { (item.text) }
                            } @else {
                                li { (item.text) }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_tips() -> Markup {
    let tips = vec![
        tip("📸", "<strong>Eilean Donan best shot:</strong> Stop at the lay-by ~200m east of the entrance on A87, not the car park. Castle + bridge + loch in one frame. Soft morning light at 07:45."),
        tip("⬅️", "<strong>Old Man of Storr:</strong> Take the left fork — 10 min longer but views reveal gradually. Walk all the way around the pinnacles; they look completely different from behind."),
        tip("🚂", "<strong>Glenfinnan viewpoint:</strong> Go left out of the car park, follow the signs uphill. Stand above and south of the viaduct — not underneath it."),
        tip("🌊", "<strong>Talisker Bay:</strong> Walk all the way to the far end. The waterfall dropping into the sea is the real highlight — most people stop halfway and miss it."),
        tip("🗿", "<strong>Sligachan Bridge legend:</strong> Dip your face in the river under the old stone bridge. 7 years of youth. The group reaction is worth it 😄"),
        tip("⛰",  "<strong>Buachaille shot:</strong> Lay-by at the A82 / Glen Etive junction. On calm days the mountain reflects in the River Coupall."),
        tip("🌅", "<strong>Daylight:</strong> Light until 21:30–22:00. Don't stress — golden hour on Day 3 at Buachaille will be spectacular."),
        tip("🦟", "<strong>Midges:</strong> Apply repellent before leaving the minibus, not once surrounded. Worst near still water at dawn/dusk."),
        tip("🥾", "<strong>Lost Valley:</strong> The rocky entrance squeeze is the hardest bit. The valley is 10 min beyond — worth pushing through."),
        tip("🐑", "<strong>Sheep on the road</strong> — they don't move for minibuses. Patience required."),
        tip("📶", "<strong>Phone signal:</strong> Very patchy in Glencoe and Glen Etive. Download all maps offline before arriving."),
    ];
    html! {
        div.panel id="panel-7" {
            div.emoji-section-title { "✨ Insider Tips" }
            @for t in &tips { (render_tip_row(t)) }
        }
    }
}

// ─────────────────────────────────────────────
// HTML COMPONENTS
// ─────────────────────────────────────────────

fn render_stop(s: &Stop, day_id: u8) -> Markup {
    let dot_class = match s.dot {
        Dot::Gold => "stop-dot",
        Dot::Warn => "stop-dot warn",
        Dot::Moss => "stop-dot moss",
    };
    let card_class = match s.tag {
        Tag::Opt => "stop-card opt",
        _        => "stop-card",
    };
    let up = |field: &str| format!("upStop({},\"{}\",\"{}\",this.innerText)", day_id, s.id, field);

    html! {
        div.stop id=(s.id) {
            span.stop-time contenteditable="true" onblur=(up("t")) { (s.time) }
            div class=(dot_class) {}
            div class=(card_class) {
                button.del-btn onclick=(format!("delStop({},\"{}\")", day_id, s.id)) { "×" }
                div style="margin-bottom:5px" {
                    span class=(format!("tag {}", s.tag.css())) {
                        span contenteditable="true" onblur=(up("tl")) { (s.tag_text) }
                    }
                }
                div.stop-name contenteditable="true" onblur=(up("nm")) { (s.name) }
                " "
                div.stop-note contenteditable="true" onblur=(up("nt")) { (s.note) }
            }
        }
    }
}

fn render_day(day: &Day, active: bool) -> Markup {
    let panel_class = if active { "panel active" } else { "panel" };
    html! {
        div class=(panel_class) id=(format!("panel-{}", day.id)) {
            div.day-header {
                div.day-badge { (day.id) }
                div style="flex:1;min-width:0" {
                    div.day-meta-date { (day.date) }
                    div.day-title-ef.serif contenteditable="true"
                        onblur=(format!("upDay({},\"title\",this.innerText)", day.id))
                    { (day.title) }
                }
            }
            @if let Some(alert) = day.alert {
                div.alert-box {
                    div.alert-label { "⚠ Heads up" }
                    div.alert-body contenteditable="true"
                        onblur=(format!("upDay({},\"alert\",this.innerText)", day.id))
                    { (alert) }
                }
            }
            div.timeline {
                @for stop in &day.stops { (render_stop(stop, day.id)) }
                div.add-stop {
                    div style="width:48px" {}
                    div style="width:10px" {}
                    button.add-stop-btn onclick=(format!("addStop({})", day.id)) { "＋ Add stop" }
                }
            }
        }
    }
}

fn render_tabs(days: &[Day]) -> Markup {
    let extra = [("⚠️ Alerts", 5), ("🎒 Gear", 6), ("✨ Tips", 7)];
    html! {
        div.tabs id="tabBar" {
            @for (i, day) in days.iter().enumerate() {
                button class=(if i == 0 { "tab active" } else { "tab" })
                    onclick=(format!("switchTab({})", i))
                { (day.tab) }
            }
            @for (label, i) in &extra {
                button class="tab" onclick=(format!("switchTab({})", i)) { (label) }
            }
        }
    }
}

// ─────────────────────────────────────────────
// CSS — fixed time column + proper line alignment
// ─────────────────────────────────────────────

fn css() -> &'static str { r#"
*{box-sizing:border-box;margin:0;padding:0}
body{font-family:'DM Sans',system-ui,sans-serif;font-weight:300;background:#f5f0e8;color:#1a1a2e;max-width:680px;margin:0 auto;min-height:100vh}
:root{
  --ink:#1a1a2e;--gold:#c9a84c;--moss:#3d6b4f;--warn:#c0392b;
  --loch:#2c5f7a;--heather:#6b4e71;--paper:#f5f0e8;--mist:#e8e2d5;
  --time-w:48px;  /* ← fixed: was 28px, too narrow for HH:MM */
  --tl-pad:22px;
  --stop-gap:14px;
  --dot-w:10px;
}
.serif{font-family:'Playfair Display',Georgia,serif}
.topbar{background:var(--ink);padding:8px 16px;display:flex;justify-content:space-between;align-items:center;position:sticky;top:0;z-index:50}
.topbar-left{font-size:11px;letter-spacing:2px;text-transform:uppercase;color:rgba(245,240,232,.4)}
.topbar-right{display:flex;align-items:center;gap:7px}
.live-dot{width:6px;height:6px;border-radius:50%;background:var(--moss);animation:pulse 2s infinite}
@keyframes pulse{0%,100%{opacity:1}50%{opacity:.35}}
.sync-txt{font-size:11px;color:rgba(245,240,232,.6);letter-spacing:.3px}
.share-banner{background:#1e3a5f;color:white;padding:10px 16px;display:flex;align-items:flex-start;gap:10px;font-size:12px;line-height:1.5}
.share-dismiss{margin-left:auto;background:rgba(255,255,255,.15);border:none;color:white;border-radius:4px;padding:2px 8px;font-size:11px;cursor:pointer;font-family:inherit}
.hero{background:var(--ink);padding:26px 22px 22px;position:relative;overflow:hidden}
.hero::before{content:'';position:absolute;inset:0;background:radial-gradient(ellipse at 80% 20%,rgba(107,78,113,.4) 0%,transparent 60%),radial-gradient(ellipse at 20% 80%,rgba(44,95,122,.35) 0%,transparent 60%);pointer-events:none}
.hero-inner{position:relative;z-index:1}
.trip-label{font-size:10px;letter-spacing:3px;text-transform:uppercase;color:var(--gold);font-weight:500;margin-bottom:6px}
.hero h1{font-size:30px;color:#f5f0e8;line-height:1.15;margin-bottom:14px}
.hero h1 em{color:var(--gold);font-style:italic}
.pills{display:flex;gap:7px;flex-wrap:wrap}
.pill{background:rgba(255,255,255,.1);border:1px solid rgba(255,255,255,.15);border-radius:20px;padding:4px 12px;font-size:12px;color:#f5f0e8}
.flights{background:var(--loch);color:white;padding:13px 22px;display:flex;gap:16px}
.fl{flex:1}
.fl-label{font-size:9px;letter-spacing:2px;text-transform:uppercase;opacity:.6;margin-bottom:2px}
.fl-num{font-size:18px;font-weight:700;letter-spacing:1px}
.fl-detail{font-size:12px;opacity:.75}
.fl-div{width:1px;background:rgba(255,255,255,.2)}
.tabs{display:flex;overflow-x:auto;background:var(--mist);border-bottom:2px solid rgba(0,0,0,.08);scrollbar-width:none;position:sticky;top:37px;z-index:10}
.tabs::-webkit-scrollbar{display:none}
.tab{flex-shrink:0;padding:11px 16px;font-size:11px;font-weight:500;letter-spacing:.5px;text-transform:uppercase;color:rgba(26,26,46,.45);background:none;border:none;border-bottom:2px solid transparent;margin-bottom:-2px;cursor:pointer;white-space:nowrap;font-family:inherit;transition:color .15s}
.tab.active{color:var(--ink);border-bottom-color:var(--gold)}
.day-header{padding:20px 22px 0;display:flex;gap:14px;align-items:flex-start}
.day-badge{background:var(--ink);color:var(--gold);font-size:24px;font-weight:700;width:50px;height:50px;display:flex;align-items:center;justify-content:center;border-radius:4px;flex-shrink:0}
.day-meta-date{font-size:10px;letter-spacing:2px;text-transform:uppercase;color:var(--heather);font-weight:500;margin-bottom:2px}
.day-title-ef{font-size:18px;line-height:1.25;color:var(--ink)}
.alert-box{margin:14px 22px 0;border-left:3px solid var(--warn);background:#fdf3f3;border-radius:0 8px 8px 0;padding:10px 13px}
.alert-label{font-size:10px;letter-spacing:1.5px;text-transform:uppercase;color:var(--warn);font-weight:600;margin-bottom:2px}
.alert-body{font-size:13px;color:var(--ink)}

/* ── TIMELINE: aligned with fixed time column ── */
.timeline{
  padding:16px var(--tl-pad) 0;
  position:relative;
}
.timeline::before{
  content:'';
  position:absolute;
  /* center on the dot:  tl-pad  + time-w  + gap      + ½dot  */
  left: calc(var(--tl-pad) + var(--time-w) + var(--stop-gap) + calc(var(--dot-w) / 2));
  top:16px;
  bottom:0;
  width:1px;
  background:linear-gradient(to bottom,var(--gold),transparent);
}

/* ── STOP ROW ── */
.stop{display:flex;gap:var(--stop-gap);align-items:flex-start;margin-bottom:18px}

/* fixed-width, tabular digits so HH:MM never overflows */
.stop-time{
  font-size:11px;
  font-weight:500;
  color:var(--heather);
  width:var(--time-w);   /* ← 48px, was 28px */
  flex-shrink:0;
  padding-top:5px;
  text-align:right;
  letter-spacing:.3px;
  font-variant-numeric:tabular-nums;
  white-space:nowrap;
}
.stop-dot{width:var(--dot-w);height:var(--dot-w);border-radius:50%;background:var(--gold);flex-shrink:0;margin-top:6px;position:relative;z-index:1;transition:box-shadow .15s}
.stop:hover .stop-dot{box-shadow:0 0 0 3px rgba(201,168,76,.2)}
.stop-dot.warn{background:var(--warn)}
.stop-dot.moss{background:var(--moss)}
.stop-card{flex:1;background:white;border-radius:8px;padding:11px 13px;box-shadow:0 1px 4px rgba(0,0,0,.07);position:relative;transition:box-shadow .15s;min-width:0}
.stop:hover .stop-card{box-shadow:0 3px 14px rgba(201,168,76,.15)}
.stop-card.opt{border-left:3px solid var(--moss)}
.tag{display:inline-block;font-size:9px;letter-spacing:1.5px;text-transform:uppercase;padding:2px 8px;border-radius:10px;font-weight:500;margin-bottom:5px;cursor:text}
/* iOS fix 1 & 2: webkit prefix + block display required for iPhone keyboard */
[contenteditable]{
  border-bottom:1.5px dashed transparent;border-radius:2px;
  transition:border-color .15s;cursor:text;outline:none;word-break:break-word;
  display:block;
  -webkit-user-select:text;user-select:text;
  -webkit-user-modify:read-write-plaintext-only;
  min-height:1.2em;
  -webkit-tap-highlight-color:rgba(201,168,76,.15);
}
[contenteditable]:hover{border-bottom-color:rgba(201,168,76,.45)}
[contenteditable]:focus{border-bottom-color:var(--gold);background:rgba(201,168,76,.06)}
.stop-time[contenteditable]{display:inline-block}
.stop-name{font-weight:500;font-size:14px;color:var(--ink);display:block;margin-bottom:3px;width:100%}
.stop-note{font-size:13px;color:rgba(26,26,46,.62);line-height:1.5;display:block;width:100%}
.del-btn{display:none;position:absolute;top:8px;right:8px;width:20px;height:20px;border-radius:50%;background:#fae0e0;border:none;color:var(--warn);font-size:14px;cursor:pointer;align-items:center;justify-content:center;z-index:2;font-family:inherit}
.stop:hover .del-btn{display:flex}
.add-stop{display:flex;gap:var(--stop-gap);padding-bottom:40px;margin-top:4px}
.add-stop-btn{flex:1;padding:8px 16px;font-size:12px;font-weight:500;color:var(--heather);background:white;border:1.5px dashed rgba(107,78,113,.3);border-radius:20px;cursor:pointer;font-family:inherit;text-align:left;transition:all .15s}
.add-stop-btn:hover{border-color:var(--heather);background:#f8f4fb}
.t-drive{background:#deeef5;color:var(--loch)}
.t-hike{background:#dff0e4;color:var(--moss)}
.t-food{background:#fdf4e2;color:#7a5c10}
.t-castle{background:#ede5f3;color:var(--heather)}
.t-view{background:#fef8ed;color:#7a5c10}
.t-stay{background:var(--ink);color:var(--gold)}
.t-fly{background:var(--loch);color:#fff}
.t-warn{background:#fae0e0;color:var(--warn)}
.t-opt{background:#e4f2e8;color:var(--moss)}
.panel{display:none}
.panel.active{display:block}
.push-btn{font-size:11px;font-weight:500;padding:4px 12px;border-radius:12px;border:1px solid var(--gold);color:var(--gold);background:transparent;cursor:pointer;font-family:inherit;letter-spacing:.3px;transition:all .15s;white-space:nowrap}
.push-btn:hover{background:var(--gold);color:var(--ink)}
#ghModal{background:white;border-radius:14px;padding:24px;width:340px;max-width:92vw;box-shadow:0 20px 60px rgba(0,0,0,.3)}
.gh-title{font-family:'Playfair Display',Georgia,serif;font-size:18px;color:#1a1a2e;margin-bottom:6px}
.gh-sub{font-size:12px;color:#666;line-height:1.5;margin-bottom:18px}
.gh-label{display:block;font-size:11px;font-weight:500;letter-spacing:.5px;text-transform:uppercase;color:#6b4e71;margin-bottom:4px;margin-top:12px}
.gh-link{font-size:11px;color:#2c5f7a;text-decoration:none;float:right;font-weight:400;text-transform:none;letter-spacing:0}
.gh-input{display:block;width:100%;padding:8px 10px;border:1px solid #ddd;border-radius:6px;font-size:13px;font-family:inherit;outline:none;transition:border-color .15s}
.gh-input:focus{border-color:#c9a84c}
.gh-note{font-size:11px;color:#999;margin-top:14px;line-height:1.5;padding:8px;background:#fafafa;border-radius:6px}
.gh-actions{display:flex;gap:10px;margin-top:18px}
.gh-cancel{flex:1;padding:8px;border:1px solid #ddd;border-radius:7px;background:none;cursor:pointer;font-family:inherit;font-size:13px;color:#666}
.gh-push{flex:2;padding:8px;border:none;border-radius:7px;background:#1a1a2e;color:#c9a84c;cursor:pointer;font-family:inherit;font-size:13px;font-weight:500}
.gh-push:hover{background:#2c3060}.gh-push:disabled{opacity:.5;cursor:not-allowed}
.gh-status{font-size:12px;margin-top:10px;min-height:18px;text-align:center;line-height:1.4}
.emoji-section-title{font-family:'Playfair Display',Georgia,serif;font-size:18px;padding:20px 22px 8px}
.tip-row{display:flex;align-items:flex-start;gap:12px;padding:10px 22px;border-bottom:1px solid var(--mist);font-size:13px;line-height:1.5}
.tip-icon{font-size:16px;flex-shrink:0;width:28px;text-align:center;padding-top:1px}
.info-cards{display:grid;grid-template-columns:1fr 1fr;gap:12px;padding:14px 22px 8px}
.info-card{background:white;border-radius:8px;padding:12px;box-shadow:0 1px 4px rgba(0,0,0,.07)}
.info-card-title{font-size:9px;letter-spacing:2px;text-transform:uppercase;color:var(--heather);font-weight:500;margin-bottom:4px}
.info-card-val{font-size:13px;font-weight:500;color:var(--ink)}
.check-section{padding:0 22px 16px}
.check-title{font-size:15px;font-weight:500;color:var(--ink);margin:16px 0 8px;padding-top:8px;border-top:1px solid var(--mist)}
.checklist{list-style:none;display:flex;flex-direction:column;gap:6px}
.checklist li{display:flex;align-items:flex-start;gap:10px;font-size:13px;line-height:1.4;color:#2a2a3a}
.checklist li::before{content:'☐';font-size:16px;color:var(--gold);flex-shrink:0;line-height:1.2}
.checklist li.crit::before{content:'⚠';color:var(--warn)}
.checklist li.crit{font-weight:500;color:var(--warn)}
"# }

// ─────────────────────────────────────────────
// JAVASCRIPT (interaction layer)
// ─────────────────────────────────────────────

fn js() -> &'static str { r#"
const SK = 'scot_offline_v2';
let activeTab = 0;

// Persist edits to localStorage
// ── iOS fix 3: blur fires unreliably on iPhone — also save on input ──
document.addEventListener('input', e => {
  if (e.target.isContentEditable) save();
});

// ── iOS fix 4: tap-to-focus — Safari needs explicit focus() on tap ──
document.addEventListener('touchend', e => {
  const el = e.target.closest('[contenteditable="true"]');
  if (el && document.activeElement !== el) {
    e.preventDefault();
    el.focus();
    const range = document.createRange();
    const sel = window.getSelection();
    range.selectNodeContents(el);
    range.collapse(false);
    sel.removeAllRanges();
    sel.addRange(range);
  }
}, { passive: false });

function upStop(dayId, stopId, field, val) { save(); }
function upDay(dayId, field, val)          { save(); }

function save() {
  try {
    // Snapshot the entire panels innerHTML so edits survive tab switches
    const snap = {};
    document.querySelectorAll('.panel').forEach(p => { snap[p.id] = p.innerHTML; });
    localStorage.setItem(SK, JSON.stringify({ snap, tab: activeTab }));
  } catch(e) {}
}

function load() {
  try {
    const raw = localStorage.getItem(SK);
    if (!raw) return;
    const { snap, tab } = JSON.parse(raw);
    Object.entries(snap).forEach(([id, html]) => {
      const el = document.getElementById(id);
      if (el) el.innerHTML = html;
    });
    switchTab(tab || 0, false);
  } catch(e) {}
}

function switchTab(i, doScroll = true) {
  activeTab = i;
  document.querySelectorAll('.panel').forEach((p, idx) => p.classList.toggle('active', idx === i));
  document.querySelectorAll('.tab').forEach((b, idx)  => b.classList.toggle('active', idx === i));
  if (doScroll) window.scrollTo({ top: 0, behavior: 'smooth' });
}

function addStop(dayId) {
  const id = 'new-' + Math.random().toString(36).slice(2, 7);
  const html = `
    <div class="stop" id="${id}">
      <span class="stop-time" contenteditable="true" onblur="upStop(${dayId},'${id}','t',this.innerText)">00:00</span>
      <div class="stop-dot"></div>
      <div class="stop-card">
        <button class="del-btn" onclick="delStop(${dayId},'${id}')" style="display:flex">×</button>
        <div style="margin-bottom:5px">
          <span class="tag t-view"><span contenteditable="true" onblur="upStop(${dayId},'${id}','tl',this.innerText)">New stop</span></span>
        </div>
        <div class="stop-name" contenteditable="true" onblur="upStop(${dayId},'${id}','nm',this.innerText)">Stop name</div>
        <div class="stop-note" contenteditable="true" onblur="upStop(${dayId},'${id}','nt',this.innerText)">Add notes…</div>
      </div>
    </div>`;
  const btn = document.querySelector(`#panel-${dayId} .add-stop`);
  btn.insertAdjacentHTML('beforebegin', html);
  document.querySelector(`#${id} .stop-name`)?.focus();
  save();
}

function delStop(dayId, stopId) {
  if (!confirm('Remove this stop?')) return;
  document.getElementById(stopId)?.remove();
  save();
}

// ── GitHub push ──────────────────────────────────────────────
function openPushModal() {
  const saved = JSON.parse(sessionStorage.getItem('scot_gh_settings') || '{}');
  if (saved.repo)   document.getElementById('ghRepo').value   = saved.repo;
  if (saved.branch) document.getElementById('ghBranch').value = saved.branch;
  document.getElementById('ghOverlay').style.display = 'flex';
  document.getElementById('ghStatus').textContent = '';
}

function closePushModal() {
  document.getElementById('ghOverlay').style.display = 'none';
}

document.addEventListener('DOMContentLoaded', () => {
  load();
  document.getElementById('ghOverlay').addEventListener('click', e => {
    if (e.target === document.getElementById('ghOverlay')) closePushModal();
  });
});

async function doPush() {
  const repo   = document.getElementById('ghRepo').value.trim();
  const branch = document.getElementById('ghBranch').value.trim() || 'trip-edits';
  const token  = document.getElementById('ghToken').value.trim();
  const status = document.getElementById('ghStatus');
  const btn    = document.getElementById('ghPushBtn');

  if (!repo || !token) {
    status.style.color = '#c0392b';
    status.textContent = 'Please fill in all fields.';
    return;
  }

  sessionStorage.setItem('scot_gh_settings', JSON.stringify({ repo, branch }));
  btn.disabled = true;
  status.style.color = '#666';

  const apiBase = `https://api.github.com/repos/${repo}/contents/index.html`;
  const headers = { Authorization: `token ${token}`, 'Content-Type': 'application/json' };

  try {
    status.textContent = 'Fetching current file SHA…';
    const getRes = await fetch(`${apiBase}?ref=${branch}`, { headers });
    let sha;
    if (getRes.ok) {
      sha = (await getRes.json()).sha;
    } else if (getRes.status === 404) {
      sha = undefined;
    } else if (getRes.status === 401) {
      throw new Error('Token invalid or expired. Ensure it has Contents (write) scope.');
    } else {
      throw new Error(`GitHub returned ${getRes.status}. Check repo name and token.`);
    }

    status.textContent = 'Encoding page…';
    const html = '<!DOCTYPE html>\n' + document.documentElement.outerHTML;
    const bytes = new TextEncoder().encode(html);
    let binary = '';
    bytes.forEach(b => binary += String.fromCharCode(b));
    const content = btoa(binary);

    status.textContent = `Pushing to ${branch}…`;
    const body = { message: `Update itinerary via web editor`, content, branch };
    if (sha) body.sha = sha;

    const putRes = await fetch(apiBase, { method: 'PUT', headers, body: JSON.stringify(body) });

    if (putRes.ok) {
      status.style.color = '#3d6b4f';
      status.textContent = `✅ Pushed to ${branch}! Organiser can squash-merge after the trip.`;
    } else {
      const err = await putRes.json();
      throw new Error(err.message || `Push failed (${putRes.status})`);
    }
  } catch(e) {
    status.style.color = '#c0392b';
    status.textContent = '❌ ' + e.message;
  }
  btn.disabled = false;
}
"# }

// ─────────────────────────────────────────────
// FULL DOCUMENT
// ─────────────────────────────────────────────

fn render_document(days: &[Day]) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width,initial-scale=1";
                title { "Scotland Highlands — Group Itinerary" }
                link rel="stylesheet"
                     href="https://fonts.googleapis.com/css2?family=Playfair+Display:ital,wght@0,700;1,400&family=DM+Sans:wght@300;400;500&display=swap";
                style { (PreEscaped(css())) }
            }
            body {

                // ── Top bar ──
                div.topbar {
                    div.topbar-left { "Scotland · May 2025" }
                    div.topbar-right {
                        span.live-dot id="lDot" {}
                        span.sync-txt id="syncTxt" { "Offline · edits saved locally" }
                        button.push-btn id="pushBtn" onclick="openPushModal()" { "⬆ Push to GitHub" }
                    }
                }

                // ── GitHub push modal ──
                div id="ghOverlay" style="display:none;position:fixed;inset:0;background:rgba(0,0,0,.55);z-index:300;align-items:center;justify-content:center;" {
                    div id="ghModal" {
                        div.gh-title { "Push changes to GitHub" }
                        div.gh-sub { "Commits your edits to the " code{"trip-edits"} " branch. Your organiser squash-merges after the trip." }
                        label.gh-label { "Repository " span style="color:#aaa;font-weight:300"{"(owner/repo)"} }
                        input id="ghRepo" class="gh-input" placeholder="e.g. yourname/scotland-trip-2025" autocomplete="off" {}
                        label.gh-label { "Branch" }
                        input id="ghBranch" class="gh-input" value="trip-edits" autocomplete="off" {}
                        label.gh-label {
                            "Personal Access Token "
                            a href="https://github.com/settings/tokens/new?scopes=contents&description=Scotland+trip"
                              target="_blank" class="gh-link" { "generate one ↗" }
                        }
                        input id="ghToken" class="gh-input" type="password" placeholder="ghp_xxxxxxxxxxxx" autocomplete="off" {}
                        div.gh-note { "⚠ Token is stored in this browser session only — never saved to disk or sent anywhere except GitHub's API." }
                        div.gh-actions {
                            button.gh-cancel onclick="closePushModal()" { "Cancel" }
                            button.gh-push id="ghPushBtn" onclick="doPush()" { "Push changes ⬆" }
                        }
                        div id="ghStatus" class="gh-status" {}
                    }
                }

                // ── Share banner ──
                div.share-banner id="shareBanner" {
                    div style="font-size:16px;flex-shrink:0" { "💬" }
                    div {
                        strong { "Group editing: " }
                        "Share via Google Drive or WhatsApp. Everyone edits their own local copy — changes save automatically to their device."
                    }
                    button.share-dismiss onclick="document.getElementById('shareBanner').style.display='none'" {
                        "Got it"
                    }
                }

                // ── Hero ──
                div.hero {
                    div.hero-inner {
                        div.trip-label { "Group Trip · 7–8 People" }
                        h1.serif { "Scottish " em { "Highlands" } }
                        div.pills {
                            span.pill { "30 Apr – 4 May 2025" }
                            span.pill { "🚐 1× Minibus" }
                            span.pill { "🏕 Lochcarron + Glencoe" }
                        }
                    }
                }

                // ── Flights ──
                div.flights {
                    div.fl {
                        div.fl-label { "Inbound" }
                        div.fl-num.serif { "RK 8605" }
                        div.fl-detail { "✈ Arrives Edinburgh · 30 Apr · 21:00" }
                    }
                    div.fl-div {}
                    div.fl {
                        div.fl-label { "Outbound" }
                        div.fl-num.serif { "RK 8604" }
                        div.fl-detail { "✈ Departs Edinburgh · 4 May · 20:10" }
                    }
                }

                // ── Tab navigation ──
                (render_tabs(days))

                // ── Day panels ──
                div id="panels" {
                    @for (i, day) in days.iter().enumerate() {
                        (render_day(day, i == 0))
                    }
                    (render_alerts())
                    (render_gear())
                    (render_tips())
                }

                script { (PreEscaped(js())) }
            }
        }
    }
}

// ─────────────────────────────────────────────
// ENTRY POINT
// ─────────────────────────────────────────────

fn main() {
    let days = itinerary();
    let doc  = render_document(&days);
    let path = "index.html";
    fs::write(path, doc.into_string()).expect("Could not write index.html");
    println!("✅  Generated {path}  ({} days + Alerts/Gear/Tips, {} total stops)",
        days.len(),
        days.iter().map(|d| d.stops.len()).sum::<usize>());
}
