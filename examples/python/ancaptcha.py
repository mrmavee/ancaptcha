"""anCaptcha Python Wrapper.

This module uses `ctypes` to interface with the anCaptcha shared library.
"""

import ctypes
from enum import Enum

CInt = ctypes.c_int32
CCharP = ctypes.c_char_p
CSIZE_T = ctypes.c_size_t
SECRET_KEY_LEN = 32


class AnCaptchaError(Exception):
    """Exception raised for errors in the anCaptcha library."""


class Difficulty(Enum):
    """Challenge difficulty levels."""

    EASY = 0
    MEDIUM = 1
    HARD = 2


class CaptchaType(Enum):
    """Supported captcha challenge styles."""

    ROTATE = "rotate"
    SLIDER = "slider"
    PAIR = "pair"


class AnCaptcha:
    """Interface for stateless captcha generation and verification."""

    def __init__(self, lib_path: str, secret_key: bytes) -> None:
        """Initialize the shared library and configure the secret key.

        Args:
            lib_path: Absolute path to `libancaptcha_ffi.so`.
            secret_key: 32-byte key for authenticated encryption.

        """
        if len(secret_key) != SECRET_KEY_LEN:
            msg = f"Secret key must be exactly {SECRET_KEY_LEN} bytes"
            raise ValueError(msg)

        try:
            self._lib = ctypes.CDLL(lib_path)
        except OSError as e:
            msg = f"Failed to load library at {lib_path}: {e}"
            raise RuntimeError(msg) from e

        self._configure_ffi()
        self._init_library(secret_key)

    def _configure_ffi(self) -> None:
        """Map C function signatures to Python types."""
        self._lib.anCaptcha_set_config.argtypes = [
            ctypes.POINTER(ctypes.c_uint8),
            CInt,
        ]
        self._lib.anCaptcha_set_config.restype = CInt

        gen_sig = [
            CInt,
            CCharP,
            ctypes.POINTER(CCharP),
            ctypes.POINTER(CCharP),
            ctypes.POINTER(CCharP),
        ]
        self._lib.anCaptcha_generate_rotate.argtypes = gen_sig
        self._lib.anCaptcha_generate_rotate.restype = CInt
        self._lib.anCaptcha_generate_slider.argtypes = gen_sig
        self._lib.anCaptcha_generate_slider.restype = CInt
        self._lib.anCaptcha_generate_pair.argtypes = gen_sig
        self._lib.anCaptcha_generate_pair.restype = CInt

        self._lib.anCaptcha_verify_auto.argtypes = [CCharP]
        self._lib.anCaptcha_verify_auto.restype = CInt

        self._lib.anCaptcha_free_string.argtypes = [CCharP]
        self._lib.anCaptcha_free_string.restype = None
        self._lib.anCaptcha_last_error_length.argtypes = []
        self._lib.anCaptcha_last_error_length.restype = CSIZE_T
        self._lib.anCaptcha_last_error_message.argtypes = [CCharP, CSIZE_T]
        self._lib.anCaptcha_last_error_message.restype = CInt

        self._lib.anCaptcha_set_theme.argtypes = [
            CCharP,
            CCharP,
            CCharP,
            CCharP,
            CCharP,
            CCharP,
        ]
        self._lib.anCaptcha_set_theme.restype = CInt

        self._lib.anCaptcha_set_layout.argtypes = [
            CCharP,
            CCharP,
            CCharP,
            CCharP,
            CCharP,
            CCharP,
        ]
        self._lib.anCaptcha_set_layout.restype = CInt

        self._lib.anCaptcha_warm_up.argtypes = [CInt]
        self._lib.anCaptcha_warm_up.restype = CInt

    def _get_last_error(self) -> str:
        """Retrieve the last error message from the library thread-local storage."""
        length = self._lib.anCaptcha_last_error_length()
        if length == 0:
            return "Unknown error"
        buf = ctypes.create_string_buffer(length + 1)
        self._lib.anCaptcha_last_error_message(buf, length + 1)
        return buf.value.decode("utf-8", errors="replace")

    def _init_library(self, secret: bytes) -> None:
        """Set the global library configuration."""
        c_secret = (ctypes.c_uint8 * SECRET_KEY_LEN)(*secret)
        res = self._lib.anCaptcha_set_config(c_secret, 1)
        if res != 0:
            msg = f"Initialization failed: {self._get_last_error()}"
            raise AnCaptchaError(msg)

    def generate(
        self,
        kind: CaptchaType,
        difficulty: Difficulty,
        error_message: str | None = None,
    ) -> dict[str, str]:
        """Generate a challenge bundle.

        Returns:
            A dictionary containing 'token', 'html', and 'css'.

        """
        t_ptr = CCharP()
        h_ptr = CCharP()
        c_ptr = CCharP()

        e_ptr = error_message.encode("utf-8") if error_message else None

        if kind == CaptchaType.ROTATE:
            func = self._lib.anCaptcha_generate_rotate
        elif kind == CaptchaType.SLIDER:
            func = self._lib.anCaptcha_generate_slider
        elif kind == CaptchaType.PAIR:
            func = self._lib.anCaptcha_generate_pair
        else:
            msg = "Invalid captcha type"
            raise ValueError(msg)

        res = func(
            difficulty.value,
            e_ptr,
            ctypes.byref(t_ptr),
            ctypes.byref(h_ptr),
            ctypes.byref(c_ptr),
        )

        if res != 0:
            msg = f"Generation failed: {self._get_last_error()}"
            raise AnCaptchaError(msg)

        try:
            return {
                "token": self._to_str(t_ptr),
                "html": self._to_str(h_ptr),
                "css": self._to_str(c_ptr),
            }
        finally:
            self._lib.anCaptcha_free_string(t_ptr)
            self._lib.anCaptcha_free_string(h_ptr)
            self._lib.anCaptcha_free_string(c_ptr)

    def verify(self, form_data_urlencoded: bytes) -> bool:
        """Verify a submission from raw URL-encoded form data.

        Args:
            form_data_urlencoded: Bytes of the POST request body.

        """
        res = self._lib.anCaptcha_verify_auto(ctypes.c_char_p(form_data_urlencoded))
        if res == 0:
            return True
        if res == 1:
            return False
        msg = f"Verification error: {self._get_last_error()}"
        raise AnCaptchaError(msg)

    @staticmethod
    def _to_str(ptr: CCharP) -> str:
        """Convert a C string pointer to a Python string."""
        if not ptr or not ptr.value:
            return ""
        return ptr.value.decode("utf-8", errors="replace")

    def set_theme(
        self,
        theme: dict[str, str],
    ) -> None:
        """Configure visual colors and typography."""
        res = self._lib.anCaptcha_set_theme(
            theme.get("bg", "").encode("utf-8"),
            theme.get("border", "").encode("utf-8"),
            theme.get("text", "").encode("utf-8"),
            theme.get("accent", "").encode("utf-8"),
            theme.get("error", "").encode("utf-8"),
            theme.get("font", "").encode("utf-8"),
        )
        if res != 0:
            msg = f"Failed to set theme: {self._get_last_error()}"
            raise AnCaptchaError(msg)

    def set_layout(
        self,
        layout: dict[str, str],
    ) -> None:
        """Configure structural dimensions."""
        res = self._lib.anCaptcha_set_layout(
            layout.get("width", "").encode("utf-8"),
            layout.get("max_width", "").encode("utf-8"),
            layout.get("margin", "").encode("utf-8"),
            layout.get("height", "").encode("utf-8"),
            layout.get("padding", "").encode("utf-8"),
            layout.get("checkbox_size", "").encode("utf-8"),
        )
        if res != 0:
            msg = f"Failed to set layout: {self._get_last_error()}"
            raise AnCaptchaError(msg)

    def warm_up(self, kind: CaptchaType) -> None:
        """Pre-compute asset variations for a specific style."""
        style_map = {CaptchaType.ROTATE: 0, CaptchaType.SLIDER: 1, CaptchaType.PAIR: 2}
        style_val = style_map.get(kind, 0)
        res = self._lib.anCaptcha_warm_up(CInt(style_val))
        if res != 0:
            msg = f"Warm up failed: {self._get_last_error()}"
            raise AnCaptchaError(msg)
