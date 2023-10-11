(ns annalyns-infiltration)

(def can-fast-attack? ;; no fn necessary since its equivalent to existung function not. instead its can-fast-attack? is a var now that holds not function (sort of alias)
  "Returns true if a fast-attack can be made, false otherwise.
   
   Fast attack: a fast attack can be made if the knight is sleeping, as it takes time for him to get his armor on, so he will be vulnerable."

  not)


(defn can-spy? ;; again no function needed, can-spy? is essentially or but or is apparently no function but a macro, so it cannot be passed directly - so i let it be a function definition
  "Returns true if the kidnappers can be spied upon, false otherwise.
   
   Spy: the group can be spied upon if at least one of them is awake. Otherwise, spying is a waste of time."
  [knight-awake? archer-awake? prisoner-awake?]
  (or knight-awake? archer-awake? prisoner-awake?))


(defn can-signal-prisoner?
  "Returns true if the prisoner can be signalled, false otherwise.
   
   Signal prisoner: the prisoner can be signalled using bird sounds if the prisoner is awake and the archer is sleeping, as archers are trained in bird signaling so they could intercept the message."
  [archer-awake? prisoner-awake?]
  (if archer-awake?
    false
    prisoner-awake?))


(defn can-free-prisoner?
  "Returns true if prisoner can be freed, false otherwise.
   
   Free prisoner: Annalyn can try sneaking into the camp to free the prisoner. This is a risky thing to do and can only succeed in one of two ways:
   If Annalyn has her pet dog with her she can rescue the prisoner if the archer is asleep. The knight is scared of the dog and the archer will not have time to get ready before Annalyn and the prisoner can escape.
   If Annalyn does not have her dog then she and the prisoner must be very sneaky! Annalyn can free the prisoner if the prisoner is awake and the knight and archer are both sleeping, but if the prisoner is sleeping they can't be rescued: the prisoner would be startled by Annalyn's sudden appearance and wake up the knight and archer.
"
  [knight-awake? archer-awake? prisoner-awake? dog-present?]
  (or (and dog-present? (not archer-awake?))
      (and (not (or archer-awake? knight-awake?)) prisoner-awake?)))

  ;; (if dog-present?
  ;;   (not archer-awake?)
  ;;   (if (or archer-awake? knight-awake?)
  ;;     false
  ;;     prisoner-awake?)))

