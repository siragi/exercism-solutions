(ns difference-of-squares)

(defn sum-of-squares
  ([x] (sum-of-squares x 0))
  ([x acc] ; overloading :-)
   (if (= x 1)
     (+ acc 1)
     (recur (- x 1) (+ acc (* x x))))))

(defn square-of-sum [x] ;; <- arglist goes here
  ;; your code goes here
  (let [sum (reduce + (range (+ x 1)))]
    (* sum sum)))

(defn difference [x] ;; <- arglist goes here
  ;; your code goes here
  (- (square-of-sum x) (sum-of-squares x)))
