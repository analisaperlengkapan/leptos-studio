use leptos::*;
use gloo_net::http::Request;

#[component]
pub fn GitPanel() -> impl IntoView {
    let (status, set_status) = create_signal(String::from("(loading...)"));
    let (log, set_log) = create_signal(String::new());
    let (commit_msg, set_commit_msg) = create_signal(String::new());
    let (busy, set_busy) = create_signal(false);

    // Fetch git status on mount
    leptos::spawn_local(async move {
        set_status.set("Memuat status...".to_string());
        match Request::get("/api/git/status").send().await {
            Ok(r) => match r.text().await {
                Ok(txt) => set_status.set(txt),
                Err(_) => set_status.set("(Gagal membaca status)".to_string()),
            },
            Err(_) => set_status.set("(Gagal fetch status)".to_string()),
        }
    });

    let do_commit = move |_| {
        set_busy.set(true);
        let msg = commit_msg.get();
        let set_status = set_status.clone();
        let set_busy = set_busy.clone();
        leptos::spawn_local(async move {
            match Request::post("/api/git/commit").body(msg).expect("Failed to build request").send().await {
                Ok(r) => match r.text().await {
                    Ok(txt) => set_status.set(txt),
                    Err(_) => {},
                },
                Err(_) => {},
            }
            set_busy.set(false);
        });
    };
    let do_log = move |_| {
        set_busy.set(true);
        let set_log = set_log.clone();
        let set_busy = set_busy.clone();
        leptos::spawn_local(async move {
            match Request::get("/api/git/log").send().await {
                Ok(r) => match r.text().await {
                    Ok(txt) => set_log.set(txt),
                    Err(_) => {},
                },
                Err(_) => {},
            }
            set_busy.set(false);
        });
    };
    view! {
        <div style="margin-top:8px;">
            <div style="font-size:12px;white-space:pre-line;background:#eee;padding:4px 8px;border-radius:4px;">{status}</div>
            <input placeholder="Commit message..." prop:value=commit_msg on:input=move |ev| set_commit_msg.set(event_target_value(&ev)) style="margin:8px 0;width:100%;padding:4px;" />
            <button on:click=do_commit disabled=busy.get() || commit_msg.get().trim().is_empty() style="margin-right:8px;">Commit</button>
            <button on:click=do_log disabled=busy.get() style="margin-right:8px;">Log</button>
            <div style="font-size:12px;white-space:pre-line;background:#f4f4f4;padding:4px 8px;border-radius:4px;margin-top:8px;">{log}</div>
        </div>
    }
}
