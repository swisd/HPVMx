macro_rules! hpvm_log {
    ($color:expr, $prefix:expr, $($arg:tt)*) => {
        uefi::system::with_stdout(|stdout| {
            // Bring the trait into scope INSIDE the closure
            //use uefi::proto::console::text::Output;
            use core::fmt::Write;

            // let old_attribute = stdout.get_attribute().ok();

            // Set prefix color
            let _ = stdout.set_color($color, uefi::proto::console::text::Color::Black);
            let _ = write!(stdout, "[{}] ", $prefix);

            // Reset to white for message
            match $color {
                Color::Yellow => {}
                Color::Red => {}
                _ => {let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);}
            }
            let _ = write!(stdout, $($arg)*);
            let _ = write!(stdout, "\n");
            let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);

            // Restore original attributes if they existed
            // if let Some(attr) = old_attribute {
            //     let _ = stdout.set_attribute(attr);
            // }
        })
    };
}

macro_rules! message {
    ($start:expr, $($arg:tt)*) => {
        uefi::system::with_stdout(|stdout| {
            use core::fmt::Write;
            let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);
            let _ = write!(stdout, $start);
            let _ = write!(stdout, $($arg)*);
            let _ = write!(stdout, "\n");
        })
    }
}

macro_rules! hpvm_info {
    ($tag:expr, $($arg:tt)*) => { hpvm_log!(Color::LightCyan, $tag, $($arg)*) };
}

macro_rules! hpvm_warn {
    ($tag:expr, $($arg:tt)*) => { hpvm_log!(Color::Yellow, $tag, $($arg)*) };
}

// Added this to stop the "unused macro" warning
macro_rules! hpvm_error {
    ($tag:expr, $($arg:tt)*) => { hpvm_log!(Color::Red, $tag, $($arg)*) };
}
