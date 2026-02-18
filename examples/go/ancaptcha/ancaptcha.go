// Package ancaptcha provides the CGO bindings for the anCaptcha library.
//
// The package interacts with the compiled anCaptcha shared library
// to provide stateless captcha generation and verification.
package ancaptcha

/*
#cgo LDFLAGS: -lancaptcha_ffi
#include "ancaptcha-ffi.h"
#include <stdlib.h>
*/
import "C"
import (
	"errors"
	"html/template"
	"unsafe"
)

// Bundle contains the generated captcha artifacts.
type Bundle struct {
	Token string
	HTML  template.HTML
	CSS   template.CSS
}

// Client is the main wrapper struct for interacting with the shared library.
type Client struct{}

const (
	secretLen = 32
	diffEasy  = 0
	diffMed   = 1
	diffHard  = 2

	// StyleRotate represents the image rotation challenge.
	StyleRotate = 0
	// StyleSlider represents the puzzle slider challenge.
	StyleSlider = 1
	// StylePair represents the matching pair identification challenge.
	StylePair = 2
)

// New initializes the library with a 32-byte secret key.
//
// The secret key must be consistent across all nodes in a distributed environment
// to maintain stateless verification integrity.
func New(secret []byte) (*Client, error) {
	if len(secret) != secretLen {
		return nil, errors.New("secret must be 32 bytes")
	}
	
	if res := C.anCaptcha_set_config((*C.uint8_t)(unsafe.Pointer(&secret[0])), diffMed); int32(res) != 0 {
		return nil, errors.New("init failed: " + getLastError())
	}
	
	return &Client{}, nil
}

func getLastError() string {
	length := C.anCaptcha_last_error_length()
	if length == 0 {
		return "Unknown error"
	}
	buffer := make([]C.char, length+1)
	C.anCaptcha_last_error_message(&buffer[0], length+1)
	return C.GoString(&buffer[0])
}

// Generate creates a captcha bundle of the specified kind and difficulty.
func (a *Client) Generate(kind, diff string, errorMsg string) (*Bundle, error) {
	var tOut, hOut, cOut *C.char
	var res C.int32_t

	diffMap := map[string]int32{"easy": diffEasy, "medium": diffMed, "hard": diffHard}
	dVal := int32(diffMed)
	if v, ok := diffMap[diff]; ok {
		dVal = v
	}

	var cError *C.char
	if errorMsg != "" {
		cError = C.CString(errorMsg)
		defer C.free(unsafe.Pointer(cError))
	}

	pt, ph, pc := &tOut, &hOut, &cOut

	switch kind {
	case "rotate":
		res = C.anCaptcha_generate_rotate(C.int32_t(dVal), cError, pt, ph, pc)
	case "slider":
		res = C.anCaptcha_generate_slider(C.int32_t(dVal), cError, pt, ph, pc)
	case "pair":
		res = C.anCaptcha_generate_pair(C.int32_t(dVal), cError, pt, ph, pc)
	default:
		return nil, errors.New("unknown captcha type")
	}

	if int32(res) != 0 {
		return nil, errors.New("generation failed: " + getLastError())
	}
	
	defer C.anCaptcha_free_string(tOut)
	defer C.anCaptcha_free_string(hOut)
	defer C.anCaptcha_free_string(cOut)

	return &Bundle{
		Token: C.GoString(tOut),
		HTML:  template.HTML(C.GoString(hOut)), //nolint:gosec
		CSS:   template.CSS(C.GoString(cOut)), //nolint:gosec
	}, nil
}

// Verify checks the submitted form data using the auto-detection logic.
//
// The formData should be provided as a raw URL-encoded string.
func (a *Client) Verify(formData string) (bool, error) {
	cForm := C.CString(formData)
	defer C.free(unsafe.Pointer(cForm))

	res := int32(C.anCaptcha_verify_auto(cForm))

	if res == 0 {
		return true, nil
	}
	if res == 1 {
		return false, nil
	}
	return false, errors.New("verification error: " + getLastError())
}

// SetTheme configures the visual presentation of the captcha interface.
func (a *Client) SetTheme(bg, border, text, accent, error, font string) error {
	cBg := C.CString(bg)
	defer C.free(unsafe.Pointer(cBg))
	cBrd := C.CString(border)
	defer C.free(unsafe.Pointer(cBrd))
	cTxt := C.CString(text)
	defer C.free(unsafe.Pointer(cTxt))
	cAcc := C.CString(accent)
	defer C.free(unsafe.Pointer(cAcc))
	cErr := C.CString(error)
	defer C.free(unsafe.Pointer(cErr))
	cFnt := C.CString(font)
	defer C.free(unsafe.Pointer(cFnt))

	if res := C.anCaptcha_set_theme(cBg, cBrd, cTxt, cAcc, cErr, cFnt); int32(res) != 0 {
		return errors.New("failed to set theme: " + getLastError())
	}
	return nil
}

// SetLayout configures the structural dimensions and spacing.
func (a *Client) SetLayout(width, maxWidth, margin, height, padding, checkboxSize string) error {
	cW := C.CString(width)
	defer C.free(unsafe.Pointer(cW))
	cMW := C.CString(maxWidth)
	defer C.free(unsafe.Pointer(cMW))
	cM := C.CString(margin)
	defer C.free(unsafe.Pointer(cM))
	cH := C.CString(height)
	defer C.free(unsafe.Pointer(cH))
	cP := C.CString(padding)
	defer C.free(unsafe.Pointer(cP))
	cBS := C.CString(checkboxSize)
	defer C.free(unsafe.Pointer(cBS))

	if res := C.anCaptcha_set_layout(cW, cMW, cM, cH, cP, cBS); int32(res) != 0 {
		return errors.New("failed to set layout: " + getLastError())
	}
	return nil
}

// WarmUp pre-computes assets for a specific style.
//
// This is a CPU-intensive operation and should be called during application
// startup, preferably in a separate goroutine.
func (a *Client) WarmUp(style int) error {
	if res := C.anCaptcha_warm_up(C.int32_t(style)); int32(res) != 0 {
		return errors.New("warm up failed: " + getLastError())
	}
	return nil
}
