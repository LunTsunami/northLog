pub mod function {
    use ansi_term::Colour;
    pub fn panic_console(level:u64, content: String){
        match level {
            1 => println!("{}{}",Colour::Cyan.paint("Note: "),content),
            2 => println!("{}{}",Colour::Yellow.paint("Warning: "),content),
            3 => println!("{}{}",Colour::Red.paint("Error: "),content),
            _ => println!("x_x")
        }
    }
}
pub mod time_utility {
    /// 时间戳
    use std::time::{SystemTime, UNIX_EPOCH};
    pub fn timestamp_get() -> u128{
        let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

        time
    }
    pub fn timestamp_output() {
        let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

        println!("{}",time);
    }
}