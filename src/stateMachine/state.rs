use bevy::prelude::*;

#[derive((Debug, Clone, Copy, PartialEq, Eq, Hash, States, Default))]
enum BotState{
    #[default]
    idel,
    right,
    left,
    jump,
    jump_r,
    jump_l,
}

#[derive(Component)]
struct StateMachine {
    current: BotState,
    previous: BotState,
}

impl StateMachine {
    fn new(Init: BotState) -> Self {
        //this as the constructor for tree
        Self {
            current: init,
            previous: init,
        }
    }

    fn transition(&mut self, new_state: BotState){
        self.previous = self.current;
        self.current = new_state;
    }

    fn current(&self) -> BotState {
        self.current
    }

    fn previous(&self) -> BotState {
        self.previous
    }
}