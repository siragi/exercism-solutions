(ns elyses-destructured-enchantments)


;; https://clojure.org/guides/destructuring#_where_to_destructure
;; When defining a function in clojure, destructuring can be applied on the incoming parameters, just like in a let..
(defn first-card
  "Returns the first card from deck."
  [[first]] ;; destructuring of the deck
  first)
;; (defn first-card
;;   "Returns the first card from deck."
;;   [deck]
;;   (let [[x & _] deck]
;;     x))
(defn second-card
  "Returns the second card from deck."
  [[_ second]]
  second)
;; (defn second-card
;;   "Returns the second card from deck."
;;   [deck]
;;   (let [[_ y & _] deck]
;;     y))


(defn swap-top-two-cards
  "Returns the deck with first two items reversed."
  [[x y & xs]]
  (concat [y x] xs))
;; (defn swap-top-two-cards
;;   "Returns the deck with first two items reversed."
;;   [deck]
;;   (let [[x y & xs] deck]
;;     (concat [y x] xs)))


(defn discard-top-card
  "Returns a sequence containing the first card and
   a sequence of the remaining cards in the deck."
  [deck]
  (let [[x & xs] deck]
    [x xs]))


(def face-cards
  ["jack" "queen" "king"])

(defn insert-face-cards
  "Returns the deck with face cards between its head and tail."
  [deck]
  (if (empty? deck)
    face-cards
    (let [[x & xs] deck]
      (cons x (concat face-cards xs)))))

