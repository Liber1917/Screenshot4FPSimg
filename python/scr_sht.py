import cv2
from mss import mss
import numpy
import os
import threading
import pynput.mouse
from pynput.mouse import Listener
import time

# 定义截图窗口的位置
ScrX = 2560
ScrY = 1600
# window_size=(
#     int(ScrX/2-520),
#     int(ScrY/2-520),
#     int(ScrX/2+520),
#     int(ScrY/2+520),
# )
window_size=(
    0,
    0,
    int(ScrX),
    int(ScrY),
)
# 创建截图对象
Screenshot_value = mss()

# 指定保存截图的文件夹路径
save_folder = './ScreenShot/'

# 确保文件夹存在，如果不存在则创建
os.makedirs(save_folder, exist_ok=True)

# 计数器，用于生成文件名
count = 1

# 截图并保存函数
def screenshot_and_save():
    global count
    img = Screenshot_value.grab(window_size)
    img = numpy.array(img)
    img = cv2.cvtColor(img, cv2.COLOR_BGRA2BGR)
    # 生成文件名
    file_name = os.path.join(save_folder, f'screenshot_{count}.png')
    # 保存截图
    cv2.imwrite(file_name, img)
    # 更新计数器
    count += 1
    time.sleep(0.01)

shot = 0
def mouse_click(x, y, button, pressed):
    global shot
    if pressed and button == pynput.mouse.Button.left:
        shot = 1
    else:
        shot = 0
    time.sleep(0.1)


# 全局标志位
terminate_flag = False

def listener_thread():
    global terminate_flag
    # 构造监听器对象，方式2（可替换上面with）（监听哪几种类型事件）
    listener = Listener(on_click=mouse_click)
    # 开始监听
    listener.start()
    # 循环检查标志位
    while not terminate_flag:
        time.sleep(0.1)  # 这里可以替换为其他需要执行的代码
        pass  # 可以做一些有用的工作，但同时要注意检查标志位
    # 结束监听
    listener.stop()

if __name__ == "__main__":
    print("start")
    threading.Thread(target=listener_thread).start()
    while True:
        try:
            # terminate_flag = True
            # 启动鼠标监听线程
            # threading.Thread(target=listener_thread).start()
            if shot:
                screenshot_and_save()
                shot = 0
            time.sleep(0.01)
        except KeyboardInterrupt:
            terminate_flag = True
            print("end")
            break

# if __name__ == "__main__":
#     print("start")
#     threading.Thread(target=listener_thread).start()
#     while True:
#         screenshot_and_save()
#         time.sleep(0.2)
