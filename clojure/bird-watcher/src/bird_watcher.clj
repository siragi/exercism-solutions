(ns bird-watcher)

(def last-week 
  [0 2 5 3 7 8 4])

(defn today [birds]
  (last birds))

(defn inc-bird [birds]
  (conj (pop birds) (inc (today birds))))

(defn day-without-birds? [birds]
  (not= (filter (fn [cnt] (= cnt 0)) birds) []))

(defn n-days-count [birds n]
  (reduce + (take n birds)))

(defn busy-days [birds]
  (count (filter (fn [cnt] (>= cnt 5)) birds)))

(defn odd-week? [birds]
  (= birds [1 0 1 0 1 0 1]))
