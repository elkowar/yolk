(defpackage #:yolk.templating
  (:use #:cl #:defstar #:iterate #:trivia)
  (:export #:process-text))

(in-package #:yolk.templating)

; TODO parse surrounding /es as a regex, then escape the pattern if its _not_ a regex
;      this should be done for _all_ directives
; TODO implement `data' structure for user configuration
; TODO add `access' library for dot-access syntax
; TODO add support for next-line stuff
; TODO add more directives
; TODO add support for conditional inclusion of blocks of text

(defparameter *tests*
  '(("bg_color = \"#ebdbb2\" # [yolk:ri] \" -> \"12\""
     . "bg_color = \"12\" # [yolk:ri] \" -> \"12\"")
    ("bg_color = \"#ebdbb2\" # [yolk:ri] \" -> (+ 5 2)"
     . "bg_color = \"7\" # [yolk:ri] \" -> (+ 5 2)")
    ("bg_color = \"#ebdbb2\" # [yolk:r] \".*?\" -> \"\\\"12\\\"\""
     . "bg_color = \"12\" # [yolk:r] \".*?\" -> \"\\\"12\\\"\"")))
    ;("bg_color = \"#ebdbb2\" # [yolk:r] /\".*\"/ -> (concat \"\\\"\" colors.background \"\\\"\")"
    ; . "bg_color = \"#ff0000\" # [yolk:r] /\".*\"/ -> (concat \"\\\"\" colors.background \"\\\"\")")))

(defun* run-replacement-code ((input string))
  (eval (read-from-string input)))


(defun* (within -> string) ((outer string) (inner string))
  (str:concat outer inner outer))

(defun* (directive-replace-within -> string) ((pattern string) (input string) (replacement string))
  (cl-ppcre:regex-replace (within pattern ".*?") input (within pattern replacement)))

(defun* (directive-replace -> string) ((pattern string) (input string) (replacement string))
  (cl-ppcre:regex-replace pattern input replacement))

(defun* (read-directive -> (or null (function (string string string) string)))
        ((directive string))
  (ematch directive
    ("ri" (function directive-replace-within))
    ("r" (function directive-replace))))


(defclass yolk-call ()
  ((directive :initarg :directive :accessor directive)
   (pattern :initarg :pattern :accessor pattern)
   (replacement :initarg :replacement :accessor replacement)))

(defmethod print-object ((c yolk-call) stream)
  (format stream "(yolk-call :directive \"~a\" :pattern \"~a\" :replacement \"~a\")"
          (directive c) (pattern c) (replacement c)))

(defun* (run-call -> string) ((call yolk-call) (input string))
  (let ((dir (read-directive (directive call))))
    (funcall dir (pattern call) input (format nil "~a" (run-replacement-code (replacement call))))))


(defun* (read-inline-yolk-call -> (or null yolk-call)) ((input string))
  (cl-ppcre:register-groups-bind (directive pattern replacement)
      (".*\\[yolk:(.*?)\\](.*) *-> *(.*)" input)
    (make-instance 'yolk-call :directive (str:trim directive)
                              :pattern (str:trim pattern)
                              :replacement (str:trim replacement))))

(defun process-text (data text)
  (declare (ignore data))
  (iter (for line in (str:lines text))
    (collect (run-call (print (read-inline-yolk-call text)) line))))

(defun tests ()
  (format t "~%Running tests...~%")
  (let ((data '()))
    (iter (for (input . expected) in *tests*)
      (let ((actual (process-text data input)))
        (format t "Processing: ~a~%Expected: ~a~%Actual: ~a~%~%" input expected actual)))))

#+(or)
(tests)
