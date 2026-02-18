// Package main: anCaptcha integration example for net/http.
//
// Setup:
// 1. Put libancaptcha_ffi.so in project root or system path.
// 2. Make sure ancaptcha/ directory has ancaptcha.go wrapper.
// 3. Init module: `go mod init example`.
// 4. Use a consistent 32-byte secret.
//
// Tip: Call WarmUp at startup in a goroutine so images are ready immediately.
package main

import (
	"ancaptcha/ancaptcha"
	"fmt"
	"html/template"
	"log"
	"net/http"
	"strings"
	"time"
)

var tmpl = template.Must(template.ParseGlob("templates/*.html"))
// Use a real 32-byte key from env vars in production.
// Generate with: `openssl rand -hex 32`
var captcha *ancaptcha.Client

const (
	secretLen    = 32
	serverPort   = ":8081"
	readTimeout  = 5 * time.Second
	writeTimeout = 10 * time.Second
	minPathParts = 2
)

/// Initializes the anCaptcha client and triggers asset warming.
func init() {
	secret := make([]byte, secretLen)
	var err error
	captcha, err = ancaptcha.New(secret)
	if err != nil {
		log.Fatalf("Critical init error: %v", err)
	}

	// Warm up asset variations in a background goroutine.
	go func() {
		_ = captcha.WarmUp(ancaptcha.StyleRotate)
		_ = captcha.WarmUp(ancaptcha.StyleSlider)
		_ = captcha.WarmUp(ancaptcha.StylePair)
	}()
}

func main() {
	mux := http.NewServeMux()
	mux.HandleFunc("/", handleIndex)
	mux.HandleFunc("/{kind}/{diff}", handleCaptcha)

	srv := &http.Server{
		Addr:         serverPort,
		Handler:      mux,
		ReadTimeout:  readTimeout,
		WriteTimeout: writeTimeout,
	}

	fmt.Println("Server starting on " + serverPort + "...")
	if err := srv.ListenAndServe(); err != nil {
		fmt.Printf("Server failed: %v\n", err)
	}
}

/// Renders the main index page.
func handleIndex(w http.ResponseWriter, r *http.Request) {
	if r.URL.Path != "/" {
		http.NotFound(w, r)
		return
	}
	_ = tmpl.ExecuteTemplate(w, "index.html", nil)
}

// PageData holds the template variables.
type PageData struct {
	Kind       string
	Difficulty string
	Token      string
	HTML       template.HTML
	CSS        template.CSS
	StatusHTML template.HTML
	ShowForm   bool
}

func capitalize(s string) string {
	if len(s) == 0 {
		return ""
	}
	return strings.ToUpper(s[:1]) + s[1:]
}

/// Renders a captcha challenge within the template layout.
func renderCaptchaPage(w http.ResponseWriter, _ *http.Request, kind, diff string, success bool, errorMsg string) {
	bundle, err := captcha.Generate(kind, diff, errorMsg)
	if err != nil {
		log.Printf("Captcha generation failed: %v", err)
		http.Error(w, "System Error: Failed to load captcha.", http.StatusInternalServerError)
		return
	}

	data := PageData{
		Kind:       capitalize(kind),
		Difficulty: capitalize(diff),
		Token:      bundle.Token,
		HTML:       bundle.HTML,
		CSS:        bundle.CSS,
		ShowForm:   true,
	}

	if success {
		data.StatusHTML = template.HTML(`<div style="background:#d4edda;color:#155724;padding:15px;border-radius:4px;margin-bottom:20px;text-align:center;font-weight:bold;">Comment posted successfully!</div>`) //nolint:gosec
	}

	_ = tmpl.ExecuteTemplate(w, "captcha.html", data)
}

/// Handles challenge routing and submission verification.
func handleCaptcha(w http.ResponseWriter, r *http.Request) {
	parts := strings.Split(strings.Trim(r.URL.Path, "/"), "/")
	if len(parts) < minPathParts {
		http.Error(w, "Invalid path", http.StatusBadRequest)
		return
	}
	kind, diff := parts[0], parts[1]

	if r.Method == "GET" {
		renderCaptchaPage(w, r, kind, diff, false, "")
		return
	}

	if r.Method == "POST" {
		_ = r.ParseForm()

		formData := r.Form.Encode()
		isValid, err := captcha.Verify(formData)

		if err != nil {
			log.Printf("Captcha verification system error: %v", err)
			renderCaptchaPage(w, r, kind, diff, false, "Invalid or expired token")
			return
		}

		if isValid {
			renderCaptchaPage(w, r, kind, diff, true, "")
		} else {
			renderCaptchaPage(w, r, kind, diff, false, "Incorrect answer. Please try again.")
		}
	}
}
