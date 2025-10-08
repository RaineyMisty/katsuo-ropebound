use bevy::prelude::*;


struct Bot {
    state_machine: StateMachine,
}

impl Bot{
    pub fn new() -> Self {
        Self {
            state_machine: StateMachine::new(BotState::Idle),
        }
    }

    pub fn change(&mut self, input: &Input) -> (Bot,Input){

        let next = match self.state_machine.current(){
            //idle change to 
            BotState::Idle =>{
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
                    BotState::idle
                }
            }

            BotState::right =>{
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
                    BotState::idle
                }
            }

             BotState::left =>{
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
                    BotState::idle
                }
            }
             BotState::jump =>{
                if input == 0{
                    BotState::jump_r
                }
                else if input == 1{
                    BotState::jump_l
                }
                else if input == 2{
                    BotState::idle
                }
            }
            BotState::jump_r =>{
                if input == 0{
                    BotState::right
                }
                else if input == 1{
                    BotState::left
                }
                else if input == 2{
                    BotState::idle
                }
            }
            BotState::jump_l =>{
                if input == 0{
                    BotState::right
                }
                else if input == 1{
                    BotState::left
                }
                else if input == 2{
                    BotState::idle
                }
            }
            };
            self.state_machine.transition(next_state);
            
        }
    }