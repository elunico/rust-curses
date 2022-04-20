extern crate pancurses;

use pancurses::Input;
pub mod buffer;
pub mod moveable;
pub mod positionable;
pub mod reporting;
pub mod windowimpl;
use windowimpl::Moveable;
use windowimpl::Positionable;

const CTRL_A: Input = Input::Character('\u{1}');
const CTRL_B: Input = Input::Character('\u{2}');
// const CTRL_C: Input = Input::Character('\u{3}');
const CTRL_D: Input = Input::Character('\u{4}');
const CTRL_E: Input = Input::Character('\u{5}');
const CTRL_F: Input = Input::Character('\u{6}');
const CTRL_G: Input = Input::Character('\u{7}');
const CTRL_H: Input = Input::Character('\u{8}');
const CTRL_I: Input = Input::Character('\u{9}');
const CTRL_J: Input = Input::Character('\u{a}');
const CTRL_K: Input = Input::Character('\u{b}');
const CTRL_L: Input = Input::Character('\u{c}');
const CTRL_M: Input = Input::Character('\u{d}');
const CTRL_N: Input = Input::Character('\u{e}');
const CTRL_O: Input = Input::Character('\u{f}');
const CTRL_P: Input = Input::Character('\u{10}');
// const CTRL_Q: Input = Input::Character('\u{11}');
const CTRL_R: Input = Input::Character('\u{12}');
// const CTRL_S: Input = Input::Character('\u{13}');
const CTRL_T: Input = Input::Character('\u{14}');
const CTRL_U: Input = Input::Character('\u{15}');
const CTRL_V: Input = Input::Character('\u{16}');
const CTRL_W: Input = Input::Character('\u{17}');
const CTRL_X: Input = Input::Character('\u{18}');
const CTRL_Y: Input = Input::Character('\u{19}');
// const CTRL_Z: Input = Input::Character('\u{1a}');
const ESC: Input = Input::Character('\u{1b}');

pub fn input_from(chtype: pancurses::chtype) -> Input {
    let ch = chtype & pancurses::A_CHARTEXT;
    let character = char::from_u32(ch);
    if let Some(c) = character {
        return Input::Character(c);
    } else {
        return Input::Unknown(ch.try_into().unwrap_or(-1));
    }
}

pub fn readch(window: &mut pancurses::Window) -> Input {
    let ch = window.mvinch(window.y(), window.x());
    let character = char::from_u32(ch & pancurses::A_CHARTEXT);
    if let Some(c) = character {
        return Input::Character(c);
    } else {
        return Input::Unknown(ch.try_into().unwrap_or(-1));
    }
}

pub fn mainloop(window: &mut pancurses::Window) {
    loop {
        let input = window.getch();
        match input {
            None => {}
            Some(input) => {
                match input {
                    CTRL_A => {
                        window.mv(window.y() - 1, 0);
                        window.clrtoeol();
                        window.addstr("Enter a filename: ");
                    }

                    Input::Character('\u{7f}') | Input::KeyBackspace => {
                        if window.x() > 0 {
                            window.move_left();
                        } else if window.y() > 0 {
                            window.move_up();
                            window.mv(window.y(), window.get_max_x() - 1);
                            while readch(window) == Input::Character(' ') {
                                window.move_left();
                                if window.x() == 0 {
                                    window.mv(window.y() - 1, window.get_max_x() - 1);
                                }
                            }
                            window.move_right();
                        }
                        window.delch();
                    }
                    ESC => {
                        break;
                    }
                    Input::Character(c) => {
                        window.addch(c);
                    }
                    Input::KeyUp => {
                        window.move_up();
                    }
                    Input::KeyDown => {
                        window.move_down();
                    }
                    Input::KeyLeft => {
                        window.move_left();
                    }
                    Input::KeyRight => {
                        window.move_right();
                    }
                    e @ _ if std::env::var("DEBUG").is_ok() => {
                        window.addstr(&format!("Unhandled key: {:?} ", e));
                    }
                    _ => {}
                };
            }
        }
    }
}

fn main() {
    // let mut window = pancurses::initscr();
    // window.printw("Type things, press delete to quit\n");
    // window.refresh();
    // window.keypad(true);
    // pancurses::noecho();

    // if std::env::args()
    //     .nth(1)
    //     .unwrap_or("".to_string())
    //     .to_lowercase()
    //     == "report"
    // {
    //     reporting::report_chars(&window);
    // } else {
    //     mainloop(&mut window);
    // }
    // pancurses::endwin();
    if std::env::args()
        .nth(1)
        .unwrap_or("".to_string())
        .to_lowercase()
        == "report"
    {
        let mut window = pancurses::initscr();
        reporting::report_chars(&mut window);
    } else {
        let window = pancurses::initscr();
        window.nodelay(true);
        window.keypad(true);
        pancurses::noecho();
        let mut buffer = buffer::Buffer::new(window, 80, 24);

        loop {
            let input = buffer.getch();
            match input {
                None => {}
                Some(input) => match input {
                    ESC => {
                        break;
                    }
                    CTRL_A => {
                        buffer.clrtoeol();
                    }
                    CTRL_B => {
                        buffer.cursor.mv(0, 0);
                    }
                    CTRL_N => {
                        buffer.mvtoeol();
                    }
                    Input::KeyBackspace | Input::Character('\u{7f}') => {
                        buffer.backspace();
                    }
                    Input::Character(c) => {
                        buffer.addch(c);
                    }
                    Input::KeyUp => {
                        buffer.cursor.move_up();
                    }
                    Input::KeyDown => {
                        buffer.cursor.move_down();
                    }
                    Input::KeyLeft => {
                        buffer.cursor.move_left();
                    }
                    Input::KeyRight => {
                        buffer.cursor.move_right();
                    }
                    _ => {}
                },
            }
            buffer.display();
        }
    }
}
