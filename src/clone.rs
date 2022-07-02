use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{FetchOptions, Progress, RemoteCallbacks, Repository};
use std::io::{self, Write};
use std::path::PathBuf;

struct State {
    progress: Option<Progress<'static>>,
    total: usize,
    current: usize,
    path: Option<PathBuf>,
    newline: bool,
}

impl State {
    pub fn new() -> State {
        State {
            progress: None,
            total: 0,
            current: 0,
            path: None,
            newline: false,
        }
    }
}

fn print(state: &mut State) {
    let stats = state.progress.as_ref().unwrap();
    let network_pct = (100 * stats.received_objects()) / stats.total_objects();
    let index_pct = (100 * stats.indexed_objects()) / stats.total_objects();
    let co_pct = if state.total > 0 {
        (100 * state.current) / state.total
    } else {
        0
    };
    let kbytes = stats.received_bytes() / 1024;
    if stats.received_objects() == stats.total_objects() {
        if !state.newline {
            println!();
            state.newline = true;
        }
        print!(
            "Resolving deltas {}/{}\r",
            stats.indexed_deltas(),
            stats.total_deltas()
        );
    } else {
        print!(
            "net {:3}% ({:4} kb, {:5}/{:5})  /  idx {:3}% ({:5}/{:5})  \
             /  chk {:3}% ({:4}/{:4}) {}\r",
            network_pct,
            kbytes,
            stats.received_objects(),
            stats.total_objects(),
            index_pct,
            stats.indexed_objects(),
            stats.total_objects(),
            co_pct,
            state.current,
            state.total,
            state
                .path
                .as_ref()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default()
        )
    }
    io::stdout().flush().unwrap();
}

pub fn clone_repo(url: &str, path: &PathBuf) -> Result<Repository, git2::Error> {
    let mut state = State::new();
    let mut transfer_state = State::new();

    let mut cb = RemoteCallbacks::new();

    cb.transfer_progress(move |stats| {
        state.progress = Some(stats.to_owned());
        print(&mut state);
        true
    });

    let mut co = CheckoutBuilder::new();
    co.progress(move |path, cur, total| {
        transfer_state.path = path.map(|p| p.to_path_buf());
        transfer_state.current = cur;
        transfer_state.total = total;
        print(&mut transfer_state);
    });

    let mut fo = FetchOptions::new();
    fo.remote_callbacks(cb);
    RepoBuilder::new()
        .fetch_options(fo)
        .with_checkout(co)
        .clone(url, path)
}
