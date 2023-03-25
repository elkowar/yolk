(defpackage #:yolk/test
  (:use #:cl #:fiveam #:yolk))

(in-package #:yolk/test)

(def-suite yolk
  :description "Test yolk")

(def-suite* templating
  :description "test templating logic"
  :in yolk)

(defparameter *test-data* '(:example-value 1 :colors '(:background "#282828")))

(test make-eval-arg-works
  (is (equal '(let ((a 1)
                    (b '(:c 1)))
               (declare (ignorable a))
               (declare (ignorable b))
               0)
             (yolk.templating::make-eval-arg '(:a 1 :b '(:c 1)) "0"))))

(test directive-replace-within-works
  (is (string= "'bar'" (yolk.templating::directive-replace-within "'" "'foo'" "bar"))))

(test directive-replace-works
  (is (string= "bar" (yolk.templating::directive-replace "f.o" "foo" "bar"))))

(test read-inline-yolk-call-works
  (let ((result (yolk.templating::read-inline-yolk-call "foo = 'abc' # [yolk:ri] ' -> xyz")))
    (is (string= "ri" (yolk.templating::directive result)))
    (is (string= "'" (yolk.templating::pattern result)))
    (is (string= "xyz" (yolk.templating::replacement result)))))

(test read-inline-yolk-call-returns-nil
  (is (null (yolk.templating::read-inline-yolk-call "not a valid input"))))

(test process-text-returns-unmodified-when-no-directive
  :description "process-text returns its input unmodified when no directive is found"
  (is (string= "foo bar baz"
               (yolk.templating:process-text *test-data* "foo bar baz"))))

(test process-text-ri
  :description "process-text supports ri directive"
  (is (string= "bg_color = \"12\" # [yolk:ri] \" -> \"12\""
               (yolk.templating:process-text *test-data* "bg_color = \"#ebdbb2\" # [yolk:ri] \" -> \"12\""))))

(test process-text-r
  :description "process-text supports r directive"
  (is (string= "bg_color = \"12\" # [yolk:r] \".*?\" -> \"\\\"12\\\"\""
               (yolk.templating:process-text *test-data* "bg_color = \"12\" # [yolk:r] \".*?\" -> \"\\\"12\\\"\""))))

(test process-text-runs-code
  :description "process-text can run lisp expressions"
  (is (string= "bg_color = \"7\" # [yolk:ri] \" -> (+ 5 2)"
               (yolk.templating:process-text *test-data* "bg_color = \"#ebdbb2\" # [yolk:ri] \" -> (+ 5 2)"))))

(test process-text-can-access-state
  :description "process-text can access user defined state in lisp expressions"
  (is (string= "bg_color = \"1\" # [yolk:ri] \" -> example-value"
               (yolk.templating:process-text *test-data* "bg_color = \"#ebdbb2\" # [yolk:ri] \" -> example-value"))))

(test process-text-can-access-nested-state
  :description "process-text can access nested user defined state in lisp expressions"
  (is (string= "bg_color = \"#282828\" # [yolk:ri] \" -> (getf colors :background)"
               (yolk.templating:process-text *test-data* "bg_color = \"#ff0000\" # [yolk:ri] \" -> (getf colors :background)"))))
