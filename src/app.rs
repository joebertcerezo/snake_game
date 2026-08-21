use std::{error::Error, time::Instant};

use crate::{
    config::Config,
    game::{state::GameState, world::GameWorld},
    input::{AppCommand, poll_command},
    renderer::render_frame,
    terminal::TerminalGuard,
};

pub struct App {
    world: GameWorld,
}

impl App {
    pub fn new(config: Config) -> Result<Self, Box<dyn Error>> {
        config.validate()?;
        Ok(Self { world: GameWorld::new(config) })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let _guard = TerminalGuard::init()?;
        let mut last_tick = Instant::now();
        let mut should_quit = false;

        while !should_quit {
            render_frame(&self.world)?;

            let timeout =
                self.world.tick_rate().checked_sub(last_tick.elapsed()).unwrap_or_default();

            if let Some(command) = poll_command(timeout)? {
                match command {
                    AppCommand::Quit => should_quit = true,
                    AppCommand::Move(dir) => self.world.change_direction(dir),
                    AppCommand::Pause => {
                        let was_paused = self.world.state() == GameState::Paused;
                        self.world.toggle_pause();
                        if was_paused && self.world.state() == GameState::Running {
                            last_tick = Instant::now();
                        }
                    }
                    AppCommand::Restart => {
                        if matches!(self.world.state(), GameState::GameOver | GameState::Won) {
                            self.world.reset();
                            last_tick = Instant::now();
                        }
                    }
                }
            }

            if last_tick.elapsed() >= self.world.tick_rate() {
                if self.world.state() == GameState::Running {
                    self.world.update();
                }
                last_tick = Instant::now();
            }
        }

        Ok(())
    }
}
