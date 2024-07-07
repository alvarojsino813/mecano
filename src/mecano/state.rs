use crate::modes::Mode;
use std::{
    cmp::min, collections::VecDeque, io::{self, stdout, Write}, time::Duration};

use crossterm::{
    cursor::{self, MoveDown, MoveLeft, MoveTo, MoveToColumn, MoveUp}, 
    event::{KeyCode, KeyEvent, KeyModifiers},
    execute,
    queue,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}};


use crate::config::Config;

use super::{buffer::MecanoBuffer, drawing::{draw_box, draw_too_narrow, print_empty_width, BoxInfo}};

#[derive(Clone, Copy)]
enum Screen {
    DictMode,
    Punctuation,
    TooNarrow,
}

pub struct State {
    input_offset : u16,
    typed_word : String,
    buffer : MecanoBuffer,
    n_total_words : usize,
    n_correct_words : usize,
    box_info : BoxInfo,
    config : Config,
    end : bool,
    resized : bool,
    actual_time : Duration,
    screen : Screen, 
}

impl State {
    
    // CHANGE
    pub fn start(config : Config) -> io::Result<State> {

        // TODO: No utilizar unwrap
        let words_source = 
        Mode::new(&config.mode, &config.file, config.width);
        let words_source = words_source.unwrap();

        let _ = crossterm::terminal::enable_raw_mode();
        let _ = execute!(stdout(), EnterAlternateScreen);

        let box_info = BoxInfo::default();
        let actual_time = config.get_max_time(); 
        let buffer = MecanoBuffer::new(
            words_source,
            config.config_line,
            (std::cmp::min::<u16>( box_info.size.1 / 2 - 2,
                config.lines_to_show as u16),
                box_info.width)
        );

        let mut state : State = State {
            typed_word : String::new(),
            input_offset : 0,
            buffer, 
            n_total_words : 0,
            n_correct_words : 0,
            box_info, 
            end : false,
            resized : false,
            config,
            actual_time,
            screen : Screen::DictMode,
        };

        state.draw()?;

        stdout().flush()?;

        return Ok(state);
    }

    pub fn end(&mut self) {
        self.end = true;
        self.screen = Screen::Punctuation;
        let _ = self.draw();
    }

    pub fn is_ended(&self) -> bool { return self.end; }

    pub fn is_resized(&self) -> bool { return self.resized; }

    pub fn update_time(&mut self, elapsed : Duration) -> io::Result<()> {
        self.actual_time -= min(elapsed, self.actual_time);
        self.print_time()?;
        if self.actual_time == Duration::from_secs(0) {
            self.end()
        }
        return Ok(());
    }

    pub fn draw(&mut self) -> io::Result<()> {

        queue!(stdout(), Clear(ClearType::All))?;
        let real_size = crossterm::terminal::size().unwrap();
        self.resized = real_size != self.box_info.size;

        if let Ok(box_info) = BoxInfo::centered(self.config.width, real_size) {
            self.input_offset -= self.box_info.left_padding;
            self.box_info = box_info;
            self.input_offset += self.box_info.left_padding;
            self.buffer.set_size((self.lines_to_show(), self.box_info.width));
            self.buffer.set_column(self.box_info.left_padding);
            self.screen = if self.end { Screen::Punctuation } 
                else { Screen::DictMode }
        } else {
            self.screen = Screen::TooNarrow;
        }

        match self.screen {
            Screen::DictMode => {
                self.draw_dict_mode()?;
            }
            Screen::Punctuation => {
                self.draw_punct()?;
            }
            Screen::TooNarrow => {
                draw_too_narrow()?;
            }
        }

        return Ok(());
    }

    fn draw_dict_mode(&mut self) -> io::Result<()> {
        draw_box((0, 0), self.box_info.size)?;
        draw_box(
            (self.box_info.left_padding - 1, self.box_info.top_padding - 1),
            (self.box_info.width + 2, self.lines_to_show() as u16 + 2))?;
        self.print_time()?;
        self.print_lines()?;
        queue!(stdout(), MoveToColumn(self.box_info.left_padding))?;
        write!(stdout(), "{}", self.typed_word)?;
        stdout().flush()?;

        return Ok(());
    }

    fn draw_punct(&mut self) -> io::Result<()> {
        draw_box((0, 0), self.box_info.size,)?;
        queue!(stdout(),
            MoveTo(self.box_info.size.0 / 2 - 8, self.box_info.size.1 / 2))?;

        write!(stdout(), "Total WPM:   {}",
            self.n_total_words as u64 *
            60 / self.config.get_max_time().as_secs())?;
        queue!(stdout(),
            MoveTo(self.box_info.size.0 / 2 - 8,
                self.box_info.size.1 / 2 + 1))?;

        write!(stdout(), "Correct WPM: {}", 
            self.n_correct_words as u64 * 
            60 / self.config.get_max_time().as_secs())?;
        queue!(stdout(),
            MoveTo(self.box_info.size.0 / 2 - 8,
                self.box_info.size.1 / 2 + 3))?;

        write!(stdout(), "<ESC> to exit")?;
        queue!(stdout(), cursor::Hide)?;
        stdout().flush()?;
        return Ok(());
    }

    pub fn type_key_event(&mut self, key : KeyEvent) -> io::Result<()> {
        if self.end { return Ok(()); }
        if let Screen::TooNarrow = self.screen { return Ok(()); }
        let shift = KeyModifiers::from_name("SHIFT").unwrap();
        let none = KeyModifiers::empty();
        if key.modifiers != shift &&
        key.modifiers != none { return Ok(()); }
        match key.code {
            KeyCode::Char(c) => {
                self.buffer.type_char(c);
                if c.is_whitespace() {
                    self.typed_word.clear();
                    self.input_offset = self.box_info.left_padding;
                } else {
                    self.typed_word.push(c);
                    self.input_offset += 1;
                }
            },

            KeyCode::Backspace => {
                self.buffer.backspace();
                self.typed_word.pop().map(|_| self.input_offset -= 1);
            },

            // Controles para cambiar tamaño

            _ => (),
        }
        return Ok(());
    }

    fn print_lines(&mut self) -> io::Result<()> {
        queue!(stdout(),
            MoveTo(self.box_info.left_padding, self.box_info.top_padding))?;
        write!(stdout(), "{}", self.buffer)?;
        self.go_to_input()?;
        return Ok(());
    }

    fn print_time(&mut self) -> io::Result<()> {
        if let Screen::TooNarrow = self.screen { return Ok(()); }
        queue!(stdout(), 
            MoveTo(self.box_info.left_padding, self.box_info.top_padding - 2))?;
        let secs = self.actual_time.as_secs() % 60;
        let mins = self.actual_time.as_secs() / 60;
        write!(stdout(), "{mins:0>2}:{secs:0>2}")?; 
        self.go_to_input()?;
        stdout().flush()?;

        return Ok(());
    }


    fn go_to_input(&mut self) -> io::Result<()> {
        queue!(stdout(),
            MoveTo(self.input_offset,
                self.box_info.top_padding + self.lines_to_show() as u16 + 1))?;
        return Ok(());
    }

    fn lines_to_show(&self) -> u16 {
        return std::cmp::min::<u16>(
            self.box_info.size.1 / 2 - 2,
            self.config.lines_to_show as u16);
    }
}

impl Drop for State {
    fn drop (&mut self) {
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = execute!(stdout(), Clear(ClearType::All),
            LeaveAlternateScreen, cursor::Show);
    }
}

#[cfg(test)]
mod test {
    use std::thread;
    use std::time::Duration;

    use super::State;
    use crate::find_path_to_file;
    use crate::mecano::Config;


    #[test]
    fn typing_normally() {
        let mut config = Config::_default();
        config.file = find_path_to_file("1_test").expect("1_test file not found");
        config.mode = "dictionary".to_string();
        let mut state = State::start(config).unwrap();
        let keys = [
            'H', 'o', 'l', 'a', ' ', 
            'q', 'u', 'e', ' ',
            't', 'a', 'l', ' ',
            'H', 'o', 'l', 'a', ' ', 
            'H', 'o', 'l', 'a', ' ', 
        ];
        for i in keys {
            thread::sleep(Duration::from_millis(5));
            let _ = state.type_char(i);
        }

        thread::sleep(Duration::from_millis(500));
        assert_eq!(state.n_correct_words, 3);
        assert_eq!(state.n_total_words, 5);
    }
}
