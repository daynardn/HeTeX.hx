(require "hetex-lib.scm")
(require "helix/editor.scm")
(require (prefix-in helix. "helix/commands.scm"))
(require "helix/misc.scm")
(require "helix/ext.scm")
(require "helix/static.scm")
(require-builtin steel/time)
(require-builtin helix/core/text)
(require (prefix-in helix.static. "helix/static.scm"))
(require "helix/components.scm")
(require-builtin helix/core/text as text.)
(require "steel/sync")

(provide hetex-run)

(define (current-path)
  (let* ([focus (editor-focus)]
         [focus-doc-id (editor->doc-id focus)])
    (editor-document->path focus-doc-id)))

(define (hetex-run [path "."])
 (helix.run-shell-command "echo '"  (latex-parse (current-highlighted-text!)) "'" ))
