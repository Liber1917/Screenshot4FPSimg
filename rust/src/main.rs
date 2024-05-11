use screenshots::Screen;
use image::{DynamicImage, RgbaImage};
use winput::{Vk, Action, MouseMotion};
use winput::message_loop;
use std::time::{Instant, Duration};
use std::path::{Path, PathBuf};
use std::fs;
use std::thread;

fn main() {
    let start = Instant::now();

    // 创建名为 "images" 的文件夹，如果不存在的话
    let images_dir = Path::new(".").join("Scr_images");
    if !images_dir.exists() {
        fs::create_dir(&images_dir).unwrap();
    }

    // 定时截图的时间间隔（以秒为单位）
    let capture_interval = Duration::from_secs(1); // 每隔1秒截图一次

    // 启动消息循环并获取接收器
    let receiver = message_loop::start().unwrap();

    // 定时截图的线程
    let capture_thread = thread::spawn(move || {
        let mut image_counter = 0; // 用于保存图片的序号
        loop {
            let screens = Screen::all().unwrap();
            for screen in screens {
                println!("capturer {:?}", screen);

                // 构建保存路径，文件名为序号+后缀（例如：1.png, 2.png, 3.png）
                let image_filename = format!("{}.png", image_counter);
                let full_image_path = images_dir.join(&image_filename);

                // 截取屏幕图像
                let image = screen.capture().unwrap();

                // 保存图像
                save_image(&image, &full_image_path);

                // 增加图片序号
                image_counter += 1;
            }

            // 等待指定的时间间隔
            thread::sleep(capture_interval);
        }
    });

    // 进入事件处理循环
    loop {
        // 从接收器中获取下一个事件
        match receiver.next_event() {
            // // 处理键盘事件
            // message_loop::Event::Keyboard { vk, action, .. } => {
            //     println!("Keyboard event - Virtual Key Code: {:?}, Action: {:?}", vk, action);
            // },
            // // 处理鼠标移动事件
            // message_loop::Event::MouseMoveRelative { x, y } => {
            //     println!("Mouse moved relative - X: {}, Y: {}", x, y);
            // },
            // 处理鼠标按键事件
            message_loop::Event::MouseButton { action, button } => {
                println!("Mouse button event - Action: {:?}, Button: {:?}", action, button);
            },
            // // 处理鼠标滚轮事件
            // message_loop::Event::MouseWheel { delta, direction } => {
            //     println!("Mouse wheel event - Delta: {}, Direction: {:?}", delta, direction);
            // },
            // 其他事件暂时不处理，直接忽略
            _ => (),
        }
    }

    // 等待定时截图线程结束
    capture_thread.join().unwrap();

    println!("运行耗时: {:?}", start.elapsed());
}

fn save_image(image: &RgbaImage, filename: &Path) {
    image.save(filename).unwrap();
}
