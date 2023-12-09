(ns matching-brackets)

(def brackets "()[]{}")

(defn valid? [text]
  (or (empty? text)

      (let [filtered-text (filter (set brackets) text)
            pairs (apply hash-map brackets)]
        (loop [[open & stillopen :as openings] (list)
               [current & remaining] filtered-text]
          (cond
            ;; if we got down the list until its empty it depends on whether we still have open brackets or not.
            (nil? current) (nil? open)
            ;; if opening bracket (contains? looks in the keys of the pair var) 
            (contains? pairs current) (recur (cons current openings) remaining)
            ;; if closing bracket, it should be the one that corresponds to that open bracket 
            (= (pairs open) current) (recur stillopen remaining)
            :else false)))))
