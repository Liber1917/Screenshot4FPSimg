use winput::{Action, MouseMotion, Vk};
use winput::message_loop;

fn main() {
    // 启动消息循环并获取接收器
    let receiver = message_loop::start().unwrap();

    // 进入事件处理循环
    loop {
        // 从接收器中获取下一个事件
        match receiver.next_event() {
            // 处理键盘事件
            message_loop::Event::Keyboard { vk, action, .. } => {
                println!("Keyboard event - Virtual Key Code: {:?}, Action: {:?}", vk, action);
            },
            // 处理鼠标移动事件
            message_loop::Event::MouseMoveRelative { x, y } => {
                println!("Mouse moved relative - X: {}, Y: {}", x, y);
            },
            // 处理鼠标按键事件
            message_loop::Event::MouseButton { action, button } => {
                println!("Mouse button event - Action: {:?}, Button: {:?}", action, button);
            },
            // 处理鼠标滚轮事件
            message_loop::Event::MouseWheel { delta, direction } => {
                println!("Mouse wheel event - Delta: {}, Direction: {:?}", delta, direction);
            },
            // 其他事件暂时不处理，直接忽略
            _ => (),
        }
    }
}
