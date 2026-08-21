use std::{
    io::{self, stdout},
    sync::atomic::AtomicBool,
};

use crossterm::{
    cursor::Hide, execute, terminal::EnterAlternateScreen, terminal::disable_raw_mode,
    terminal::enable_raw_mode,
};

static CLEANED_UP: AtomicBool = AtomicBool::new(false);

pub struct TerminalGuard;

impl TerminalGuard {
    pub fn init() -> io::Result<Self> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, Hide)?;

        let default_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            Self::cleanup_raw();
            default_hook(panic_info);
        }));

        Ok(Self)
    }

    fn cleanup_raw() {
        if !CLEANED_UP.load(std::sync::atomic::Ordering::SeqCst) {
            let mut stdout = stdout();
            execute!(stdout, crossterm::terminal::LeaveAlternateScreen).unwrap();
            disable_raw_mode().unwrap();
            CLEANED_UP.store(true, std::sync::atomic::Ordering::SeqCst);
        }
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        Self::cleanup_raw();
    }
}
