use std::{
    cmp::min, fmt::{Display, Formatter}, io::{self, stdout, Write}, slice::Iter, str::Chars, time::Duration
};

use crossterm::{
    cursor::{MoveDown, MoveToColumn, MoveUp}, queue, style::{Attribute, Color, Print, SetAttribute, SetForegroundColor}
};

use self::word::StatefulChar;

use super::{Count, Idx, TermUnit};

use crate::{
    config::ConfigTextBox, 
    engine::Punct,
    mode::WordSource,
};

mod word;

use word::{Word, State};

pub struct Text {
    words : Vec<Word>,
    words_source : Box<dyn WordSource>,
    selected_word : Idx,
    word_print_offset : Idx,
    line_chars : TermUnit,
    total_chars_to_show : Count,
    config : ConfigTextBox,
    size : (TermUnit, TermUnit),
    column : TermUnit,
    total_duration : Duration,
    last_key_duration : Duration,
}

impl Text {

    pub fn new(words_source : Box<dyn WordSource>, 
        config : ConfigTextBox,
        dur : Duration, 
        size : (TermUnit, TermUnit)) -> Text {

        let words : Vec<Word> = Vec::new();

        let mut textbox =  Text {
            words,
            words_source,
            selected_word : 0,
            word_print_offset : 0,
            line_chars : 0,
            total_chars_to_show : 0,
            config,
            size, 
            column : 0,
            total_duration : dur,
            last_key_duration : Duration::ZERO,
        };

        textbox.set_size(size);
        if let Some(w) = textbox.words.get_mut(0) {
            w.select();
        }
        return textbox;
    }

    fn print_word(&self, word : &Word, max_width : TermUnit) -> io::Result<TermUnit> {
        return print_word(&self.config, word, max_width);
    }

    fn print_stateful_chars(&self, chars : Iter<StatefulChar>) -> io::Result<()> {
        return print_stateful_chars(&self.config, chars);
    }

    fn print_selected_word(&self) -> io::Result<TermUnit> {
        self.go_to_selected_word()?;
        let result = self.print_word(
            &self.words[self.selected_word], 
            self.get_size_x() - self.line_chars
        );
        return result;
    }


    fn go_to_selected_word(&self) -> io::Result<()> {
        return queue!(stdout()
            ,MoveToColumn(self.column + self.line_chars as TermUnit));
    }

    pub fn type_char(&mut self, c : char) -> io::Result<()> {
        self.words[self.selected_word]
            .push_duration(self.last_key_duration);

        if !c.is_whitespace() {
            let n_extra_before = self.words[self.selected_word].n_extra();

            self.words[self.selected_word]
                .type_char(c);

            let n_extra_now = self.words[self.selected_word].n_extra();

            // Print change
            if n_extra_now != n_extra_before {
                write!(stdout(), "{self}")?;
            } else {
                self.print_selected_word()?;
            }

        } else {
            self.next_word()?;
        }
        self.last_key_duration = Duration::ZERO;
        return Ok(());
    }

    fn next_word(&mut self) -> io::Result<()> {
        // Unselect actual word
        self.words[self.selected_word].unselect();
        self.print_selected_word()?;

        // Update internal state
        let n_word_chars = self.words[self.selected_word].n_chars_and_extra();
        self.total_chars_to_show -= (n_word_chars + 1) as Count;
        self.line_chars += n_word_chars + 1;

        // Select next word
        self.selected_word += 1;
        self.words[self.selected_word].select();

        // Checks if next line
        let n_word_chars = self.words[self.selected_word].n_chars_and_extra(); 
        if self.get_size_x() <= self.line_chars + n_word_chars {
            self.word_print_offset = self.selected_word;
            self.line_chars = 0;
            self.complete_size();
            write!(stdout(), "{self}")?;
        } else {
            self.print_selected_word()?;
        }

        return Ok(());
    }

    pub fn backspace(&mut self) -> io::Result<()> {

        let n_extra_before = self.words[self.selected_word].n_extra();
        self.words[self.selected_word].pop();
        let n_extra_now = self.words[self.selected_word].n_extra();

        // Print change
        if n_extra_now != n_extra_before {
            write!(stdout(), "{}", self)?;
        } else {
            self.print_selected_word()?;
        }

        return Ok(());
    }

    pub fn update_time(&mut self, dur : Duration) -> bool {
        self.total_duration -= min(dur, self.total_duration);
        self.last_key_duration += dur;
        return self.total_duration != Duration::ZERO;
    }

    pub fn get_remaining_time(&mut self) -> &Duration {
        return &self.total_duration; 
    }

    pub fn get_punct(&self, total_time : Duration) -> Punct {

        let mut c_right : Count = 0;
        let mut c_wrong : Count = 0;
        let mut c_extra : Count = 0;
        let mut c_missed : Count = 0;
        let mut w_right : Count = 0;
        let mut w_wrong : Count = 0;

        for i in self.words.iter() {
            let w_punct = i.get_punct();

            c_right += w_punct.right;
            c_wrong += w_punct.wrong;
            c_extra += w_punct.extra;
            c_missed += w_punct.missed;

            match w_punct.state {
                State::Right => w_right += 1,
                State::Wrong => w_wrong += 1,
                _ => (),
            }
        }

        let wpm = 
        (c_right) as f64 / total_time.as_secs_f64() * 60.0 / 5.0;
        
        let raw = 
        (c_right + c_wrong) as f64 / total_time.as_secs_f64() * 60.0 / 5.0;
        
        let acc = 
        w_right as f64 / (w_right + w_wrong) as f64;

        let mecano_punct = Punct {
            c_right ,
            c_wrong ,
            c_extra ,
            c_missed ,
            w_right ,
            w_wrong ,
            wpm ,
            raw ,
            acc ,
        };

        return mecano_punct;
    }

    fn complete_size(&mut self) {
        while self.total_chars_to_show < (self.get_size_x() * self.get_size_y()) as Count {
            let new_word = self.words_source.yield_word();
            self.total_chars_to_show += (new_word.chars().count() + 1) as Count;
            self.words.push(Word::from_str(new_word));
        }
    }

    fn get_size_x(&self) -> TermUnit { return self.size.0; }

    fn get_size_y(&self) -> TermUnit { return self.size.1; }

    pub fn set_size(&mut self, size : (TermUnit, TermUnit)) {
        self.size = size;
        self.complete_size();
    }

    pub fn set_column(&mut self, column : TermUnit) { self.column = column; }
}



impl Display for Text {
    // REFACTOR
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let go_to_column = MoveToColumn(self.column);
        write!(f, "{go_to_column}")?;

        // Clean textbox stdout buffer
        let move_down = MoveDown(1);
        let blank_width = " ".repeat(self.get_size_x() as usize);
        for _ in 0..self.get_size_y() {
            write!(f, "{blank_width}{move_down}{go_to_column}")?;
        }

        // Go back to beginning
        let move_up = MoveUp(1);
        for _ in 0..self.get_size_y() {
            write!(f, "{move_up}")?;
        }
        write!(f, "{go_to_column}")?;

        // Print words
        let mut remaining_width : TermUnit = self.get_size_x();
        let mut lenght : TermUnit = 0;
        for idx in self.word_print_offset..self.words.len() {
            let word = &self.words[idx];

            if remaining_width < word.n_chars() + 1 {
                lenght += 1;
                remaining_width = self.get_size_x();
                write!(f, "{move_down}{go_to_column}")?;
            }

            if lenght >= self.get_size_y() {
                break;
            }

            let result = self.print_word(word, remaining_width);
            if let Ok(n_printed) = result {
                remaining_width -= n_printed;
            } else {
                return std::fmt::Result::Err(std::fmt::Error);
            }
        }
        return Ok(());
    }
}

fn print_word(config : &ConfigTextBox, word : &Word, max_width : TermUnit) -> io::Result<TermUnit> {
    // This case should be ckecked upfront
    assert!(max_width >= word.n_chars());
    if word.is_selected() {
        queue!(stdout(), SetAttribute(Attribute::Underlined))?;
    }

    let n_chars_printed;
    // width enough for everything
    if max_width >= word.n_chars_and_extra() + 1 {
        print_stateful_chars(config, word.chars())?;
        let extra = word.extra();
        queue!(stdout(), SetForegroundColor(config.wrong))?;
        write!(stdout(), "{extra}")?;
        queue!(stdout(), SetForegroundColor(Color::Reset))?;
        n_chars_printed = word.n_chars_and_extra() + 1; 

    // width enough for some extra chars
    } else if max_width > word.n_chars() + 1{
        print_stateful_chars(config, word.chars())?;
        let remaining_width = max_width - word.n_chars();

        if remaining_width > 2 {
            queue!(stdout(), SetForegroundColor(config.wrong))?;
            let extra_to_print = word
                .extra()
                .chars()
                .take(remaining_width as usize - 2)
                .collect::<String>();
            write!(stdout(), "{extra_to_print}")?;

            let last_extra = word
                .extra()
                .chars()
                .last()
                .unwrap_or('\0');
            write!(stdout(), "{last_extra}")?;
            queue!(stdout(), SetForegroundColor(Color::Reset))?;
        } else if remaining_width > 1 {
            let last_extra = word
                .extra()
                .chars()
                .last()
                .unwrap_or('\0');
            write!(stdout(), "{last_extra}")?;
            queue!(stdout(), SetForegroundColor(Color::Reset))?;
        }
                        
        n_chars_printed = max_width;

    // last word char is replaced by last extra if exists
    } else {
        let word_but_last = word
            .chars()
            .take(word.n_chars() as usize - 1)
            .collect::<Word>();
        print_stateful_chars(config, word_but_last.chars())?;

        if word.n_extra() > 0 {
            queue!(stdout(), SetForegroundColor(config.wrong))?;
            let last_extra = word.extra().chars().last().unwrap();
            write!(stdout(), "{last_extra}")?;
        } else {
            queue!(stdout(), SetForegroundColor(config.right))?;
            let last_char = word.chars().last().unwrap().c;
            write!(stdout(), "{last_char}")?;
        }
        queue!(stdout(), SetForegroundColor(Color::Reset))?;
        n_chars_printed = max_width;
    }

    if word.is_selected() {
        queue!(stdout(), SetAttribute(Attribute::Reset))?;
    }
    write!(stdout(), " ")?;
    return Ok(n_chars_printed);
}

fn print_stateful_chars(config : &ConfigTextBox, chars : Iter<StatefulChar>) -> io::Result<()> {
    for character in chars {
        let color = match character.state {
            State::Right => config.right,
            State::Wrong => config.wrong,
            State::Selected => config.selected,
            State::Unreached => Color::Reset,
        };
        queue!(stdout(), 
            SetForegroundColor(color),
            Print(character.c))?;
    }
    return Ok(());
}

#[cfg(test)]

mod test {
    use crate::config::ConfigTextBox;

    use super::{print_word, word::Word};




    #[test]
    fn test_print_word() {
        let mut word = Word::from_str("prueba");

        let incorrect = "pruebaeeee";

        for c in incorrect.chars() {
            word.type_char(c);
        }

        assert!(word.n_chars() == 6);
        let _ = print_word(&ConfigTextBox::default(), &word, 8);
    }
}
