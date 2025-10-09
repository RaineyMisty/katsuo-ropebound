use bevy::prelude::*;
use rand::prelude::*;
use super::state::*;

pub struct Bot {
    state_machine: StateMachine,
}

#[derive(Event)]
pub struct PlayerEvent{
    pub enetity: Entity,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
}

impl Bot{
    pub fn new() -> Self {
        Self {
            state_machine: StateMachine::new(BotState::idel),
        }
    }

    pub fn Change(&mut self, /*input: &Input*/) -> BotState{
        //temporary random movement to change state
        let rng = rand::rng();
        //remove when done please
        let next = match self.state_machine.current{
            //idel change to 
            BotState::idel =>{
                let input = rand::rng().random_range(0..3);
                if input == 0{
                    BotState::right
                }
                else if input == 1{
                    BotState::left
                }
                else if input == 2{
                    BotState::jump
                }
                else if input == 3{
                    BotState::idel
                }
                else{
                    BotState::idel
                }
            }

            BotState::right =>{
                let input = rand::rng().random_range(0..3);
                if input == 0{
                    BotState::right
                }
                else if input == 1{
                    BotState::left
                }
                else if input == 2{
                    BotState::jump_r
                }
                else if input == 3{
                    BotState::idel
                }
                else{
                    BotState::idel
                }
            }

             BotState::left =>{
                let input = rand::rng().random_range(0..3);
                if input == 0{
                    BotState::right
                }
                else if input == 1{
                    BotState::left
                }
                else if input == 2{
                    BotState::jump_l
                }
                else if input == 3{
                    BotState::idel
                }
                else{
                    BotState::idel
                }
            }
             BotState::jump =>{
                let input = rand::rng().random_range(0..2);
                if input == 0{
                    BotState::jump_r
                }
                else if input == 1{
                    BotState::jump_l
                }
                else if input == 2{
                    BotState::idel
                }
                else{
                    BotState::idel
                }
            }
            BotState::jump_r =>{
                let input = rand::rng().random_range(0..2);
                if input == 0{
                    BotState::right
                }
                else if input == 1{
                    BotState::left
                }
                else if input == 2{
                    BotState::idel
                }
                else{
                    BotState::idel
                }
            }
            BotState::jump_l =>{
                let input = rand::rng().random_range(0..2);
                if input == 0{
                    BotState::right
                }
                else if input == 1{
                    BotState::left
                }
                else if input == 2{
                    BotState::idel
                }
                else{
                    BotState::idel
                }
            }
            };
            return next;
            
        }
    }