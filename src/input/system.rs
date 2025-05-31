use bevy::prelude::*;

use crate::component::PlayerCommand;

pub(super) fn player_input(
    keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut player_command: Query<&mut PlayerCommand>
) {
    let mut player_command = match player_command.single_mut() {
        Ok(player_command) => player_command,
        _ => return,
    };

    use PlayerCommand::*;

    let mut new_player_command = match *player_command {
        None | MoveRight | MoveDown | MoveLeft | MoveUp => *player_command,
        _ => return
    };

    for key in keyboard_input.get_pressed() {
        // log::debug!("Key pressed: {key:?}");

        new_player_command = match new_player_command {
            None => match key {
                KeyCode::ArrowRight => MoveRight,
                KeyCode::ArrowDown => MoveDown,
                KeyCode::ArrowLeft => MoveLeft,
                KeyCode::ArrowUp => MoveUp,
                _ => None
            }
            MoveRight => match key {
                KeyCode::ArrowDown => MoveDownRight,
                KeyCode::ArrowUp => MoveUpRight,
                _ => new_player_command
            }
            MoveDown => match key {
                KeyCode::ArrowRight => MoveDownRight,
                KeyCode::ArrowLeft => MoveDownLeft,
                _ => new_player_command
            },
            MoveLeft => match key {
                KeyCode::ArrowDown => MoveDownLeft,
                KeyCode::ArrowUp => MoveUpLeft,
                _ => new_player_command
            },
            MoveUp => match key {
                KeyCode::ArrowRight => MoveUpRight,
                KeyCode::ArrowLeft => MoveUpLeft,
                _ => new_player_command
            },
            _ => new_player_command
        }; 
    }

    *player_command = new_player_command;
}
