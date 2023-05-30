#![no_std]
#![no_main]
#![feature(core_intrinsics)]

use core::arch::global_asm;

use game_logic::{Player, GameState};
use oxos_syscall::{uart_send, exit};
use player_controls::get_field_input;
use rendering::{PlayingField, Symbol};

mod rendering;
mod player_controls;
mod game_logic;
mod panic;

global_asm!(include_str!("init.s"));

#[no_mangle]
pub extern "C" fn main() {
    uart_send("Launching Tic-Tac-Toe...");
    let playing_field = PlayingField::new();
    playing_field.draw_playing_field();

    let mut game_state = GameState::new();
    let mut turn = Player::Pl1;
    let mut winner: Option<Player> = None;

    while winner.is_none() {
        playing_field.valid_play();
        let field = get_field_input();

        if game_state.check_valid_play(field) {
            game_state.register_play(field, turn);
            match turn {
                Player::Pl1 => {
                    playing_field.draw_field_entry(field.0, field.1, Symbol::Cross, 0xabf707);
                    turn = Player::Pl2;
                },
                Player::Pl2 => {
                    playing_field.draw_field_entry(field.0, field.1, Symbol::Circle, 0xf75f07);
                    turn = Player::Pl1;
                },
            }
            winner = game_state.check_for_winner();
        } else {
            playing_field.inindicate_invalid_play();
        }

    }

    let winning_row = match game_state.get_winning_row() {
        Some(winning_row) => winning_row,
        None => panic!(),
    };
    for (x, y) in winning_row {
        playing_field.draw_field_entry(x, y, match winner {
            Some(Player::Pl1) => Symbol::Cross,
            Some(Player::Pl2) => Symbol::Circle,
            None => panic!(),
        }, 0x0000FF)
    }

    loop {}

    //exit();
}
