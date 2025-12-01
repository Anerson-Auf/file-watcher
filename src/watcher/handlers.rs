use notify_debouncer_mini::{DebouncedEvent as DebouncedEventMini, DebouncedEventKind, notify::*};
use notify_debouncer_full::{DebouncedEvent as DebouncedEventFull};
use notify::{event::{DataChange, AccessKind, RenameMode}};
use tracing::info;
use super::worker::Watcher;
use super::filter::{Ignore, Filter};

impl Watcher {
    pub fn handle_event_mini(event: DebouncedEventMini, ignore: Option<&Ignore>, find: Option<&Filter>) -> anyhow::Result<()> {
        match event.kind {
            DebouncedEventKind::Any => {
                if let Some(ignore) = ignore {
                    if ignore.is_ignored(&event.path) {
                        return Ok(());
                    }
                }
                if let Some(find) = find {
                    if !find.is_matched(&event.path) {
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
                if let Some(find) = find {
                    if !find.is_matched(&event.path) {
                        return Ok(());
                    }
                }
                info!("File continuously changed: {}", event.path.display());
            }
            _ => {}
        }
        Ok(())
    }
    
    pub fn handle_event_full(event: DebouncedEventFull, ignore: Option<&Ignore>, find: Option<&Filter>) -> anyhow::Result<()> {
        match event.kind {
            notify::EventKind::Create(_) => {
                for path in &event.paths {
                    if let Some(ignore) = ignore {
                        if ignore.is_ignored(path) {
                            continue;
                        }
                    }
                    if let Some(find) = find {
                        if !find.is_matched(path) {
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
                        if let Some(find) = find {
                            if !find.is_matched(&event.paths[0]) {
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
                    notify::event::ModifyKind::Name(rename_mode) => {
                        match rename_mode {
                            RenameMode::From => {
                                if let Some(path) = event.paths.first() {
                                    if let Some(ignore) = ignore {
                                        if ignore.is_ignored(path) {
                                            return Ok(());
                                        }
                                    }
                                    if let Some(find) = find {
                                        if !find.is_matched(path) {
                                            return Ok(());
                                        }
                                    }
                                    info!("Renamed From: {}", path.display());
                                }
                            }
                            RenameMode::To => {
                                if let Some(path) = event.paths.first() {
                                    if let Some(ignore) = ignore {
                                        if ignore.is_ignored(path) {
                                            return Ok(());
                                        }
                                    }
                                    if let Some(find) = find {
                                        if !find.is_matched(path) {
                                            return Ok(());
                                        }
                                    }
                                    info!("Renamed To: {}", path.display());
                                }
                            }
                            RenameMode::Both => {
                                if event.paths.len() >= 2 {
                                    let from = &event.paths[0];
                                    let to = &event.paths[1];
                                    
                                    if let Some(ignore) = ignore {
                                        if ignore.is_ignored(from) || ignore.is_ignored(to) {
                                            return Ok(());
                                        }
                                    }
                                    if let Some(find) = find {
                                        if !find.is_matched(from) && !find.is_matched(to) {
                                            return Ok(());
                                        }
                                    }
                                    info!("Renamed: {} -> {}", from.display(), to.display());
                                }
                            }
                            _ => {
                                for path in &event.paths {
                                    if let Some(ignore) = ignore {
                                        if ignore.is_ignored(path) {
                                            continue;
                                        }
                                    }
                                    if let Some(find) = find {
                                        if !find.is_matched(path) {
                                            continue;
                                        }
                                    }
                                    info!("Renamed: {} ({:?})", path.display(), rename_mode);
                                }
                            }
                        }
                    }
                    notify::event::ModifyKind::Metadata(meta) => {
                        if let Some(ignore) = ignore {
                            if ignore.is_ignored(&event.paths[0]) {
                                return Ok(());
                            }
                        }
                        if let Some(find) = find {
                            if !find.is_matched(&event.paths[0]) {
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
                if let Some(find) = find {
                    if !find.is_matched(&event.paths[0]) {
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
                    if let Some(find) = find {
                        if !find.is_matched(path) {
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