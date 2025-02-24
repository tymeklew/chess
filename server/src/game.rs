pub trait Game {
    async fn start();
}

pub struct PlayerGame {}
pub struct BotGame {}