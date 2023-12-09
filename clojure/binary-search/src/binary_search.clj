(ns binary-search)

(defn middle [list]
  (let [elements (count list)]
    (quot elements 2)))

(defn search-for
  ([x sorted_list]
   (search-for x (vec sorted_list) 0))

  ([x sorted_list start]
   (assert (seq sorted_list) (format "%d not found" x))
   (let [mid_pos (middle sorted_list)
         mid_val (get sorted_list mid_pos)]
     (cond
       (= mid_val x) (+ start mid_pos)
       (< x mid_val) (recur x (subvec sorted_list 0 mid_pos) start)
       :else (recur x (subvec sorted_list (inc mid_pos)) (inc mid_pos))))))

