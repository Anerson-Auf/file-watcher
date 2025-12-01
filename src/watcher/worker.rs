use walkdir::WalkDir;
use std::path::PathBuf;
use notify_debouncer_mini::{DebouncedEvent as DebouncedEventMini, Debouncer, new_debouncer, notify::*};
use notify_debouncer_full::{new_debouncer as new_debouncer_full, DebounceEventResult};
use notify::{RecommendedWatcher};
use std::sync::mpsc::Receiver;
use tracing::{info, error};

use crate::watcher::filter::{Ignore, Filter};

pub struct Watcher {
    _debouncer_mini: Option<Debouncer<ReadDirectoryChangesWatcher>>,
    _debouncer_full: Option<notify_debouncer_full::Debouncer<RecommendedWatcher, notify_debouncer_full::FileIdMap>>,
    event_receiver_mini: Option<Receiver<Result<Vec<DebouncedEventMini>>>>,
    event_receiver_full: Option<Receiver<DebounceEventResult>>,
}
impl Watcher {
    pub async fn new(path: String, detailed: bool) -> anyhow::Result<Self> {
        let path = PathBuf::from(path);
        
        if !path.exists() {
            anyhow::bail!("Path does not exist: {}", path.display());
        }
        
        if !path.is_file() && !path.is_dir() {
            anyhow::bail!("Path is neither a file nor a directory: {}", path.display());
        }
        
        if detailed {
            let (tx, rx) = std::sync::mpsc::channel();
            let mut debouncer_full = new_debouncer_full(
                std::time::Duration::from_secs(2),
                None,
                move |result: DebounceEventResult| {
                    let _ = tx.send(result);
                },
            )?;
            debouncer_full.watch(path.as_path(), RecursiveMode::Recursive)?;
            Ok(Self {
                _debouncer_mini: None,
                _debouncer_full: Some(debouncer_full),
                event_receiver_mini: None,
                event_receiver_full: Some(rx),
            })
        } else {
            let (tx, rx) = std::sync::mpsc::channel();
            let mut debouncer = new_debouncer(
                std::time::Duration::from_secs(2),
                tx,
            )?;
            debouncer.watcher().watch(path.as_path(), RecursiveMode::Recursive)?;
            Ok(Self {
                _debouncer_mini: Some(debouncer),
                _debouncer_full: None,
                event_receiver_mini: Some(rx),
                event_receiver_full: None,
            })
        }
    }
    pub async fn watch_entry(self, detailed: bool, ignore: Option<Ignore>, find: Option<Filter>) -> anyhow::Result<()> {
        if detailed {
            let receiver = self.event_receiver_full.expect("Full receiver should exist");
            loop {
                match receiver.recv() {
                    Ok(result) => {
                        match result {
                            Ok(events) => {
                                for event in events {
                                    Watcher::handle_event_full(event, ignore.as_ref(), find.as_ref())?;
                                }
                            }
                            Err(errors) => {
                                for err in errors {
                                    error!("Watch error: {}", err);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Channel error: {}", e);
                        break;
                    }
                }
            }
        } else {
            let receiver = self.event_receiver_mini.expect("Mini receiver should exist");
            loop {
                match receiver.recv() {
                    Ok(Ok(events)) => {
                        for event in events {
                            Watcher::handle_event_mini(event, ignore.as_ref(), find.as_ref())?;
                        }
                    }
                    Ok(Err(e)) => {
                        error!("Error receiving events: {}", e);
                    }
                    Err(e) => {
                        error!("Channel error: {}", e);
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    #[allow(unused)]
    #[deprecated(since = "0.1.0", note = "Use watch_entry instead, maybe i can find a way to use it in the future")]
    pub async fn watch(self, _path: PathBuf) {
        std::thread::spawn(move || {
            async move {
                loop {
                    let walker = WalkDir::new(&_path).into_iter();
                    for entry in walker {
                        let entry = entry.unwrap();
                        let path = entry.path();
                        let file_name = path.file_name().unwrap().to_str().unwrap();
                        info!("File: {}", file_name);
                    }
                }
            }
        });
    }
}