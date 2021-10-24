use winit::event::VirtualKeyCode;

pub struct UserInterface{
    curr_string : String,
}

pub enum Command{
    Continue,
    NewShape(u64),
    Quit,
}

impl UserInterface{
    pub fn new() -> Self {
        UserInterface {
            curr_string : String::from(""),
        }
    }

    pub fn process_key_press(&mut self, key_press: &Option<VirtualKeyCode>) -> Command {
        if let Some(keycode) = key_press {
            match keycode {
                VirtualKeyCode::Return => {
                    println!("Enter key was pressed!");
                    if self.curr_string.is_empty() {
                        return Command::Continue;
                    } else {
                        let command_int : u64 = self.curr_string.parse().unwrap();
                        println!("Command {}", command_int);
                        self.curr_string.clear();
                        return Command::NewShape(command_int);
                    }
                },
                VirtualKeyCode::Key1 => {
                    self.curr_string.push('1');
                    return Command::Continue;
                },
                VirtualKeyCode::Key2 => {
                    self.curr_string.push('2');
                    return Command::Continue;
                },
                VirtualKeyCode::Key3 => {
                    self.curr_string.push('3');
                    return Command::Continue;
                },
                VirtualKeyCode::Key4 => {
                    self.curr_string.push('4');
                    return Command::Continue;
                },
                VirtualKeyCode::Key5 => {
                    self.curr_string.push('5');
                    return Command::Continue;
                },
                VirtualKeyCode::Key6 => {
                    self.curr_string.push('6');
                    return Command::Continue;
                },
                VirtualKeyCode::Key7 => {
                    self.curr_string.push('7');
                    return Command::Continue;
                },
                VirtualKeyCode::Key8 => {
                    self.curr_string.push('8');
                    return Command::Continue;
                },
                VirtualKeyCode::Key9 => {
                    self.curr_string.push('9');
                    return Command::Continue;
                },
                VirtualKeyCode::Key0 => {
                    self.curr_string.push('0');
                    return Command::Continue;
                },
                VirtualKeyCode::Back => {
                    self.curr_string.pop();
                    return Command::Continue;
                },
                VirtualKeyCode::Escape => {
                    println!("Quitting!");
                    return Command::Quit;
                },
                _ => {
                    return Command::Continue
                }
            }
        } else {
            return Command::Continue
        }
    }
}