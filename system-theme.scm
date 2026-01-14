(require "system-theme-hx.scm")
(require "helix/ext.scm")
(require (only-in "helix/commands.scm" theme))
(require-builtin steel/time)

(provide auto-theme)

(define current-theme #f)

(define (apply-theme dark light)
  (define detected (detect))
  (unless (equal? detected current-theme)
    (set! current-theme detected)
    (if (equal? detected "dark")
        (theme dark)
        (theme light))))

(define (auto-theme dark light [interval-ms 2000])
  ;; Apply once immediately
  (apply-theme dark light)

  ;; Background polling loop
  (spawn-native-thread
   (lambda ()
     (let loop ()
       (time/sleep-ms interval-ms)
       (hx.with-context
        (lambda ()
          (apply-theme dark light)))
       (loop)))))
