(defpackage #:yolk
  (:use #:cl)
  (:export #:main))

(in-package #:yolk)

(defun main ()
  (format t "hi~%"))

(in-package #:cl-user)
(defun main ()
  (yolk:main))
