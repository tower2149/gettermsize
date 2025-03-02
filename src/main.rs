fn get_terminal_size(terminal_col: &mut usize, terminal_lines: &mut usize, terminal_pix_x: &mut usize, terminal_pix_y: &mut usize) {
    use crossterm::terminal::*;
    let _ = enable_raw_mode();
    let Ok(response) = xterm_query::query("\x1b[14t", 100 as u64) else { std::process::exit(0x0100); };
    let _ = disable_raw_mode();
    let terminal_size_pixel = response.replace("t", ";");
    let terminal_size_pixel = terminal_size_pixel.split(';').collect::<Vec<&str>>().clone();
    *terminal_pix_x = terminal_size_pixel[1].to_string().parse().unwrap();
    *terminal_pix_y = terminal_size_pixel[2].to_string().parse().unwrap();

    // 結果を表示
    if let Some((w, h)) = term_size::dimensions() {
        *terminal_col = w;
        *terminal_lines = h;
    } else {
        std::process::exit(0x0100);
    }
}

fn main() {
    let mut terminal_col : usize = 0;
    let mut terminal_lines : usize = 0;
    let mut terminal_pix_x : usize = 0;
    let mut terminal_pix_y : usize = 0;
    get_terminal_size(
        &mut terminal_col,
        &mut terminal_lines,
        &mut terminal_pix_x,
        &mut terminal_pix_y
        );
    println!("{}", &terminal_col);
    println!("{}", &terminal_lines);
    println!("{}", &terminal_pix_x);
    println!("{}", &terminal_pix_y);
}
