(defpackage #:yolk.templating
  (:use #:cl #:defstar #:iterate #:trivia #:defclass-std)
  (:export #:process-text))

(in-package #:yolk.templating)


(defparameter *tests*
  '(("bg_color = \"#ebdbb2\" # [yolk:ri] \" -> 12"
     . "bg_color = \"12\" # [yolk:ri] \" -> 12")
    ("bg_color = \"#ebdbb2\" # [yolk:r] /\".*\"/ -> (concat \"\\\"\" colors.background \"\\\"\")"
     . "bg_color = \"#ff0000\" # [yolk:r] /\".*\"/ -> (concat \"\\\"\" colors.background \"\\\"\")")))


(defun process-text (data text)
  (declare (ignore data))
  text)

(defun* (surrounded -> string) ((outer string) (inner string))
  (str:concat outer inner outer))

(defun* (directive-replace-in -> string) ((pattern string) (input string) (replacement string))
  (regex-replace (surrounded pattern ".*?") input (surrounded pattern replacement)))

(defun* (read-directive -> (or null (function (string string string) string)))
        ((directive string))
  (ematch directive
    ("ri" (function directive-replace-in))))


(defclass/std yolk-call () ((directive pattern replacement)))
(defmethod print-object ((c yolk-call) stream)
  (format stream "(yolk-call :directive \"~a\" :pattern \"~a\" :replacement \"~a\")"
          (directive c) (pattern c) (replacement c)))

(defun* (run-call -> string) ((call yolk-call) (input string))
  (let ((dir (read-directive (directive call))))
    (funcall dir (pattern call) input (replacement call))))

(run-call (read-inline-yolk-call "x = \"foo\" # [yolk:ri] \" -> lmao") "x = \"foo\"")

(defun* (read-inline-yolk-call -> (or null yolk-call)) ((input string))
  (cl-ppcre:register-groups-bind (directive pattern replacement)
      (".*\\[yolk:(.*?)\\](.*) *-> *(.*)" input)
    (make-instance 'yolk-call :directive (str:trim directive)
                              :pattern (str:trim pattern)
                              :replacement (str:trim replacement))))


(defun tests ()
  (format t "~%Running tests...~%")
  (let ((data '()))
    (iter (for (input . expected) in *tests*)
      (let ((actual (process-text data input)))
        (format t "Expected: ~a~%Actual: ~a~%" expected actual)))))

#+(or)
(tests)
