
use std::thread::sleep;
use std::time::Duration;
 
use hwnd_tracer::*;

fn main() {
    sleep(Duration::from_secs(2));
    let hwnd = get_focus_hwnd();

    println!("hwnd: {:?}", hwnd);

    let title = get_title(hwnd);
    println!("title: {:?}", title);

    let rect = get_rect(hwnd);
    println!("rect: {}", rect);

    let process_id = get_process_id(hwnd);
    println!("process_id: {:?}", process_id);

    let class_name = get_class_name(hwnd);
    println!("class_name: {}", class_name);

    let process_path = get_process_path(hwnd);
    println!("process_path: {}", process_path);
}
