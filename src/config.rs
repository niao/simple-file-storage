use time::Duration;

pub const MAX_FILES: usize = 5;
pub const JWT_LIFESPAN: Duration = Duration::days(30);
pub const BODY_LIMIT: usize = 512_000_000;