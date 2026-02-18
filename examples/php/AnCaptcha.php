<?php

/**
 * Exception thrown for errors within the anCaptcha library.
 */
class AnCaptchaError extends Exception {}

/**
 * Interface for the anCaptcha FFI bridge.
 *
 * This class uses the PHP FFI extension to interact with the Rust-based engine.
 * It provides stateless captcha generation and automated form verification.
 */
class AnCaptcha
{
    private FFI $ffi;

    /**
     * Initializes the library and sets the global secret key.
     *
     * @param string $libPath Path to the shared library object (.so).
     * @param string $headerPath Path to the C header file (.h).
     * @param string $secretKey 32-byte key for authenticated encryption.
     * @throws AnCaptchaError If initialization fails.
     */
    public function __construct(string $libPath, string $headerPath, string $secretKey)
    {
        if (strlen($secretKey) !== 32) {
            throw new AnCaptchaError("Secret key must be 32 bytes");
        }

        $rawCdef = file_get_contents($headerPath);
        if ($rawCdef === false) {
            throw new AnCaptchaError("Failed to read header file: $headerPath");
        }
        
        $cdef = (string)preg_replace('/^#.*$/m', '', $rawCdef);

        try {
            $this->ffi = FFI::cdef($cdef, $libPath);
        } catch (FFI\Exception $e) {
            throw new AnCaptchaError("FFI Init Failed: " . $e->getMessage());
        }

        $this->initLibrary($secretKey);
    }

    private function initLibrary(string $secret): void
    {
        $cSecret = $this->ffi->new("uint8_t[32]");
        for ($i = 0; $i < 32; $i++) {
            $cSecret[$i] = ord($secret[$i]);
        }

        if ($this->ffi->anCaptcha_set_config($cSecret, 1) !== 0) {
            throw new AnCaptchaError("Library Config Failed: " . $this->getLastError());
        }
    }

    private function getLastError(): string
    {
        $len = $this->ffi->anCaptcha_last_error_length();
        if ($len === 0) {
            return "Unknown error";
        }
        $buffer = $this->ffi->new("char[" . ($len + 1) . "]");
        $this->ffi->anCaptcha_last_error_message($buffer, $len + 1);
        return FFI::string($buffer);
    }

    /**
     * Generates a complete captcha challenge bundle.
     *
     * @param string $type Challenge style: 'rotate', 'slider', or 'pair'.
     * @param string $difficulty Level: 'easy', 'medium', or 'hard'.
     * @param string|null $errorMessage Optional message to display in the UI.
     * @return array Array containing 'token', 'html', and 'css'.
     * @throws AnCaptchaError If generation fails.
     */
    public function generate(string $type, string $difficulty, ?string $errorMessage = null): array
    {
        $diffMap = ['easy' => 0, 'medium' => 1, 'hard' => 2];
        $dVal = $diffMap[strtolower($difficulty)] ?? 1;

        $tPtr = $this->ffi->new("char*");
        $hPtr = $this->ffi->new("char*");
        $cPtr = $this->ffi->new("char*");
        
        $ePtr = $errorMessage;

        $type = strtolower($type);
        $res = match ($type) {
            'rotate' => $this->ffi->anCaptcha_generate_rotate($dVal, $ePtr, FFI::addr($tPtr), FFI::addr($hPtr), FFI::addr($cPtr)),
            'slider' => $this->ffi->anCaptcha_generate_slider($dVal, $ePtr, FFI::addr($tPtr), FFI::addr($hPtr), FFI::addr($cPtr)),
            'pair'   => $this->ffi->anCaptcha_generate_pair($dVal, $ePtr, FFI::addr($tPtr), FFI::addr($hPtr), FFI::addr($cPtr)),
            default  => -1
        };

        if ($res !== 0) {
            throw new AnCaptchaError("Generation failed: " . $this->getLastError());
        }

        $result = [
            'token' => FFI::string($tPtr),
            'html'  => FFI::string($hPtr),
            'css'   => FFI::string($cPtr),
        ];

        $this->ffi->anCaptcha_free_string($tPtr);
        $this->ffi->anCaptcha_free_string($hPtr);
        $this->ffi->anCaptcha_free_string($cPtr);

        return $result;
    }

    /**
     * Verifies a submission using automated detection logic.
     *
     * @param string $formData Raw URL-encoded form data (e.g., from http_build_query($_POST)).
     * @return bool True if valid, false otherwise.
     * @throws AnCaptchaError On system-level verification errors.
     */
    public function verify(string $formData): bool
    {
        $res = $this->ffi->anCaptcha_verify_auto($formData);

        if ($res === 0) {
            return true;
        }
        
        if ($res === 1) {
            return false;
        }

        throw new AnCaptchaError("Verification system error: " . $this->getLastError());
    }

    /**
     * Updates the visual theme configuration.
     */
    public function setTheme(string $bg, string $border, string $text, string $accent, string $error, string $font): void
    {
        if ($this->ffi->anCaptcha_set_theme($bg, $border, $text, $accent, $error, $font) !== 0) {
            throw new AnCaptchaError("Failed to set theme: " . $this->getLastError());
        }
    }

    /**
     * Updates the structural layout configuration.
     */
    public function setLayout(string $width, string $maxWidth, string $margin, string $height, string $padding, string $checkboxSize): void
    {
        if ($this->ffi->anCaptcha_set_layout($width, $maxWidth, $margin, $height, $padding, $checkboxSize) !== 0) {
            throw new AnCaptchaError("Failed to set layout: " . $this->getLastError());
        }
    }

    /**
     * Triggers JIT asset pre-computation for a specific style.
     */
    public function warmUp(string $type): void
    {
        $styleMap = ['rotate' => 0, 'slider' => 1, 'pair' => 2];
        $styleVal = $styleMap[strtolower($type)] ?? 0;

        if ($this->ffi->anCaptcha_warm_up($styleVal) !== 0) {
            throw new AnCaptchaError("Warm up failed: " . $this->getLastError());
        }
    }
}
