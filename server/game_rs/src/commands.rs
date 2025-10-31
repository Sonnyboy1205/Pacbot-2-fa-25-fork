use crate::logging::Logging;
use crate::state::GameState;

pub fn interpret_command(msg: &[u8], gs: GameState) -> bool {
    if Logging::get_command_log_enabled() {
        if msg.len() > 1 {
            
        }
    }
    
    match msg[0] {
        b'p' => {
            
        }   
    }
} 
