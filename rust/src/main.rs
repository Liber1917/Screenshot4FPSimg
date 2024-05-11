use screenshots::Screen;
use image::RgbaImage;
use winput::{Action};
use winput::message_loop;
use std::path::{Path};
use std::fs;

fn main() {
    // 创建名为 "images" 的文件夹，如果不存在的话
    let images_dir = Path::new(".").join("Scr_images");
    if !images_dir.exists() {
        fs::create_dir(&images_dir).unwrap();
    }

    // 定义截图计数器
    let mut image_counter = 0;

    // 启动消息循环并获取接收器
    let receiver = message_loop::start().unwrap();

    // 进入事件处理循环
    loop {
        // 从接收器中获取下一个事件
        match receiver.next_event() {
            // 处理鼠标按键事件
            message_loop::Event::MouseButton { action, .. } => {
                println!("Mouse button event - Action: {:?}", action);
                if action == Action::Press {
                    // 如果鼠标按键按下，则进行截图
                    let screens = Screen::all().unwrap();
                    for (index, screen) in screens.iter().enumerate() {
                        println!("capturer {:?}", screen);

                        // 构建保存路径，文件名为序号+后缀（例如：1.png, 2.png, 3.png）
                        let image_filename = format!("{}_{}.png", image_counter, index);
                        let full_image_path = images_dir.join(&image_filename);

                        // 截取屏幕图像
                        let image = screen.capture().unwrap();

                        // 保存图像
                        save_image(&image, &full_image_path);
                    }

                    // 增加截图计数器
                    image_counter += 1;
                }
            },
            // 其他事件暂时不处理，直接忽略
            _ => (),
        }
    }
}

fn save_image(image: &RgbaImage, filename: &Path) {
    image.save(filename).unwrap();
}
