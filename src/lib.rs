#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("../target/debug/build/rust-ncurses-c09001d048302184/out/bindings.rs");

pub unsafe fn render_ncurses(){
    unsafe {
        initscr();
        printw(b"Hello World\0".as_ptr() as *const i8);
        refresh();
        getch();
        endwin();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_ncurses() {
        unsafe {
            render_ncurses();
        }
    }
}
