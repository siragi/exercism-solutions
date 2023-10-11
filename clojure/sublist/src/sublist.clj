(ns sublist)
;;   (:use [clojure.set :only (subset?)]))

(defn sublist?
  "true if A in B"
  [A B]
  (let [lenA (count A)
        lenB (count B)]
    (and
     (>= lenB lenA)
     (or (= A (take lenA B))
         (recur A (rest B))))))
  ;; (cond
  ;;   (< (count B) (count A)) false
  ;;   (= A (take (count A) B)) true
  ;;   :else  (recur A (rest B))))
;; Shorter Alternatives:
  ;; (some #(= A %) (partition (count A) 1 B)))
  ;; (= A (first (drop-while #(not= A %) (partition (count A) 1 B)))))



(defn classify
  "The function classify should either return :sublist, :superlist, :equal or:unequal.
   
   Specifically, a list A is a sublist of list B if by dropping 0 or more elements from the front of B and 0 or more elements from the back of B you get a list that's completely equal to A.

   Examples: 
   A = [1, 2, 3], B = [1, 2, 3, 4, 5], A is a sublist of B
   A = [3, 4, 5], B = [1, 2, 3, 4, 5], A is a sublist of B
   A = [3, 4], B = [1, 2, 3, 4, 5], A is a sublist of B
   A = [1, 2, 3], B = [1, 2, 3], A is equal to B
   A = [1, 2, 3, 4, 5], B = [2, 3, 4], A is a superlist of B
   A = [1, 2, 4], B = [1, 2, 3, 4, 5], A is neither a superlist, sublist nor equal to B : they are unequal
   "
  [A B]
  (cond
    (= A B) :equal
    (sublist? B A) :superlist
    (sublist? A B) :sublist
    :else :unequal))









;; for sets:
;;   (cond
;;     (= A B) :equal
;;     (= (set A) (set B)) :unequal
;;     (subset? (set A) (set B)) :sublist
;;     (subset? (set B) (set A)) :superlist
;;     :else :unequal)










