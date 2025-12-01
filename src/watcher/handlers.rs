use notify_debouncer_mini::{DebouncedEvent as DebouncedEventMini, DebouncedEventKind, notify::*};
use notify_debouncer_full::{DebouncedEvent as DebouncedEventFull};
use notify::{event::{DataChange, AccessKind}};
use tracing::info;
use super::worker::Watcher;

impl Watcher {
    pub fn handle_event_mini(event: DebouncedEventMini) -> anyhow::Result<()> {
        match event.kind {
            DebouncedEventKind::Any => {
                info!("File changed: {}", event.path.display());
            }
            DebouncedEventKind::AnyContinuous => {
                info!("File continuously changed: {}", event.path.display());
            }
            _ => {}
        }
        Ok(())
    }
    
    pub fn handle_event_full(event: DebouncedEventFull) -> anyhow::Result<()> {
        match event.kind {
            notify::EventKind::Create(_) => {
                for path in &event.paths {
                    info!("Created: {}", path.display());
                }
            }
            notify::EventKind::Modify(kind) => {
                match kind {
                    notify::event::ModifyKind::Data(n) => {
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
                            info!("Renamed: {}", path.display());
                        }
                    }
                    notify::event::ModifyKind::Metadata(meta) => {
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
                    info!("Removed: {}", path.display());
                }
            }
            _ => {}
        }
        Ok(())
    }
}