use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    pub width: u16,
    pub height: u16,
    pub initial_tick_ms: u64,
    pub minimum_tick_ms: u64,
    pub tick_reduction_ms: u64,
    pub foods_per_speedup: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            width: 50,
            height: 25,
            initial_tick_ms: 150,
            minimum_tick_ms: 50,
            tick_reduction_ms: 10,
            foods_per_speedup: 5,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConfigError {
    WidthTooSmall,
    HeightTooSmall,
    ZeroFoodsPerSpeedup,
    ZeroMinimumTick,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::WidthTooSmall => write!(f, "Width must be at least 5"),
            ConfigError::HeightTooSmall => write!(f, "Height must be at least 5"),
            ConfigError::ZeroFoodsPerSpeedup => {
                write!(f, "Foods per speedup must be greater than 0")
            }
            ConfigError::ZeroMinimumTick => write!(f, "Minimum tick must be greater than 0"),
        }
    }
}

impl std::error::Error for ConfigError {}

impl Config {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.width < 5 {
            return Err(ConfigError::WidthTooSmall);
        }
        if self.height < 5 {
            return Err(ConfigError::HeightTooSmall);
        }
        if self.foods_per_speedup == 0 {
            return Err(ConfigError::ZeroFoodsPerSpeedup);
        }
        if self.minimum_tick_ms == 0 {
            return Err(ConfigError::ZeroMinimumTick);
        }
        Ok(())
    }
}
