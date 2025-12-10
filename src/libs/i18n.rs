use sys_locale::get_locale;

pub fn get_language(debugging: bool) -> String {
    let args: Vec<String> = std::env::args().collect();

    if let Some(language_flag_pos) = args.iter().position(|a| a == ("--lang")) {
        if let Some(language_arg) = args.get(language_flag_pos + 1) {
            if debugging {
                println!("Trying to use flagged language {:?}", language_arg);
            }
            return language_arg.clone();
        }
    }

    let system_lang = get_locale().unwrap_or("en".to_string());
    if debugging {
        println!("Trying to use systems language {:?}", system_lang);
    }
    system_lang
}
