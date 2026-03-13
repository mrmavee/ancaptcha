/**
 * anCaptcha integration example for PHP FFI.
 *
 * Demonstrates challenge generation and automated verification from 
 * form data.
 *
 * Setup:
 * 1. Enable PHP ffi extension in php.ini (ffi.enable=on).
 * 2. Put libancaptcha_ffi.so and ancaptcha-ffi.h in project root or system path.
 * 3. Paths can be set via env vars (ANCAPTCHA_LIB_PATH/ANCAPTCHA_HEADER_PATH).
 * 4. Include AnCaptcha.php wrapper.
 *
 * Tip: Call warmUp() during init to pre-compute assets.
 */

require_once __DIR__ . "/AnCaptcha.php";

// Environment-aware paths for the FFI library and header.
$header_path = getenv("ANCAPTCHA_HEADER_PATH") ?: __DIR__ . "/ancaptcha-ffi.h";
$lib_path = getenv("ANCAPTCHA_LIB_PATH") ?: __DIR__ . "/libancaptcha_ffi.so";

// Use a real 32-byte key from env vars in production.
// Generate with: `openssl rand -hex 32`
$secret = str_repeat("\0", 32);

try {
    $captcha = new AnCaptcha($lib_path, $header_path, $secret);
    
    // Asset pre-computation is triggered synchronously here for demonstration.
    $captcha->warmUp('rotate');
    $captcha->warmUp('slider');
    $captcha->warmUp('pair');
} catch (AnCaptchaError $e) {
    error_log("Critical anCaptcha Error: " . $e->getMessage());
    die("<h1>Critical Error</h1>Failed to initialize security system.");
}

$kind_raw = $_GET['kind'] ?? ($_POST['kind'] ?? null);
$diff_raw = $_GET['diff'] ?? ($_POST['diff'] ?? null);

$kind = is_string($kind_raw) ? $kind_raw : '';
$diff = is_string($diff_raw) ? $diff_raw : '';

/**
 * Helper for isolated template rendering.
 */
function render(string $path, array $view_data): void
{
    extract($view_data);
    include $path;
}

if ($kind === '' || $diff === '') {
    render(__DIR__ . "/templates/index.php", []);
    exit;
}

$success = false;
$user_error_msg = null;

// Handle verification POST requests.
if (isset($_SERVER['REQUEST_METHOD']) && $_SERVER['REQUEST_METHOD'] === 'POST') {
    $form_data = http_build_query($_POST);
    
    try {
        // Automated verification parses the raw form body and detects the challenge type.
        $isValid = $captcha->verify($form_data);
        if ($isValid) {
            $success = true;
        } else {
            $user_error_msg = "Incorrect answer. Please try again.";
        }
    } catch (AnCaptchaError $e) {
        error_log("anCaptcha Verification Failed: " . $e->getMessage());
        $user_error_msg = "Invalid or expired token";
    }
}

// Handle challenge generation for both GET and failed POST attempts.
try {
    $bundle = $captcha->generate($kind, $diff, $user_error_msg);
} catch (AnCaptchaError $e) {
    error_log("anCaptcha Generation Failed: " . $e->getMessage());
    die("<h1>System Error</h1>Failed to load captcha. Please try again later.");
}

$status_html = "";
if ($success) {
    $status_html = '<div style="background:#d4edda;color:#155724;padding:15px;border-radius:4px;margin-bottom:20px;text-align:center;font-weight:bold;">Comment posted successfully!</div>';
}

render(__DIR__ . "/templates/captcha.php", [
    "kind" => ucfirst($kind),
    "diff" => ucfirst($diff),
    "token" => $bundle['token'],
    "html" => $bundle['html'],
    "css" => $bundle['css'],
    "status_html" => $status_html,
    "show_form" => true,
]);
