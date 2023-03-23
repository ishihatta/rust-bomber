use crate::ai::ai_player::{AIPlayer, AIPlayerAdditionalInfo};
use crate::game_screen::GameScreen;
use crate::player_operation::PlayerOperation;
use crate::human_operation::HumanOperation;

#[derive(Copy, Clone)]
pub enum PlayerType {
    HUMAN, AI
}

impl PlayerType {
    pub fn get_player_operation(&self, player_number: usize) -> Box<dyn PlayerOperation> {
        match self {
            Self::HUMAN => Box::new(HumanOperation { player_number }),
            Self::AI => Box::new(AIPlayer::new()),
        }
    }

    pub fn get_ai_additional_info(&self, game_screen: &GameScreen, player_number: usize) -> Option<AIPlayerAdditionalInfo> {
        match self {
            Self::HUMAN => None,
            Self::AI => Some(AIPlayerAdditionalInfo::new(game_screen, player_number)),
        }
    }
}