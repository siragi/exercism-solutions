(ns squeaky-clean
  (:require [clojure.string :as str]))

(defn clean
  "clean function to
   1. replace any spaces with underscores.
   2. replace control characters (u0000' through 'u001F' or in the range 'u007F' through 'u009F') with the upper case string 'CTRL'
   3. kebab-case to camelCase
   4. omit any characters that are not letters. Note: The underscores must be preserved from the previous step
   5. omit any Greek letters in the range 'α' to 'ω'."
  [s]
  (->
   s
   (str/replace #"\s" "_")
   ;;(str/replace #"\p{Cntrl}" "CTRL") -> test seem ok, but u0080-u009F are not replaced!!  
   (str/replace #"[\u0000-\u001F\u007F-\u009F]" "CTRL")
  ;;  (str/replace #"-(\p{L})" #(str/upper-case (% 1))) ; regex group gives back list with 2 items [all letter], which must be destructured.
   (str/replace #"-(\p{L})" (fn [[_ letter]] (str/upper-case letter)))
   (str/replace #"[[^_\p{L}]α-ω]" ""))) ;; task 4 and 5. 