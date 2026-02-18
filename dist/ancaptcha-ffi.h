#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Initializes the library with a secret key and noise intensity.
 *
 * # Safety
 *
 * `secret` must be a valid pointer to a 32-byte array.
 */
int32_t anCaptcha_set_config(const uint8_t *secret, int32_t noise_intensity);

/**
 * Configures the visual theme for the captcha interface.
 *
 * # Safety
 *
 * All pointers must be valid null-terminated C strings or null to keep current values.
 */
int32_t anCaptcha_set_theme(const char *background_color,
                            const char *border_color,
                            const char *text_color,
                            const char *accent_color,
                            const char *error_color,
                            const char *font_family);

/**
 * Configures the structural layout dimensions for the captcha.
 *
 * # Safety
 *
 * All pointers must be valid null-terminated C strings or null to keep current values.
 */
int32_t anCaptcha_set_layout(const char *width,
                             const char *max_width,
                             const char *margin,
                             const char *min_height,
                             const char *padding,
                             const char *checkbox_size);

/**
 * Pre-computes asset variations for the specified captcha style.
 *
 * # Safety
 *
 * The function assumes the internal cache is already initialized via `set_config`.
 */
int32_t anCaptcha_warm_up(int32_t style);

/**
 * Returns the length of the last error message in bytes.
 */
size_t anCaptcha_last_error_length(void);

/**
 * Copies the last error message into the provided C buffer.
 *
 * # Safety
 *
 * `buffer` must be a valid pointer to a memory area of at least `length` bytes.
 */
int32_t anCaptcha_last_error_message(char *buffer, size_t length);

/**
 * Frees a C string allocated by the Rust library.
 *
 * # Safety
 *
 * `ptr` must be a valid pointer previously returned by a library function.
 */
void anCaptcha_free_string(char *ptr);

/**
 * Generates a Rotate style captcha bundle.
 *
 * # Safety
 *
 * `token_out`, `html_out`, and `css_out` must be valid pointers to string pointers.
 */
int32_t anCaptcha_generate_rotate(int32_t difficulty,
                                  const char *error_msg,
                                  char **token_out,
                                  char **html_out,
                                  char **css_out);

/**
 * Generates a Slider style captcha bundle.
 *
 * # Safety
 *
 * `token_out`, `html_out`, and `css_out` must be valid pointers to string pointers.
 */
int32_t anCaptcha_generate_slider(int32_t difficulty,
                                  const char *error_msg,
                                  char **token_out,
                                  char **html_out,
                                  char **css_out);

/**
 * Generates a Find the Pair style captcha bundle.
 *
 * # Safety
 *
 * `token_out`, `html_out`, and `css_out` must be valid pointers to string pointers.
 */
int32_t anCaptcha_generate_pair(int32_t difficulty,
                                const char *error_msg,
                                char **token_out,
                                char **html_out,
                                char **css_out);

/**
 * Verifies a Rotate captcha submission.
 *
 * # Safety
 *
 * `token` and `values` must be valid null-terminated C strings.
 */
int32_t anCaptcha_verify_rotate(const char *token, const char *const *values, size_t values_len);

/**
 * Verifies a Slider captcha submission.
 *
 * # Safety
 *
 * `token` and `values` must be valid null-terminated C strings.
 */
int32_t anCaptcha_verify_slider(const char *token, const char *const *values, size_t values_len);

/**
 * Verifies a Find the Pair captcha submission.
 *
 * # Safety
 *
 * `token` and `values` must be valid null-terminated C strings.
 */
int32_t anCaptcha_verify_pair(const char *token,
                              const char *const *const *values,
                              const size_t *stage_lengths,
                              size_t stages_count);

/**
 * Automatically detects and verifies a captcha submission from URL-encoded form data.
 *
 * # Safety
 *
 * `form_data_urlencoded` must be a valid null-terminated C string.
 */
int32_t anCaptcha_verify_auto(const char *form_data_urlencoded);
