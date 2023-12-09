(ns say)

(def dict {0 "zero"
           1 "one"
           2 "two"
           3 "three"
           4 "four"
           5 "five"
           6 "six"
           7 "seven"
           8 "eight"
           9 "nine"
           10 "ten"
           11 "eleven"
           12 "twelve"
           13 "thirteen"
           14 "fourteen"
           15 "fifteen"
           16 "sixteen"
           17 "seventeen"
           18 "eighteen"
           19 "nineteen"
           20 "twenty"
           30 "thirty"
           40 "forty"
           50 "fifty"
           60 "sixty"
           70 "seventy"
           80 "eighty"
           90 "ninety"
           100 "hundred"
           1000 "thousand"
           1000000 "million"
           1000000000 "billion"})

(defn spell
  ([hundreds tens ones]

   (cond
     (= hundreds tens ones 0) ""
     (= tens ones 0)  (apply str (get dict hundreds) " " (get dict 100))
     (= tens 0) (apply str (get dict hundreds) " " (get dict 100) " " (spell ones))
     :else (apply str (get dict hundreds) " " (get dict 100) " " (spell tens ones))))

  ([tens ones]
   (apply str
          (cond
            (= ones 0) (get dict (* 10 tens))
            (< tens 2)  (get dict (+ ones (* 10 tens)))
            :else (apply str (get dict (* 10 tens)) "-" (spell ones)))))

  ([ones]
   (apply str (get dict ones))))

(spell 2 3)

(defn tripplets [num]
  (loop [n num
         result (list)]
    (let [hundreds (quot (rem n 1000) 100)
          tens (quot (rem n 100) 10)
          ones (rem n 10)
          next_n (quot n 1000)]
      (cond
        (> next_n 0) (recur next_n (conj result (spell hundreds tens ones)))
        (> hundreds 0) (recur 0 (conj result (spell hundreds tens ones)))
        (> tens 0) (recur 0 (conj result (spell tens ones)))
        (> ones 0) (recur 0 (conj result (spell ones)))
        ;; (= ones 0) (recur 0 (conj result (spell ones)))
        :else result))))

(tripplets 834)
(tripplets 223234400)
(tripplets 1000)
(tripplets 0)
(quot 0 1000)

(defn inverse [lst]
  (->> lst
       (into [])
       (reverse)))


(apply str (cons [(inverse (tripplets 834))] [(get dict (* 1 1000))]))

;; (conj (get dict (* 1 1000)) " " (inverse (tripplets 834)))

(defn number [num]
  (loop [[tripplet & remaining] (inverse (tripplets num))
         cnt 0
         result []]
    (cond
      (= tripplet "") (recur remaining (inc cnt) result)
      (and tripplet (> cnt 0)) (recur remaining (inc cnt) (conj result [tripplet " " (get dict (int (Math/pow 1000 cnt))) " "]))
      tripplet (recur remaining (inc cnt) (conj result [tripplet]))
      ;; (and (nil? tripplet) (empty? result)) ""
      :else (apply str (flatten (reverse result))))))


(number 1000000)
(number 234)
(number 34599850)

(inverse (tripplets 0))
(tripplets 0)


