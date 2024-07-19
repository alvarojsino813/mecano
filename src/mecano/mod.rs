mod drawing;
mod buffer;
mod word;

use std::cmp::min;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};
use std::{io, thread};
use crossterm::cursor::{MoveTo, MoveToColumn};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::{cursor, execute, queue};
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use std::option::Option;

use crate::config::Config;
use crate::modes::Mode;

use self::buffer::MecanoBuffer;
use self::drawing::{draw_box, draw_too_narrow, BoxInfo};

#[derive(Clone, Copy)]
enum Screen {
    DictMode,
    Punctuation,
    TooNarrow,
}

pub struct Mecano { 
    input_offset : u16,
    typed_word : String,
    buffer : MecanoBuffer,
    n_total_words : usize,
    n_correct_words : usize,
    box_info : BoxInfo,
    config : Config,
    end : bool,
    actual_time : Duration,
    screen : Screen, 
}

impl Mecano {
    pub fn play(config : Config) -> io::Result<()> {

        let fps = config.fps;
        let words_source = Mode::new(&config.mode, &config.file, config.width);
        let words_source = words_source.unwrap();

        crossterm::terminal::enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;

        let box_info = BoxInfo::centered(config.width
            , crossterm::terminal::size().unwrap_or((0, 0))).unwrap_or_default();
        let actual_time = config.get_max_time(); 
        let buffer = MecanoBuffer::new(
            words_source,
            config.config_line,
            (std::cmp::min::<u16>( box_info.size.1 / 2 - 2,
                config.lines_to_show as u16),
                box_info.width)
        );

        let mut state : Mecano = Mecano {
            typed_word : String::new(),
            input_offset : box_info.left_padding,
            buffer, 
            n_total_words : 0,
            n_correct_words : 0,
            box_info, 
            end : false,
            config,
            actual_time,
            screen : Screen::DictMode,
        };

        state.draw()?;
        stdout().flush()?;

        let mut running = false;
        let frame_duration = Duration::from_secs_f64(1.0 / fps as f64);
        let mut delta;
        let mut chrono = Instant::now();

        loop {
            state.draw()?;

            while let Ok(true) = poll(Duration::from_secs(0)) {
                if !running {
                    running = true;
                }
                if let None = Mecano::event_read(&mut state) {
                    return Ok(());
                }
            }

            if running {
                state.update_time(frame_duration)?;
            }

            if state.is_ended() {
                break;
            }

            delta = frame_duration - min(frame_duration, chrono.elapsed());
            thread::sleep(delta);
            chrono = Instant::now();
        }

        state.draw()?;
        loop {
            if let None = Mecano::event_read(&mut state) {
                return Ok(());
            }
        }
    }

    fn end(&mut self) {
        self.end = true;
        self.screen = Screen::Punctuation;
        let _ = self.draw();
    }

    fn is_ended(&self) -> bool { return self.end; }

    fn update_time(&mut self, elapsed : Duration) -> io::Result<()> {
        self.actual_time -= min(elapsed, self.actual_time);
        self.print_time()?;
        if self.actual_time == Duration::from_secs(0) {
            self.end()
        }
        return Ok(());
    }

    fn draw(&mut self) -> io::Result<()> {

        queue!(stdout(), Clear(ClearType::All))?;
        let real_size = crossterm::terminal::size().unwrap();

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

    fn type_key_event(&mut self, key : KeyEvent) -> io::Result<()> {
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

            // Other future controls like changing size or navigating through menus

            KeyCode::Right => {
                self.config.width += 4;
            },

            KeyCode::Left => {
                self.config.width -= std::cmp::min(self.config.width, 4);
            },

            KeyCode::Down => {
                self.config.lines_to_show += 1;
            },

            KeyCode::Up => {
                self.config.lines_to_show -= std::cmp::min(self.config.lines_to_show, 1);
            },


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

    // TODO : Bad error managing
    fn event_read(state : &mut Mecano) -> Option<()> {
        if let Ok(event) = read() {
            if let Event::Key(key_event) = event {
                match key_event.code {
                    KeyCode::Esc => {
                        return None;
                    },
                    _ => {
                        let _ = state.type_key_event(key_event);
                    }
                }
            }
        }
        return Some(());
    }
}

impl Drop for Mecano {
    fn drop (&mut self) {
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = execute!(stdout(), Clear(ClearType::All),
            LeaveAlternateScreen, cursor::Show);
    }
}
