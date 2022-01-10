use std::env;

pub struct Config {
    pub src: String
}

impl Config {
    pub fn new(mut args: env::Args)  -> Result<Config, &'static str> {
        args.next();

        let src = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a src string")
        };

        Ok(Config { src })
    }
}