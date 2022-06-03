use minigrep;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

// runがResultを返すように変更
#[allow(unused)]
#[warn(unused_must_use)]
mod version6 {
    use std::env;
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::process;

    pub fn main() {
        let args: Vec<String> = env::args().collect();

        let config = Config::new(&args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

        println!("Searching for {}", config.query);
        println!("In file {}", config.filename);

        if let Err(e) = run(config) {
            println!("Application error: {}", e);
            process::exit(1);
        }
    }

    struct Config {
        query: String,
        filename: String,
    }

    impl Config {
        fn new(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("not enough arguments");
            }
            let query = args[1].clone();
            let filename = args[2].clone();
            Ok(Config { query, filename })
        }
    }

    fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let mut f = File::open(config.filename)?;

        let mut contents = String::new();
        f.read_to_string(&mut contents)?;

        println!("With text:\n{}", contents);

        Ok(())
    }
}

//
// ## Result handling: if Err, ...
//
// ? ................... return Err
// expect .............. panic with message
// unwrap .............. panic
// unwrap_or ........... eagerly evaluated value
// unwrap_or_default ... default value of type
// unwrap_or_else ...... lazily evaluated value with closure
//

// Config::newがResultを返すように変更
#[allow(unused)]
mod version5 {
    use std::env;
    use std::fs::File;
    use std::io::prelude::*;
    use std::process;

    pub fn main() {
        let args: Vec<String> = env::args().collect();

        let config = Config::new(&args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

        println!("Searching for {}", config.query);
        println!("In file {}", config.filename);

        run(config);
    }

    struct Config {
        query: String,
        filename: String,
    }

    impl Config {
        fn new(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("not enough arguments");
            }
            let query = args[1].clone();
            let filename = args[2].clone();
            Ok(Config { query, filename })
        }
    }

    fn run(config: Config) {
        let mut f = File::open(config.filename).expect("file not found");

        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        println!("With text:\n{}", contents);
    }
}

// parse_configをConfig::newに変更
// エラー処理を追加
#[allow(unused)]
mod version4 {
    use std::env;
    use std::fs::File;
    use std::io::prelude::*;

    pub fn main() {
        let args: Vec<String> = env::args().collect();

        let config = Config::new(&args);

        println!("Searching for {}", config.query);
        println!("In file {}", config.filename);
    }

    struct Config {
        query: String,
        filename: String,
    }

    impl Config {
        fn new(args: &[String]) -> Config {
            if args.len() < 3 {
                panic!("not enough arguments");
            }
            let query = args[1].clone();
            let filename = args[2].clone();
            Config { query, filename }
        }
    }
}

// Configに設定値をまとめる
#[allow(unused)]
mod version3 {
    use std::env;

    pub fn main() {
        let args: Vec<String> = env::args().collect();

        let config = parse_config(&args);
    }

    struct Config {
        query: String,
        filename: String,
    }

    fn parse_config(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}

// parse_configに分ける
#[allow(unused)]
mod version2 {
    use std::env;

    pub fn main() {
        let args: Vec<String> = env::args().collect();

        let (query, filename) = parse_config(&args);
    }

    fn parse_config(args: &[String]) -> (&str, &str) {
        let query = &args[1];
        let filename = &args[2];

        (query, filename)
    }
}

#[allow(unused)]
mod version1 {
    use std::env;
    use std::fs::File;
    use std::io::prelude::*;

    pub fn main() {
        let args: Vec<String> = env::args().collect();

        let query = &args[1];
        let filename = &args[2];

        println!("Searching for {}", query);
        println!("In file {}", filename);

        let mut f = File::open(filename).expect("file not found");

        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        println!("With text:\n{}", contents);
    }
}
