extern crate tcod;
use tcod::console::{Root, Console, BackgroundFlag, Offscreen, blit};
use tcod::console::{TextAlignment, Renderer};
use tcod::input::{Key, KeyCode, KEY_PRESS, MOUSE, check_for_event, Event};
use tcod::system;
use tcod::colors;
use tcod::chars;

fn render_colors(first: bool) -> () {}
fn render_offscreen(first:bool) -> () {}
fn render_lines(first: bool) -> () {}
fn render_noise(first: bool) -> () {}
fn render_fov(first: bool) -> () {}
fn render_path(first: bool) -> () {}
fn render_bsp(first: bool) -> () {}
fn render_image(first: bool) -> () {}
fn render_mouse(first: bool) -> () {}
fn render_name(first: bool) -> () {}
fn render_sdl(first: bool) -> () {}

struct MenuItem {
    name : String,
    function : fn(bool) -> ()
}

impl MenuItem {
    fn new(name : &str, f : fn(bool) -> ()) -> Self {
        MenuItem { name: name.to_string(), function: f}
    }
}

fn main() {
    let samples = vec![
        MenuItem::new("  True colors      ", render_colors),
        MenuItem::new("  Offscreen console", render_offscreen),
        MenuItem::new("  Line drawing     ", render_lines),
        MenuItem::new("  Noise            ", render_noise),
        MenuItem::new("  Field of view    ", render_fov),
        MenuItem::new("  Path finding     ", render_path),
        MenuItem::new("  Bsp toolkit      ", render_bsp),
        MenuItem::new("  Image toolkit    ", render_image),
        MenuItem::new("  Mouse support    ", render_mouse),
        MenuItem::new("  Name generator   ", render_name),
        MenuItem::new("  SDL callback     ", render_sdl)
            ];
    let mut cur_sample = 0;
    let mut first = true;
    
    let renderer = Renderer::SDL;
    let mut root = Root::initializer()
        .size(80, 50)
        .title("libtcod Rust sample")
        .fullscreen(false)
        .renderer(renderer)
        .init();
    while !root.window_closed() {
        // print the list of samples
        for i in 0..samples.len() {
            if i == cur_sample {
                root.set_default_foreground(colors::WHITE);
                root.set_default_background(colors::LIGHT_BLUE);
            } else {
                root.set_default_foreground(colors::GREY);
                root.set_default_background(colors::BLACK);
            }
            let y : i32 = 46 - (samples.len() as i32 - i as i32);
            let fun = &samples[i].name;
            root.print_ex(2, y, BackgroundFlag::Set, TextAlignment::Left, fun);
        }

        // print the help message
        root.set_default_foreground(colors::GREY);
        root.print_ex(79, 46, BackgroundFlag::None, TextAlignment::Right,
                      format!("last frame : {:3.0} ms ({:3} fps)",
                      system::get_last_frame_length() * 1000.0,
                              system::get_fps()));
        let time = system::get_elapsed_time();
        root.print_ex(79, 47, BackgroundFlag::None, TextAlignment::Right,
                      format!("elapsed {:8}ms {:4.2}s",
                              time.num_milliseconds(),
                              time.num_milliseconds() as f32/ 1000.0));
        root.print(2, 47, format!("{}{} : select a sample",
                                  chars::ARROW_N, chars::ARROW_S));
        let fullscreen_text = if root.is_fullscreen() {"windowed mode"}
                              else {"fullscren_mode"};
        root.print(2, 48, format!("ALT-ENTER : switch to {}", fullscreen_text));
        
        root.flush();
        let event = check_for_event(KEY_PRESS | MOUSE);
        match event {
            None => {continue;}
            Some((flag, Event::Key(state))) => {
                match state.key {
                    Key::Special(KeyCode::Down) => {
                        cur_sample = (cur_sample + 1) % samples.len();
                        first = true
                    }
                    Key::Special(KeyCode::Up) => {
                        if cur_sample == 0 { cur_sample = samples.len()-1; }
                        else { cur_sample -= 1; }
                        first = true
                    }
                    Key::Special(KeyCode::Enter) if state.left_alt => {
                        let fullscreen = root.is_fullscreen();
                        root.set_fullscreen(!fullscreen)
                    }
                    Key::Special(KeyCode::PrintScreen) => {
                        // TODO
                    }
                    Key::Special(KeyCode::Escape) => {break}
                    _ => {continue;}
                }
            }
            _ => {continue;}
        }
    }
}
