//! Example of anCaptcha integration using Axum.
//!
//! Shows how to use Rotate, Slider, and Pair challenges with different 
//! difficulties. 
//!
//! Setup:
//! 1. Add `ancaptcha` to your Cargo.toml.
//! 2. Call `init_cache()` once during startup.
//! 3. Use a 32-byte secret for initialization.
//!
//! Tip: Run `cache.warm_up()` in a background task to pre-compute assets 
//! so the first request isn't slow.

use ancaptcha::{AnCaptcha, Config, Difficulty, init_cache};
use axum::{
    Router,
    extract::Form,
    response::{Html, IntoResponse, Response},
    routing::get,
};
use tower_http::trace::TraceLayer;
use tracing::{error, info};

use std::sync::OnceLock;

/// Encryption secret. 
/// Use a real 32-byte key from env vars in production.
/// You can generate one with: `openssl rand -hex 32`
static SECRET: OnceLock<[u8; 32]> = OnceLock::new();

/// Verification states for UI feedback.
#[derive(PartialEq, Eq)]
enum VerificationStatus {
    Initial,
    Success,
    WrongAnswer,
    TokenError,
}

/// Retrieves the global secret key from environment or defaults.
fn get_secret() -> &'static [u8; 32] {
    SECRET.get_or_init(|| {
        dotenvy::dotenv().ok();
        let hex_secret = std::env::var("ANCAPTCHA_SECRET").unwrap_or_else(|_| "42".repeat(32));
        let bytes = hex::decode(hex_secret).unwrap_or_else(|_| vec![0x42; 32]);
        let mut key = [0x42; 32];
        let len = bytes.len().min(32);
        key[..len].copy_from_slice(&bytes[..len]);
        key
    })
}

/// Constructs a new `AnCaptcha` service instance.
fn get_ancaptcha() -> AnCaptcha {
    AnCaptcha::new(Config::new(*get_secret()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let cache = init_cache();

    // Trigger asset warming in a background task to avoid blocking startup.
    tokio::task::spawn_blocking(move || {
        cache.warm_up(ancaptcha::CaptchaStyle::Rotate);
        cache.warm_up(ancaptcha::CaptchaStyle::Slider);
        cache.warm_up(ancaptcha::CaptchaStyle::Pair);
    });

    let _ = get_secret();

    let app = Router::new()
        .route("/", get(index))
        .route("/rotate/easy", get(rotate_easy).post(verify_rotate_easy))
        .route(
            "/rotate/medium",
            get(rotate_medium).post(verify_rotate_medium),
        )
        .route("/rotate/hard", get(rotate_hard).post(verify_rotate_hard))
        .route("/slider/easy", get(slider_easy).post(verify_slider_easy))
        .route(
            "/slider/medium",
            get(slider_medium).post(verify_slider_medium),
        )
        .route("/slider/hard", get(slider_hard).post(verify_slider_hard))
        .route("/pair/easy", get(pair_easy).post(verify_pair_easy))
        .route("/pair/medium", get(pair_medium).post(verify_pair_medium))
        .route("/pair/hard", get(pair_hard).post(verify_pair_hard))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    info!("anCaptcha Rust example running on http://127.0.0.1:3000");
    axum::serve(listener, app).await?;

    Ok(())
}

/// Renders the main index page.
async fn index() -> Html<String> {
    Html(
        r"<!DOCTYPE html>
<html>
<head>
<meta charset='utf-8'>
<meta name='viewport' content='width=device-width, initial-scale=1.0'>
<title>anCaptcha Rust Implementation</title>
<style>
body{font-family:sans-serif;margin:0;display:flex;justify-content:center;align-items:center;min-height:100vh;background:#f0f2f5;padding:20px;box-sizing:border-box}
.container{background:#fff;max-width:800px;width:100%;padding:40px;border-radius:12px;box-shadow:0 10px 25px rgba(0,0,0,0.05);box-sizing:border-box}
h1{border-bottom:2px solid #333;margin-top:0}
.section{margin:30px 0;padding:20px;background:#f5f5f5;border-radius:8px}
.section h2{margin-top:0}
ul{list-style:none;padding:0}
li{margin:10px 0}
a{color:#0066cc;text-decoration:none;padding:8px 16px;background:#fff;border:1px solid #ddd;border-radius:4px;display:inline-block;transition:all 0.2s;user-select:none;}
a:hover{background:#0066cc;color:#fff;transform:translateY(-1px);box-shadow:0 2px 5px rgba(0,0,0,0.1);}
a:active{transform:translateY(0);scale:0.98;}
.highlight{background:#fffacd;padding:10px;border-left:4px solid #ff6600;margin:15px 0}
</style>
</head>
<body>
<div class='container'>
<h1>anCaptcha Rust Implementation</h1>
<div class='highlight'>
<strong>Library Implementation Test</strong><br>
This example uses the native Rust crate.
</div>
<div class='section'>
<h2>Rotate Captcha</h2>
<ul>
<li><a href='/rotate/easy'>Easy</a></li><li><a href='/rotate/medium'>Medium</a></li><li><a href='/rotate/hard'>Hard</a></li>
</ul>
</div>
<div class='section'>
<h2>Slider Puzzle</h2>
<ul>
<li><a href='/slider/easy'>Easy</a></li><li><a href='/slider/medium'>Medium</a></li><li><a href='/slider/hard'>Hard</a></li>
</ul>
</div>
<div class='section'>
<h2>Find the Pair</h2>
<ul>
<li><a href='/pair/easy'>Easy</a></li><li><a href='/pair/medium'>Medium</a></li><li><a href='/pair/hard'>Hard</a></li>
</ul>
</div>
</div>
</body>
</html>"
            .to_string(),
    )
}

async fn rotate_easy() -> impl IntoResponse {
    render_captcha(Difficulty::Easy, "Rotate", &VerificationStatus::Initial)
}

async fn verify_rotate_easy(f: Form<Vec<(String, String)>>) -> impl IntoResponse {
    verify_generic(f, Difficulty::Easy, "Rotate")
}

async fn rotate_medium() -> impl IntoResponse {
    render_captcha(Difficulty::Medium, "Rotate", &VerificationStatus::Initial)
}

async fn verify_rotate_medium(f: Form<Vec<(String, String)>>) -> impl IntoResponse {
    verify_generic(f, Difficulty::Medium, "Rotate")
}

async fn rotate_hard() -> impl IntoResponse {
    render_captcha(Difficulty::Hard, "Rotate", &VerificationStatus::Initial)
}

async fn verify_rotate_hard(f: Form<Vec<(String, String)>>) -> impl IntoResponse {
    verify_generic(f, Difficulty::Hard, "Rotate")
}

async fn slider_easy() -> impl IntoResponse {
    render_captcha(Difficulty::Easy, "Slider", &VerificationStatus::Initial)
}

async fn verify_slider_easy(f: Form<Vec<(String, String)>>) -> impl IntoResponse {
    verify_generic(f, Difficulty::Easy, "Slider")
}

async fn slider_medium() -> impl IntoResponse {
    render_captcha(Difficulty::Medium, "Slider", &VerificationStatus::Initial)
}

async fn verify_slider_medium(f: Form<Vec<(String, String)>>) -> impl IntoResponse {
    verify_generic(f, Difficulty::Medium, "Slider")
}

async fn slider_hard() -> impl IntoResponse {
    render_captcha(Difficulty::Hard, "Slider", &VerificationStatus::Initial)
}

async fn verify_slider_hard(f: Form<Vec<(String, String)>>) -> impl IntoResponse {
    verify_generic(f, Difficulty::Hard, "Slider")
}

async fn pair_easy() -> impl IntoResponse {
    render_captcha(Difficulty::Easy, "Pair", &VerificationStatus::Initial)
}

async fn verify_pair_easy(f: Form<Vec<(String, String)>>) -> impl IntoResponse {
    verify_generic(f, Difficulty::Easy, "Pair")
}

async fn pair_medium() -> impl IntoResponse {
    render_captcha(Difficulty::Medium, "Pair", &VerificationStatus::Initial)
}

async fn verify_pair_medium(f: Form<Vec<(String, String)>>) -> impl IntoResponse {
    verify_generic(f, Difficulty::Medium, "Pair")
}

async fn pair_hard() -> impl IntoResponse {
    render_captcha(Difficulty::Hard, "Pair", &VerificationStatus::Initial)
}

async fn verify_pair_hard(f: Form<Vec<(String, String)>>) -> impl IntoResponse {
    verify_generic(f, Difficulty::Hard, "Pair")
}

/// Generic handler for automated form verification.
fn verify_generic(
    Form(pairs): Form<Vec<(String, String)>>,
    diff: Difficulty,
    kind: &str,
) -> impl IntoResponse {
    let mut map = std::collections::HashMap::new();
    for (k, v) in pairs {
        map.entry(k).or_insert_with(Vec::new).push(v);
    }

    let ancaptcha = get_ancaptcha();
    let result = ancaptcha.verify_request(&map);

    handle_verification(result, diff, kind)
}

/// Renders a specific captcha challenge within a form layout.
fn render_captcha(diff: Difficulty, kind: &str, status: &VerificationStatus) -> Response {
    let err_msg = match status {
        VerificationStatus::WrongAnswer => Some("Incorrect answer. Please try again."),
        VerificationStatus::TokenError => Some("Invalid or expired token"),
        _ => None,
    };

    let ancaptcha = AnCaptcha::new(Config::new(*get_secret()).with_difficulty(diff));

    let bundle_res = match kind {
        "Rotate" => ancaptcha.generate_rotate(err_msg),
        "Slider" => ancaptcha.generate_slider(err_msg),
        "Pair" => ancaptcha.generate_pair(err_msg),
        _ => return Html("Unknown kind").into_response(),
    };

    match bundle_res {
        Ok(b) => {
            let status_html = match status {
                VerificationStatus::Success => {
                    r#"<div style="background:#d4edda;color:#155724;padding:15px;border-radius:4px;margin-bottom:20px;text-align:center;font-weight:bold;">Comment posted successfully!</div>"#
                }
                _ => "",
            };

            let form_content = format!(
                r#"<form method="post"><textarea name="comment" placeholder="Write comment...">Simulated comment.</textarea>
                {html}<div style="margin-top:20px;"><button type="submit" class="submit-btn">Submit Comment</button></div>
                </form>"#,
                html = b.html
            );

            Html(wrap_layout(kind, diff, &b.css, status_html, &form_content)).into_response()
        }
        Err(e) => {
            error!("Generation error for {kind}: {e:?}");
            Html("<h3>Error</h3><p>Failed to load captcha. Please try again later.</p><p><a href='/'>Back</a></p>".to_string())
            .into_response()
        }
    }
}

/// Wraps the challenge content in a standard HTML page layout.
fn wrap_layout(
    kind: &str,
    diff: Difficulty,
    css: &str,
    status_html: &str,
    form_content: &str,
) -> String {
    format!(
        r#"<!DOCTYPE html><html><head><meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>{kind} - {diff:?}</title>
        <style>{css}
        body {{ margin: 0; min-height: 100vh; background: #f0f2f5; font-family: sans-serif; padding: 20px; box-sizing: border-box; display: flex; align-items: center; justify-content: center; }}
        .container {{ width: 100%; max-width: 500px; padding: 40px 30px 30px; background: #fff; border-radius: 12px; box-shadow: 0 10px 30px rgba(0,0,0,0.05); box-sizing: border-box; }}
        h2 {{ margin-top: 0; text-align: center; color: #333; }}
        textarea {{ width: 100%; height: 100px; padding: 12px; border: 1px solid #ddd; border-radius: 4px; margin-bottom: 30px; font-family: inherit; box-sizing: border-box; resize: vertical; }}
        .submit-btn {{ background: #2c3e50; color: white; border: none; padding: 12px 24px; border-radius: 4px; cursor: pointer; font-size: 16px; font-weight: bold; width: 100%; transition: all 0.2s; user-select: none; margin-top: 10px; }}
        .submit-btn:hover {{ background: #34495e; transform: translateY(-1px); box-shadow: 0 4px 8px rgba(0,0,0,0.1); }}
        .submit-btn:active {{ transform: translateY(0) scale(0.98); }}
        </style></head><body>
        <div class="container">
            <h2>Post a Comment</h2>
            {status_html}
            {form_content}
            <p style="margin-top:20px;text-align:center;"><a href="/" style="color:#666;text-decoration:none;">&larr; Back to Index</a></p>
        </div>
        </body></html>"#
    )
}

/// Maps verification results to appropriate UI responses.
fn handle_verification(res: ancaptcha::Result<bool>, diff: Difficulty, kind: &str) -> Response {
    match res {
        Ok(true) => render_captcha(diff, kind, &VerificationStatus::Success),
        Ok(false) => render_captcha(diff, kind, &VerificationStatus::WrongAnswer),
        Err(e) => {
            error!("Validation error: {e:?}");
            render_captcha(diff, kind, &VerificationStatus::TokenError)
        }
    }
}
