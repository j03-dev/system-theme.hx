(require "system-theme-hx.scm")
(require "helix/ext.scm")
(require (only-in "helix/commands.scm" theme))
(require-builtin steel/time)

(provide auto-theme)

(define (auto-theme dark light [interval-ms 2000])
  (define current-theme (detect))

  (if (equal? current-theme "dark")
      (theme dark)
      (theme light))

  (spawn-native-thread
   (lambda ()
     (let loop ()
       (time/sleep-ms interval-ms)

       (define detected (detect))

       (unless (equal? detected current-theme)
         (set! current-theme detected)

         (hx.with-context
          (lambda ()
            (if (equal? detected "dark")
                (theme dark)
                (theme light)))))

       (loop)))))
