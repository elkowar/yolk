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
