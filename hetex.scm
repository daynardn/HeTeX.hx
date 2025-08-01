(require "hetex-lib.scm")
(require (prefix-in helix. "helix/commands.scm"))
(require "helix/static.scm")

(provide hetex-run)

(define (hetex-run [path "."])
 (helix.run-shell-command "echo '"  (latex-parse (current-highlighted-text!)) "'" ))
