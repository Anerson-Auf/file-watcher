use notify_debouncer_mini::{DebouncedEvent as DebouncedEventMini, DebouncedEventKind, notify::*};
use notify_debouncer_full::{DebouncedEvent as DebouncedEventFull};
use notify::{event::{DataChange, AccessKind}};
use tracing::info;
use super::worker::Watcher;
use super::ignore::Ignore;

impl Watcher {
    pub fn handle_event_mini(event: DebouncedEventMini, ignore: Option<&Ignore>) -> anyhow::Result<()> {
        match event.kind {
            DebouncedEventKind::Any => {
                if let Some(ignore) = ignore {
                    if ignore.is_ignored(&event.path) {
                        return Ok(());
                    }
                }
                info!("File changed: {}", event.path.display());
            }
            DebouncedEventKind::AnyContinuous => {
                if let Some(ignore) = ignore {
                    if ignore.is_ignored(&event.path) {
                        return Ok(());
                    }
                }
                info!("File continuously changed: {}", event.path.display());
            }
            _ => {}
        }
        Ok(())
    }
    
    pub fn handle_event_full(event: DebouncedEventFull, ignore: Option<&Ignore>) -> anyhow::Result<()> {
        match event.kind {
            notify::EventKind::Create(_) => {
                for path in &event.paths {
                    if let Some(ignore) = ignore {
                        if ignore.is_ignored(path) {
                            continue;
                        }
                    }
                    info!("Created: {}", path.display());
                }
            }
            notify::EventKind::Modify(kind) => {
                match kind {
                    notify::event::ModifyKind::Data(n) => {
                        if let Some(ignore) = ignore {
                            if ignore.is_ignored(&event.paths[0]) {
                                return Ok(());
                            }
                        }
                        match n {
                            DataChange::Any => {
                                info!("Modify data: Any");
                            }
                            DataChange::Size => {
                                info!("Modify data: Size");
                            }
                            DataChange::Content => {
                                info!("Modify data: Content");
                            }
                            DataChange::Other => {
                                info!("Modify data: Other");
                            }
                        }
                    }
                    notify::event::ModifyKind::Name(_) => {
                        for path in &event.paths {
                            if let Some(ignore) = ignore {
                                if ignore.is_ignored(path) {
                                    continue;
                                }
                            }
                            info!("Renamed: {}", path.display());
                        }
                    }
                    notify::event::ModifyKind::Metadata(meta) => {
                        if let Some(ignore) = ignore {
                            if ignore.is_ignored(&event.paths[0]) {
                                return Ok(());
                            }
                        }
                        match meta {
                            event::MetadataKind::AccessTime => {
                                info!("Modify metadata: AccessTime");
                            }
                            event::MetadataKind::WriteTime => {
                                info!("Modify metadata: WriteTime");
                            }
                            event::MetadataKind::Permissions => {
                                info!("Modify metadata: Permissions");
                            }
                            event::MetadataKind::Other => {
                                info!("Modify metadata: Other");
                            }
                            event::MetadataKind::Ownership => {
                                info!("Modify metadata: Other");
                            }
                            event::MetadataKind::Any => {
                                info!("Modify metadata: Other");
                            }
                            event::MetadataKind::Extended => {
                                info!("Modify metadata: Extended");
                            }
                        }
                    }
                    _ => {}
                }
            }
            notify::EventKind::Access(ac) => {
                if let Some(ignore) = ignore {
                    if ignore.is_ignored(&event.paths[0]) {
                        return Ok(());
                    }
                }
                match ac {
                    AccessKind::Any => {
                        info!("Access: Any");
                    }
                    AccessKind::Read => {
                        info!("Access: Read");
                    }
                    AccessKind::Open(_) => {
                        info!("Access: Open");
                    }
                    AccessKind::Close(_) => {
                        info!("Access: Close");
                    }
                    AccessKind::Other => {
                        info!("Access: Other");
                    }
                }
            }
            notify::EventKind::Remove(_) => {
                for path in &event.paths {
                    if let Some(ignore) = ignore {
                        if ignore.is_ignored(path) {
                            continue;
                        }
                    }
                    info!("Removed: {}", path.display());
                }
            }
            _ => {}
        }
        Ok(())
    }
}