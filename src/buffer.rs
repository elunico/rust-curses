use pancurses::Input;

pub use crate::moveable::Moveable;
pub use crate::positionable::Positionable;
pub mod cursor;
pub use crate::buffer::cursor::Cursor;

pub struct Buffer {
    pub lines: Vec<Vec<char>>,
    pub start_line: i32,
    pub end_line: i32,
    pub start_col: i32,
    pub end_col: i32,
    pub width: i32,
    pub height: i32,
    pub cursor: Cursor,
    pub window: pancurses::Window,
}

impl Positionable for Buffer {
    fn x(&self) -> i32 {
        return self.start_col;
    }
    fn y(&self) -> i32 {
        return self.start_line;
    }
}

impl Moveable for Buffer {
    fn move_down(&mut self) {
        self.start_line += 1;
        self.end_line += 1;
    }
    fn move_up(&mut self) {
        self.start_line -= 1;
        self.end_line -= 1;
    }

    fn move_left(&mut self) {
        self.start_col -= 1;
        self.end_col -= 1;
    }

    fn move_right(&mut self) {
        self.start_col += 1;
        self.end_col += 1;
    }
    fn mv(&mut self, x: i32, y: i32) {
        self.start_col = x;
        self.start_line = y;
        self.end_col = x + self.width;
        self.end_line = y + self.height;
    }
}

pub fn input_from_charq(ch: Option<char>) -> Input {
    if let Some(c) = ch {
        return Input::Character(c);
    } else {
        return Input::Unknown(-1);
    }
}

impl Buffer {
    pub fn new(window: pancurses::Window, width: i32, height: i32) -> Buffer {
        return Buffer {
            window: window,
            lines: Vec::new(),
            start_line: 0,
            end_line: 0,
            start_col: 0,
            end_col: 0,
            cursor: Cursor { x: 0, y: 0 },
            width: width,
            height: height,
        };
    }
    pub fn add_line(&mut self, line: &str) {
        let mut l = line.to_owned();
        l.push('\n');
        self.lines.push(l.chars().collect());
        self.cursor.y += 1;
        self.cursor.x = 0;
    }

    pub fn addstr(&mut self, string: &str) {
        let mut lines: Vec<&str> = string.split_terminator("\n").collect();
        let line_count = string.matches("\n").count();
        self.lines.reserve(line_count);
        for line in lines.iter() {
            self.lines
                .insert(self.cursor.y as usize, line.chars().collect());
            self.cursor.y += 1;
        }
    }

    pub fn addch(&mut self, ch: char) {
        if ch == '\n' {
            self.cursor.y += 1;
            self.cursor.x = 0;
            self.lines.push(vec![]);
        } else {
            if let Some(line) = self.lines.get_mut(self.cursor.y as usize) {
                line.insert(self.cursor.x as usize, ch);
            } else {
                self.lines.insert(self.cursor.y as usize, vec![]);
                self.lines[self.cursor.y as usize].insert(self.cursor.x as usize, ch);
            }
            self.cursor.x += 1;
        }
    }

    pub fn delch(&mut self) {
        // TODO: I have defeated the borrow checker but at what cost?
        let line = self.lines.get_mut(self.cursor.y as usize);
        if let Some(l) = line {
            l.remove(self.cursor.x as usize);
        }
    }

    pub fn display(&self) {
        self.window.erase();
        for (i, line) in self.lines.iter().enumerate() {
            // self.window.mv(i as i32, 0);
            self.window
                .mvaddstr(i as i32, 0, line.into_iter().collect::<String>());
            // self.window.addstr(line.into_iter().collect::<String>());
        }
        self.window.mv(self.cursor.y, self.cursor.x);
        self.window.refresh();
    }

    pub fn backspace(&mut self) {
        if self.cursor.x > 0 {
            self.cursor.x -= 1;
            self.delch();
        } else {
            if self.cursor.y > 0 {
                self.cursor.y -= 1;
                self.cursor.x = self.lines[self.cursor.y as usize].len() as i32;
            }
        }
    }

    pub fn wait(&mut self) {
        loop {
            let input = self.window.getch();
            match input {
                None => {}
                Some(input) => break,
            }
        }
    }

    pub fn clrtoeol(&mut self) {
        let line = self.lines.get_mut(self.cursor.y as usize);
        if let Some(l) = line {
            l.drain(self.cursor.x as usize..);
        }
    }

    pub fn get_max_x(&self) -> i32 {
        return self.window.get_max_x();
    }

    pub fn readch(&self) -> Input {
        return input_from_charq(
            self.lines
                .get(self.cursor.y as usize)
                .unwrap()
                .iter()
                .nth(self.cursor.x as usize)
                .map(|c| *c),
        );
    }

    pub fn getch(&self) -> Option<Input> {
        self.window.getch()
    }

    pub fn get_window(&self) -> &pancurses::Window {
        return &self.window;
    }

    pub fn mvtoeol(&mut self) {
        let line = self.lines.get_mut(self.cursor.y as usize);
        if let Some(l) = line {
            self.cursor.x = l.len() as i32;
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        pancurses::endwin();
    }
}
