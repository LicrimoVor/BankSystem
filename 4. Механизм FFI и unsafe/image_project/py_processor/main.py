from pathlib import Path
import cv2
from ctypes import cdll, c_uint32, c_char_p, c_uint8, POINTER
import numpy as np
from model import read_img, read_config

root = Path(__file__).parent.parent
img_path = root.joinpath("assets/ava.jpg")
lib_path = root.joinpath("target/debug/plugin_blur.dll")
# lib_path = root.joinpath("target/debug/plugin_mirror.dll")
config_path = root.joinpath("py_processor/blur.json")

old_img = read_img(img_path)
new_img = old_img.copy()
height, width, _ = new_img.shape
img_ptr = new_img.ctypes.data_as(POINTER(c_uint8))

config = read_config(config_path)

lib = cdll.LoadLibrary(lib_path)
lib.process_image.argtypes = [c_uint32, c_uint32, POINTER(c_uint8), c_char_p]
lib.process_image(c_uint32(width), c_uint32(height), img_ptr, config.encode("utf-8"))

double_img = cv2.resize(np.concat([old_img, new_img], axis=1), (1000, 800))


cv2.imshow("img", double_img)
cv2.waitKey(0)
