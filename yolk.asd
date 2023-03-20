(asdf:defsystem "yolk"
  :description "A dotfile management and templating system"
  :version "0.0.1"
  :author "ElKowar"
  :license "MIT"
  :depends-on (:alexandria :str :trivia :defstar :iterate :cl-ppcre)
  :components ((:module "yolk"
                :pathname "src"
                :components ((:file "main"
                              :depends-on ("templating"))
                             (:file "templating")))))


(asdf:defsystem "yolk/test"
  :description "Test suite for the yolk system"
  :author "ElKowar"
  :depends-on (:yolk :fiveam)
  :components ((:module "tests" :components ((:file "test-yolk"))))
  :perform (test-op (op c)
                    (symbol-call :fiveam :run-all-tests)))
