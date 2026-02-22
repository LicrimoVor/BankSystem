from pathlib import Path
from ctypes import cdll, c_char_p, c_bool, c_uint32, Structure

ROOT = Path(__file__).parent.parent

# Load the Rust shared library
mylib = cdll.LoadLibrary(ROOT.joinpath("target/release/mylib.dll"))


class Cased(Structure):
    _fields_ = [
        ("cstring", c_char_p),
        ("case", c_bool),
    ]


mylib.count_case_ascii.argtypes = [Cased]
mylib.count_case_ascii.restype = c_uint32

s = b"Hello WORLD abc XYZ"

# Count uppercase letters
upper = Cased(
    cstring=s,
    case=True,
)
upper_count = mylib.count_case_ascii(upper)

# Count lowercase letters
lower = Cased(
    cstring=s,
    case=False,
)
lower_count = mylib.count_case_ascii(lower)

print("Uppercase count:", upper_count)
print("Lowercase count:", lower_count)
