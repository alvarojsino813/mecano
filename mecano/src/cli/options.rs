use std::io;

use crate::config::fields::FieldError;
use crate::{Count, TermUnit};
use crate::config::Config;

use super::config_file_path;

pub fn config_with_args(args : &Vec<String>) -> io::Result<Config> {
    let mut config; 
    let config_file = config_file_path();

    if let Ok(c) = Config::from_path(&config_file_path()) {
        config = c;
    } else {
        let config_file_display = config_file.display();
        let error_msg = format!("invalid configuration in \"{config_file_display}\"");
        return Err(io::Error::new(io::ErrorKind::InvalidData, error_msg));
    }

    let mut args_iter = args.iter().skip(1);
    while let Some(item) = args_iter.next() {
        let opt;
        if let Some(arg) = args_iter.next() {
            opt = arg;
            let mut err : Option<FieldError> = None;
            match item.as_str() {
                "-m" | "--mode" => {
                    err = config.set_mode(opt);
                }

                "-f" | "--file" => {
                    err = config.set_file(opt);
                }

                "-t" | "--time" => {
                    let time = opt.parse::<Count>();
                    if let Err(_) = time {
                        err = Some(FieldError::NotAPositiveNumber);
                    } else {
                        config.set_max_time(time.unwrap());
                    }
                }

                "-r" | "--rate" => {
                    let rate = opt.parse::<TermUnit>();
                    if let Err(_) = rate {
                        err = Some(FieldError::NotAPositiveNumber);
                    } else {
                        config.set_rate(rate.unwrap());
                    }
                },

                _ => (),
            }

            if let Some(e) = err {
                let error = e.error_msg();
                let expecting = e.expecting();
                let err_msg = format!("Error in option \"{item}\": \"{arg}\" {error}. Expecting {expecting}");
                return Err(io::Error::new(io::ErrorKind::InvalidInput, err_msg));
            }


        } else {
            let error_msg = format!("Missing argument for option \"{item}\"");
            return Err(io::Error::new(io::ErrorKind::InvalidInput, error_msg));
        }
    }
    return Ok(config);
}
