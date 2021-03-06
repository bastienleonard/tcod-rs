extern crate tcod;

use tcod::{Console, BackgroundFlag};

fn main() {
    let mut con = Console::init_root(
        80, 50, "Move the cursor inside the window", false);
    let mut x = 40;
    let mut y = 25;

    while !Console::window_closed() {

        loop {
            let (_flags, event) = tcod::system::check_for_event(
                tcod::KEY | tcod::MOUSE);

            match event {
                tcod::system::Event::Key(ref key_state) => {
                    println!("{:?}", key_state);
                },
                tcod::system::Event::Mouse(ref mouse_state) => {
                    x = mouse_state.cx as i32;
                    y = mouse_state.cy as i32;
                    println!("{:?}", mouse_state);
                }
                tcod::system::Event::None => {
                    break;
                }
            }
        }

        con.clear();
        con.put_char(x, y, '@', BackgroundFlag::Set);
        Console::flush();
    }
}
