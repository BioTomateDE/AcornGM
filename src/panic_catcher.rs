use std::panic;
use std::panic::UnwindSafe;
use std::sync::{Arc, Mutex};
use std::thread::Thread;

pub fn catch_panic<T>(function: impl FnOnce() -> T + UnwindSafe) -> T {
    let panic_info = Arc::new(Mutex::new(None));
    let panic_info_clone = panic_info.clone();

    let old_hook = panic::take_hook();

    panic::set_hook(Box::new(move |info| {
        let message = if let Some(s) = info.payload().downcast_ref::<&str>() {
            *s
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.as_str()
        } else {
            "<panic>"
        };

        let location = info.location().map(|l| {
            format!("{}:{}:{}", l.file(), l.line(), l.column())
        }).unwrap_or_else(|| "<unknown>".to_string());

        let thread: Thread = std::thread::current();
        let thread_name: String = match thread.name() {
            Some(name) => name.to_string(),
            None => format!("{:?}", thread.id()),
        };

        let line1 = "========== Rust panicked! ==========";
        let line2 = format!("> {} {}", "Thread:", thread_name);
        let line3 = format!("> {} {}", "Location:", location);
        let line4 = format!("> {} {}", "Message:", message);
        let output = format!("{line1}\n{line2}\n{line3}\n{line4}");

        let mut guard = panic_info_clone.lock().unwrap();
        *guard = Some(output);

        // call the original panic hook to preserve existing logging
        old_hook(info);
    }));

    let result = panic::catch_unwind(|| {
        function()
    });

    if let Ok(value) = result {
        return value
    }

    let info = panic_info.lock().unwrap();
    let message: &str = info.as_ref().map_or("<panic message not set somehow>", |i| i.as_str());

    rfd::MessageDialog::new()
        .set_title("Fatal AcornGM Error")
        .set_description(message)
        .set_buttons(rfd::MessageButtons::Ok)
        .set_level(rfd::MessageLevel::Error)
        .show();
    std::process::exit(1);
}

