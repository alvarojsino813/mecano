use std::path::PathBuf;

use serde::{de::Visitor, Deserialize};

use crate::{mode::{all_modes_str, ALL_MODES}, path_to_file};

#[derive(Debug)]
pub enum FieldError {
    InvalidMode,
    InvalidFile,
    ZeroNotAllowed,
    NotAPositiveNumber,
}

impl FieldError {
    pub fn error_msg(&self) -> String {
        match self {
            FieldError::InvalidMode => "invalid mode",
            FieldError::InvalidFile => "invalid file",
            FieldError::ZeroNotAllowed => "zero not allowed",
            FieldError::NotAPositiveNumber => "invalid or negative number",
        }.to_string()
    }

    pub fn expecting(&self) -> String {
        let all_modes = all_modes_str();
        match self {
            FieldError::InvalidMode => format!("a valid mode among: {all_modes}"),
            FieldError::InvalidFile => String::from("a valid file"),
            FieldError::ZeroNotAllowed => String::from("a number greater than 0"),
            FieldError::NotAPositiveNumber => String::from("a valid positive number")
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModeField {
    field : String,
}

impl ModeField {
    pub fn new(s : &str) -> Result<ModeField, FieldError> { 
        if ALL_MODES.iter().any(|valid_mode| *valid_mode == s) {
            return Ok(ModeField{ field : s.to_string() });
        } else {
            return Err(FieldError::InvalidMode);
        }
    }
    
    pub fn to_string(&self) -> String {
        return self.field.clone();
    }
}

impl<'de> Deserialize<'de> for ModeField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        return deserializer.deserialize_str(ModeFieldVisitor{});
    }
}

struct ModeFieldVisitor { }

impl<'de> Visitor<'de> for ModeFieldVisitor {
    type Value = ModeField;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        let error = FieldError::InvalidMode;
        let expecting = error.expecting();
        return write!(formatter, "{expecting}");
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error, {

        let mode_field = ModeField::new(v);

        if let Ok(mode_field) = mode_field {
            return Ok(mode_field);
        } else {
            let error = FieldError::InvalidMode;
            let error_msg = error.error_msg();
            return Err(E::custom(error_msg));
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileField {
    field : PathBuf,
}

impl FileField {
    pub fn new(s : &str) -> Result<FileField, FieldError> { 
        let result = path_to_file(s);
        if let Ok(path) = result {
            return Ok(FileField{ field : path });
        } else {
            dbg!("Failed FileField::new()");
            let _ = result.inspect_err(|e| eprintln!("{}", e.kind()));
            return Err(FieldError::InvalidFile);
        }
    }

    pub fn get_pathbuf(&self) -> &PathBuf {
        return &self.field;

    }
}

impl<'de> Deserialize<'de> for FileField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        return deserializer.deserialize_str(FileFieldVisitor{});
    }
}

struct FileFieldVisitor { }

impl<'de> Visitor<'de> for FileFieldVisitor {
    type Value = FileField;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        let error = FieldError::InvalidFile;
        let expecting = error.expecting();
        return write!(formatter, "{expecting}");
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error, {

        let file_field = FileField::new(v);

        if let Ok(file_field) = file_field {
            return Ok(file_field);
        } else {
            let error = FieldError::InvalidFile;
            let error_msg = error.error_msg();
            return Err(E::custom(error_msg));
        }
    }
}
