use inotify::{EventMask, Inotify, WatchMask};
use zbus::Connection;

use crate::dbus::connection::BootloaderConfigSignals;

pub async fn listen_files(connection: &Connection) -> zbus::Result<()> {
    let mut inotify = Inotify::init().expect("Failed to initialize inotify");
    inotify
        .watches()
        .add("/etc/default/", WatchMask::MODIFY)
        .expect("Failed to watch /etc/default/grub");

    loop {
        let mut buffer = [0; 4096];
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Failed to read inotify events");

        // prevent duplicate modify event triggers
        let mut signaled = false;
        for event in events {
            if event.mask.contains(EventMask::MODIFY) && !signaled {
                if event.name.is_some_and(|name| name == "grub") {
                    signaled = true;
                    connection
                        .object_server()
                        .interface("/org/opensuse/bootloader")
                        .await?
                        .file_changed()
                        .await?;
                }
            }
        }
    }
}
