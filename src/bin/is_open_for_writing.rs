// use std::sync::RwLock;
// use std::time::{Duration, Instant};
// use inotify::{Inotify, WatchMask};

// lazy_static::lazy_static! {
//     static ref FILE_HAS_ACTIVE_WRITE_HANDLE: RwLock<Option<bool>> = RwLock::new(None);
// }

fn main() -> std::io::Result<()> {
//     let mut inotify = Inotify::init()?;
//     inotify.watches().add(".", WatchMask::CLOSE_WRITE | WatchMask::MODIFY)?;
//     let mut in_buffer = [0; 1024];

//     let duration = Duration::new(5, 0);
//     let start = Instant::now();

//     while Instant::now() - start < duration {
//         let in_events = inotify.read_events_blocking(&mut in_buffer)?;
//         for in_event in in_events {
//             if in_event.mask.contains(inotify::EventMask::CLOSE_WRITE) {
//                 let mut file_has_active_write_handle = FILE_HAS_ACTIVE_WRITE_HANDLE.write().unwrap();
//                 *file_has_active_write_handle = Some(false);
//             }
//             if in_event.mask.contains(inotify::EventMask::MODIFY) {
//                 let mut file_has_active_write_handle = FILE_HAS_ACTIVE_WRITE_HANDLE.write().unwrap();
//                 *file_has_active_write_handle = Some(true);
//             }
//         }
//     }

//     let file_has_active_write_handle = FILE_HAS_ACTIVE_WRITE_HANDLE.read().unwrap();
//     match *file_has_active_write_handle {
//         Some(true) => println!("File is open for writing"),
//         Some(false) => println!("File has been closed by writer"),
//         None => println!("maybe"),
//     }
    Ok(())
} 