(require "system-theme-hx.scm")
(require (only-in "helix/commands.scm" theme))

(provide auto-theme)

;;@doc  
;; Automatically switches between dark and light themes based on system preference
;; Usage: (auto-theme "gruvbox" "gruvbox-light")
;;        (auto-theme "catppuccin-mocha" "catppuccin-latte")
(define (auto-theme dark light)
  (if (equal? "dark" (detect))(theme dark) (theme light)))

