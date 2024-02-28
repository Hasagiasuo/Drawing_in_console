use std::{io::{stdout, Write, Stdout}, time::Duration};

use crossterm::{ cursor, event::{poll, read, Event, KeyCode, KeyEvent}, style::{Color, SetBackgroundColor}, terminal::{self, disable_raw_mode, enable_raw_mode}, QueueableCommand };

#[derive(Debug, Clone)]
struct Point {
  head: char,
  x: u16,
  y: u16
}

impl Point {
  fn new(head: char) -> Point {
    Point {head, x: 1, y: 1}
  }
}

fn show_select_color(root: &mut Stdout, yp: u16, mx: u16) {
  let mut ptr: u16 = yp;
  for _ in 0..yp {
    root.write(" ".as_bytes()).unwrap();
    ptr -= 1;
    root.queue(cursor::MoveTo(mx, ptr)).unwrap();
    root.flush().unwrap();
  }
}

fn show_info(root: &mut Stdout, mx: u16, my: u16) {
  let data = "[R]ed [B]lue [Y]ellow [X]Black [W]hite [G]reen";
  root.queue(cursor::MoveTo(mx, my)).unwrap();
  root.write(data.as_bytes()).unwrap();
  root.queue(cursor::MoveTo(0, 0)).unwrap();
  root.flush().unwrap()
}

fn handle_event(mut point: Point) {
  let mut draw_circle = false;
  let mut root: Stdout = stdout();
  root.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
  root.queue(cursor::Hide).unwrap();
  let win_size: (u16, u16) = terminal::size().expect("INFO: Cannot get size terminal");
  let draw_area: (u16, u16) = (win_size.1-1, win_size.0-1);
  show_info(&mut root, win_size.0, win_size.1);
  let mut color_select: Color = Color::White;
  root.queue(SetBackgroundColor(color_select)).unwrap();
  for x in 0..draw_area.1 {
    for y in 0..draw_area.0 {
      root.write(" ".as_bytes()).unwrap();
      root.queue(cursor::MoveTo(x, y)).unwrap();
      root.flush().unwrap();
    }
  }
  root.write(" ".as_bytes()).unwrap();
  root.flush().unwrap();
  enable_raw_mode().unwrap();
  loop {
    if poll(Duration::from_millis(0)).unwrap() {
      if let Ok(event) = read() {
        match event {
          Event::Key(KeyEvent { code, modifiers: _, kind: _, state: _}) => {
            match code {
              KeyCode::Down => { 
                if point.y < draw_area.0 {
                  point.y += 1;
                }
              } KeyCode::Up => {
                if point.y > 1 {
                  point.y -= 1;
                }
              } KeyCode::Right => {
                if point.x < draw_area.1 {
                  point.x += 1;
                }
              } KeyCode::Left => {
                if point.x > 1 {
                  point.x -= 1;
                }
              } KeyCode::Esc => {
                root.queue(cursor::Show).unwrap();
                root.flush().unwrap();
                disable_raw_mode().unwrap();
                std::process::exit(0);
              } 
              KeyCode::Char('r') => { color_select = Color::Red; }
              KeyCode::Char('w') => { color_select = Color::White; }
              KeyCode::Char('x') => { color_select = Color::Black; }
              KeyCode::Char('g') => { color_select = Color::Green; }
              KeyCode::Char('y') => { color_select = Color::Yellow; }
              KeyCode::Char('b') => { color_select = Color::Blue; }
              KeyCode::Tab => { draw_circle = !draw_circle; }
              _ => {}
            }
            // root.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
            root.queue(cursor::MoveTo(point.x, point.y)).unwrap();
            root.queue(SetBackgroundColor(color_select)).unwrap();
            if draw_circle {
              root.write("•".to_string().as_bytes()).unwrap();
            } else {
              root.write(point.head.to_string().as_bytes()).unwrap();
            }
            show_select_color(&mut root, draw_area.0, draw_area.1);
            root.flush().unwrap();
          } _ => {

          }
        }
      }
    }
  }
}

fn main() {
  let ptr = Point::new(' ');
  handle_event(ptr);
}