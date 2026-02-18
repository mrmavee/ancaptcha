"""
anCaptcha integration example using FastAPI.

Shows how to generate and verify challenges using Python ctypes wrapper.

Setup:
1. Put libancaptcha_ffi.so in project root or system path.
2. Have ancaptcha.py wrapper in project directory.
3. Path to .so can be set via ANCAPTCHA_LIB_PATH env var.
4. Use a 32-byte secret for initialization.

Tip: Call warm_up() at startup in background to pre-compute assets.
"""
import os
import sys
import threading
from enum import Enum
from fastapi import FastAPI, Request
from fastapi.responses import HTMLResponse
from fastapi.templating import Jinja2Templates
from ancaptcha import AnCaptcha, CaptchaType, Difficulty, AnCaptchaError

app = FastAPI()
templates = Jinja2Templates(directory="templates")

# Path to the compiled shared library (libancaptcha_ffi.so)
LIB_PATH = os.getenv("ANCAPTCHA_LIB_PATH", "./libancaptcha_ffi.so")

# Use a real 32-byte key from env vars in production.
# Generate with: `openssl rand -hex 32`
SECRET_KEY = bytes([0] * 32)

try:
    captcha_lib = AnCaptcha(LIB_PATH, SECRET_KEY)

    def warmup_task():
        """Runs the captcha warmup process in the background."""
        captcha_lib.warm_up(CaptchaType.ROTATE)
        captcha_lib.warm_up(CaptchaType.SLIDER)
        captcha_lib.warm_up(CaptchaType.PAIR)

    # Asset variations are pre-computed in a background thread to maximize throughput.
    threading.Thread(target=warmup_task, daemon=True).start()

except (RuntimeError, ValueError) as e:
    print(f"CRITICAL: Failed to load anCaptcha: {e}")
    sys.exit(1)

class VerificationStatus(Enum):
    """Internal states for handling verification feedback."""
    INITIAL = 0
    SUCCESS = 1
    WRONG_ANSWER = 2
    TOKEN_ERROR = 3

async def render_page(
    request: Request,
    kind: str,
    diff: str,
    status: VerificationStatus
):
    """
    Renders the captcha challenge page.
    
    Orchestrates bundle generation and status feedback within the Jinja2 template.
    """
    try:
        c_type = CaptchaType(kind)
        c_diff = Difficulty.MEDIUM
        if diff == "easy":
            c_diff = Difficulty.EASY
        elif diff == "hard":
            c_diff = Difficulty.HARD

        user_msg = None
        if status == VerificationStatus.WRONG_ANSWER:
            user_msg = "Incorrect answer. Please try again."
        elif status == VerificationStatus.TOKEN_ERROR:
            user_msg = "Invalid or expired token"

        bundle = captcha_lib.generate(c_type, c_diff, user_msg)

        success_html = ""
        if status == VerificationStatus.SUCCESS:
            success_html = (
                '<div style="background:#d4edda;color:#155724;padding:15px;'
                'border-radius:4px;margin-bottom:20px;text-align:center;'
                'font-weight:bold;">Comment posted successfully!</div>'
            )

        return templates.TemplateResponse(
            "captcha.html",
            {
                "request": request,
                "kind": kind.capitalize(),
                "difficulty": diff.capitalize(),
                "css": bundle["css"],
                "html": bundle["html"],
                "token": bundle["token"],
                "status_html": success_html,
                "show_form": True
            }
        )

    except AnCaptchaError:
        return HTMLResponse(content="System Error: Failed to load captcha.", status_code=500)
    except ValueError:
        return HTMLResponse(content="Invalid captcha type", status_code=400)

@app.get("/")
async def index(request: Request):
    """Serves the main index listing all implementation variants."""
    return templates.TemplateResponse("index.html", {"request": request})

@app.get("/{kind}/{diff}")
async def get_captcha(request: Request, kind: str, diff: str):
    """Handles initial challenge rendering requests."""
    return await render_page(request, kind, diff, VerificationStatus.INITIAL)

@app.post("/{kind}/{diff}")
async def post_captcha(request: Request):
    """
    Processes submission verification.
    
    Extracts the raw body and validates the solution through the FFI wrapper.
    """
    kind = request.path_params.get("kind", "")
    diff = request.path_params.get("diff", "")

    body = await request.body()

    try:
        is_valid = captcha_lib.verify(body)

        if is_valid:
            return await render_page(request, kind, diff, VerificationStatus.SUCCESS)

        return await render_page(request, kind, diff, VerificationStatus.WRONG_ANSWER)

    except AnCaptchaError:
        return await render_page(request, kind, diff, VerificationStatus.TOKEN_ERROR)

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="127.0.0.1", port=8003)
