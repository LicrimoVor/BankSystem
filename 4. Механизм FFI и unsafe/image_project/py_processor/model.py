from ctypes import c_ubyte
from pathlib import Path
import cv2
import numpy as np


def read_img(path: Path) -> cv2.typing.MatLike:
    with open(path, "rb") as f:
        bytes = bytearray(f.read())

    numpy_array = np.asarray(bytes, dtype=np.uint8)
    img = cv2.imdecode(numpy_array, cv2.IMREAD_COLOR)
    return cv2.cvtColor(img, cv2.COLOR_RGB2RGBA)


def read_config(path: Path):
    with open(path, "r") as f:
        config = "".join(f.readlines())

    return config
