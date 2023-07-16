use ruscii::app::{App, Config, State};
use ruscii::drawing::Pencil;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Window};
use rand::seq::SliceRandom;

fn main() {
    let mut app = App::config(Config::new().fps(20));
    let choices = vec!["rock","paper","scissors"];
    let mut player_choice = "none";
    let mut ai_choice = choices.choose(&mut rand::thread_rng());

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                KeyEvent::Pressed(Key::Enter) => player_choice = "none",
                KeyEvent::Pressed(Key::Num1) => player_choice = "rock",
                KeyEvent::Pressed(Key::Num2) => player_choice = "paper",
                KeyEvent::Pressed(Key::Num3) => player_choice = "scissors",
                _ => (),
            }
        }
        let middle = get_app_size().x / 2;
        if player_choice == "none" {
            ai_choice = choices.choose(&mut rand::thread_rng());
            Pencil::new(window.canvas_mut())
                .set_foreground(Color::Xterm(1))
                .draw_center_text("Rock paper scissors!", Vec2::xy(middle, 0))
                .draw_center_text("Choose an option", Vec2::xy(middle, 9))
                .draw_center_text("1) Rock", Vec2::xy(middle, 10))
                .draw_center_text("2) Paper", Vec2::xy(middle, 12))
                .draw_center_text("3) Scissors", Vec2::xy(middle, 14));
        }
        else {
            let results = handle_end(ai_choice,player_choice);
            Pencil::new(window.canvas_mut())
                .draw_center_text(format!("{}", results[0]).as_str(), Vec2::xy(middle, 14))
                .draw_center_text(format!("{}", results[1]).as_str(), Vec2::xy(middle, 15))
                .draw_center_text("Press enter to resume", Vec2::xy(middle, 16));

            println!("{:?}", results);
        }
    });
}
fn get_app_size() -> Vec2{
    let app = App::config(Config::new().fps(20));
    app.window().size()
}

fn handle_end(enemy_choice:Option<&&str>, player_choice:&str) -> Vec<String>{
    let mut first = "".to_string();
    let mut last = "".to_string();

    if enemy_choice == Some(&player_choice) {
        first = "You tied!".to_string();
        last = format!("Enemy choose {:?}!", enemy_choice);
    }

    if enemy_choice == Some(&"rock") && player_choice == "scissors" {
        first = "You lost!".to_string();
        last = format!("Enemy choose {:?}!", enemy_choice);
    }
    if enemy_choice == Some(&"scissors") && player_choice == "paper" {
        first = "You lost!".to_string();
        last = format!("Enemy choose {:?}!", enemy_choice);
    }
    if enemy_choice == Some(&"paper") && player_choice == "rock" {
        first = "You lost!".to_string();
        last = format!("Enemy choose {:?}!", enemy_choice);
    }

    if enemy_choice == Some(&"rock") && player_choice == "paper" {
        first = "You won!".to_string();
        last = format!("Enemy choose {:?}!", enemy_choice);
    }
    if enemy_choice == Some(&"scissors") && player_choice == "rock" {
        first = "You won!".to_string();
        last = format!("Enemy choose {:?}!", enemy_choice);
    }
    if enemy_choice == Some(&"paper") && player_choice == "scissors" {
        first = "You won!".to_string();
        last = format!("Enemy choose {:?}!", enemy_choice);
    }
    let to_return = vec![first,last];
    to_return
}
