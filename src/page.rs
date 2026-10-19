use crate::stopwatch::Stopwatch;
use std::collections::HashMap;

/// Renders the whole page. No JavaScript: actions are plain HTML forms, the
/// "new timer" modal opens with CSS `:target`, and running timers tick through
/// an animated CSS counter.
pub fn render(stopwatches: &HashMap<String, Stopwatch>) -> String {
    let mut names: Vec<&String> = stopwatches.keys().collect();
    names.sort();

    let mut rows = String::new();
    for name in &names {
        rows.push_str(&render_row(name, &stopwatches[*name]));
    }
    if names.is_empty() {
        rows.push_str(r##"<tr><td colspan="4" class="empty">No timers yet. Create one with “New timer”.</td></tr>"##);
    }

    let total_secs: u64 = stopwatches.values().map(|sw| sw.elapsed().as_secs()).sum();
    let any_running = stopwatches.values().any(Stopwatch::is_running);

    let mut html = String::new();
    html.push_str(r##"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Stopwatch</title>
<style>"##);
    html.push_str(STYLE);
    html.push_str(r##"</style>
</head>
<body>
<main>
<header>
  <h1>Stopwatch</h1>
  <a class="btn primary" href="#new">+ New timer</a>
</header>
<table>
<thead><tr><th>Name</th><th>Status</th><th class="time">Elapsed</th><th class="actions">Actions</th></tr></thead>
<tbody>
"##);
    html.push_str(&rows);
    html.push_str("</tbody>\n");
    if !names.is_empty() {
        // The total only ticks when exactly one timer runs; otherwise it is a snapshot.
        let running_count = stopwatches.values().filter(|sw| sw.is_running()).count();
        html.push_str(&format!(
            r##"<tfoot><tr><td colspan="2">Total{}</td><td class="time">{}</td><td></td></tr></tfoot>"##,
            if any_running && running_count > 1 { " <small>(refresh to update)</small>" } else { "" },
            clock(total_secs, running_count == 1),
        ));
    }
    html.push_str(r##"
</table>
</main>

<div id="new" class="modal">
  <a class="backdrop" href="#" aria-label="Close"></a>
  <form class="dialog" method="post" action="/timers">
    <h2>New timer</h2>
    <label for="name">Name</label>
    <input id="name" name="name" required maxlength="100" autocomplete="off" placeholder="e.g. client-project">
    <div class="buttons">
      <a class="btn" href="#">Cancel</a>
      <button class="btn primary" type="submit">Create</button>
    </div>
  </form>
</div>
</body>
</html>
"##);
    html
}

fn render_row(name: &str, sw: &Stopwatch) -> String {
    let name = escape(name);
    let running = sw.is_running();
    let (status, toggle) = if running {
        (r##"<span class="badge running">Running</span>"##, action_button(&name, "pause", "Pause", ""))
    } else {
        (r##"<span class="badge paused">Paused</span>"##, action_button(&name, "start", "Start", "primary"))
    };

    format!(
        r##"<tr>
  <td class="name">{name}</td>
  <td>{status}</td>
  <td class="time">{clock}</td>
  <td class="actions">{toggle}{stop}{reset}
    <details class="delete"><summary class="btn danger">Delete</summary>{confirm}</details>
  </td>
</tr>
"##,
        clock = clock(sw.elapsed().as_secs(), running),
        stop = action_button(&name, "stop", "Stop", ""),
        reset = action_button(&name, "reset", "Reset", ""),
        confirm = action_button(&name, "delete", "Confirm", "danger"),
    )
}

fn action_button(escaped_name: &str, action: &str, label: &str, class: &str) -> String {
    format!(
        r##"<form method="post" action="/timers/action"><input type="hidden" name="name" value="{escaped_name}"><button class="btn {class}" name="action" value="{action}">{label}</button></form>"##
    )
}

/// The text is the server-side value; when `live`, CSS replaces it with a
/// counter that keeps counting up from `--start`.
fn clock(secs: u64, live: bool) -> String {
    let class = if live { "clock live" } else { "clock" };
    format!(
        r##"<span class="{class}" style="--start: {secs}"><span class="static">{}</span></span>"##,
        format_duration(secs)
    )
}

fn format_duration(total_seconds: u64) -> String {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    format!("{hours:02}:{minutes:02}:{seconds:02}")
}

fn escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

const STYLE: &str = r##"
:root {
  --bg: #f6f7f9; --surface: #ffffff; --text: #1d2330; --muted: #677085;
  --border: #dde1e8; --primary: #2563eb; --primary-text: #ffffff;
  --danger: #c62828; --ok-bg: #e3f6e8; --ok-text: #17703a;
  --idle-bg: #eef0f4; --idle-text: #556074;
  color-scheme: light dark;
}
@media (prefers-color-scheme: dark) {
  :root {
    --bg: #12151b; --surface: #1b1f27; --text: #e6e9ef; --muted: #98a1b3;
    --border: #2d3340; --primary: #4c84ff; --danger: #ef5350;
    --ok-bg: #173323; --ok-text: #6fd492; --idle-bg: #262b35; --idle-text: #a9b1c2;
  }
}
* { box-sizing: border-box; }
body { margin: 0; background: var(--bg); color: var(--text);
  font: 15px/1.5 system-ui, -apple-system, "Segoe UI", Roboto, sans-serif; }
main { max-width: 900px; margin: 0 auto; padding: 32px 16px; }
header { display: flex; align-items: center; justify-content: space-between; gap: 12px; margin-bottom: 20px; }
h1 { margin: 0; font-size: 24px; }
table { width: 100%; border-collapse: collapse; background: var(--surface);
  border: 1px solid var(--border); border-radius: 10px; overflow: hidden; }
th, td { padding: 10px 14px; text-align: left; border-bottom: 1px solid var(--border); vertical-align: middle; }
th { font-size: 12px; text-transform: uppercase; letter-spacing: .04em; color: var(--muted); }
tbody tr:last-child td { border-bottom: none; }
tfoot td { font-weight: 600; border-top: 1px solid var(--border); border-bottom: none; }
tfoot small { font-weight: 400; color: var(--muted); }
.name { font-weight: 600; word-break: break-word; }
.time { font-variant-numeric: tabular-nums; font-family: ui-monospace, "SF Mono", Menlo, Consolas, monospace; white-space: nowrap; }
.empty { text-align: center; color: var(--muted); padding: 32px; }
.actions { text-align: right; white-space: nowrap; }
.actions form, .actions details { display: inline-block; margin-left: 4px; }
.badge { display: inline-block; padding: 2px 10px; border-radius: 999px; font-size: 12px; font-weight: 600; }
.badge.running { background: var(--ok-bg); color: var(--ok-text); }
.badge.paused { background: var(--idle-bg); color: var(--idle-text); }

.btn { display: inline-block; padding: 6px 12px; border-radius: 6px; border: 1px solid var(--border);
  background: var(--surface); color: var(--text); font: inherit; font-size: 14px;
  text-decoration: none; cursor: pointer; line-height: 1.3; }
.btn:hover { border-color: var(--muted); }
.btn.primary { background: var(--primary); border-color: var(--primary); color: var(--primary-text); }
.btn.danger { color: var(--danger); }
.actions .btn.danger:not(summary) { background: var(--danger); border-color: var(--danger); color: #fff; }
details.delete summary { list-style: none; }
details.delete summary::-webkit-details-marker { display: none; }
details.delete[open] summary { display: none; }

.modal { display: none; position: fixed; inset: 0; z-index: 10; align-items: center; justify-content: center; padding: 16px; }
.modal:target { display: flex; }
.backdrop { position: absolute; inset: 0; background: rgba(0, 0, 0, .45); }
.dialog { position: relative; width: 100%; max-width: 400px; background: var(--surface);
  border: 1px solid var(--border); border-radius: 12px; padding: 20px; }
.dialog h2 { margin: 0 0 16px; font-size: 18px; }
.dialog label { display: block; font-size: 13px; color: var(--muted); margin-bottom: 4px; }
.dialog input { width: 100%; padding: 8px 10px; border-radius: 6px; border: 1px solid var(--border);
  background: var(--bg); color: var(--text); font: inherit; }
.dialog .buttons { display: flex; justify-content: flex-end; gap: 8px; margin-top: 20px; }

/* Live clock: animate an integer from --start upward, one step per second. */
@property --s { syntax: "<integer>"; inherits: false; initial-value: 0; }
@supports (width: calc(mod(5px, 2px))) {
  .clock.live { --s: var(--start); animation: tick 3600000s linear forwards;
    counter-reset: h calc(round(down, var(--s) / 3600, 1)) m calc(mod(round(down, var(--s) / 60, 1), 60)) s calc(mod(var(--s), 60)); }
  .clock.live .static { display: none; }
  .clock.live::after { content: counter(h, decimal-leading-zero) ":" counter(m, decimal-leading-zero) ":" counter(s, decimal-leading-zero); }
}
@keyframes tick { from { --s: var(--start); } to { --s: calc(var(--start) + 3600000); } }

@media (max-width: 640px) {
  thead { display: none; }
  table, tbody, tfoot, tr, td { display: block; width: 100%; }
  tr { border-bottom: 1px solid var(--border); padding: 8px 0; }
  td { border: none; padding: 4px 14px; }
  .actions { text-align: left; white-space: normal; }
  .actions form, .actions details { margin: 4px 4px 0 0; }
}
"##;
