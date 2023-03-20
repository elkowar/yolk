(defpackage #:yolk.templating
  (:use #:cl #:alexandria #:defstar #:iterate #:trivia)
  (:export #:process-text))

(in-package #:yolk.templating)

; TODO parse surrounding /es as a regex, then escape the pattern if its _not_ a regex
;      this should be done for _all_ directives
; TODO implement `data' structure for user configuration
; TODO add `access' library for dot-access syntax
; TODO add support for next-line stuff
; TODO add more directives
; TODO add support for conditional inclusion of blocks of text

(defmacro with-plist-vars (plist &body body)
  `(let
    ,(loop for (key val) on plist by #'cddr
           collect `(,(intern (symbol-name key)) ,val))
    ,@body))


(defun* run-replacement-code ((data cons) (input string))
  (declare (ignore data))
  (eval (read-from-string input)))
  ;(eval (with-plist-vars data (read-from-string input))))


(defun* within ((outer string) (inner string))
  (:returns string)
  (str:concat outer inner outer))

(defun* directive-replace-within ((pattern string) (input string) (replacement string))
  (:returns string)
  (cl-ppcre:regex-replace (within pattern ".*?") input (within pattern replacement)))

(defun* directive-replace ((pattern string) (input string) (replacement string))
  (:returns string)
  (cl-ppcre:regex-replace pattern input replacement))

(defun* read-directive ((directive string))
  (:returns (or null (function (string string string) string)))
  (match directive
    ("ri" (function directive-replace-within))
    ("r" (function directive-replace))))


(defclass yolk-call ()
  ((directive :initarg :directive :accessor directive)
   (pattern :initarg :pattern :accessor pattern)
   (replacement :initarg :replacement :accessor replacement)))

(defun* run-call ((data cons) (call yolk-call) (input string))
  (:returns string)
  (let ((dir (read-directive (directive call)))
        (replacement-output (run-replacement-code data (replacement call))))
    (funcall dir (pattern call) input (format nil "~a" replacement-output))))


(defun* read-inline-yolk-call ((input string))
  (:returns (or null yolk-call))
  (cl-ppcre:register-groups-bind (directive pattern replacement)
      (".*\\[yolk:(.*?)\\](.*) *-> *(.*)" input)
    (make-instance 'yolk-call :directive (str:trim directive)
                              :pattern (str:trim pattern)
                              :replacement (str:trim replacement))))

(defun* process-text ((data t) (text string))
  (:returns string)
  (str:unlines
    (iter (for line in (str:lines text))
      (collect
        (if-let (call (read-inline-yolk-call text))
          (run-call data call line)
          line)))))
