(ns interest-is-interesting)

(defn interest-rate
  "-3.213% for a negative balance.
   0.5% for a positive balance less than 1000 dollars.
   1.621% for a positive balance greater or equal than 1000 dollars and less than 5000 dollars.
   2.475% for a positive balance greater or equal than 5000 dollars."
  [balance]
  (condp > balance
    0 -3.213
    1000 0.5
    5000 1.621
    2.475))

;; (defn abs [n]
;;   (if (< n 0)
;;     (* n -1)
;;     n))

(defn annual-balance-update
  "gets the interest-rate and uses it to update current balance"
  [balance]
  (->
   (interest-rate balance)
   Math/abs
   bigdec
   (* balance 0.01M)
   (+ balance)))


(defn amount-to-donate
  "you donate twice the tax free amount to charities, rounded down to the nearest whole dollar"
  [balance tax-free-percentage]
  (if (pos? balance)
    (int (* balance 0.02 tax-free-percentage))
    0))
