// use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    // Construct a HighScores struct, given the 'scores'.
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        // Return all the scores as a slice
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        // Return the latest (last) score

        // a) by copying
        // self.scores.last().cloned()

        // b) by extracting and dereferencing:
        // - If you are not aware that there exists a fn cloned() which does produce Option<u32> -
        match self.scores.last() {
            Some(last) => Some(*last),
            None => None,
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        // Return the highest score

        // a) by copying or cloning
        //self.scores.iter().max().cloned()

        // b) by extracting and dereferencing:
        match self.scores.iter().max() {
            Some(max) => Some(*max),
            None => None,
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        // Return 3 highest scores

        // a) vec - sort - iter - dereferencing via closure ( instead of using copied() )
        /* let mut scores_vec = self.scores.to_vec();
        scores_vec.sort();
        scores_vec.iter().rev().take(3).map(|hs| *hs).collect() */

        // b) my preferred: vec - sort - reverse - truncate
        /* let mut scores_vec: Vec<u32> = self.scores.to_vec();
        scores_vec.sort_unstable(); // not sort(): to be fast (no need to have correct order of equal elements)
        scores_vec.reverse(); // to get the best scores at the beginning, since ...
        scores_vec.truncate(3); // truncate gets rid of the end.
        scores_vec */

        // c) vec - sort - drain the highest scores - reverse them
        let mut scores_vec: Vec<u32> = self.scores.to_vec();
        scores_vec.sort_unstable(); // not sort(): to be fast (no need to have correct order of equal elements)

        match scores_vec.len() {
            0 | 1 => scores_vec,
            2 | 3 => {
                scores_vec.reverse();
                scores_vec
            }
            _ => {
                let mut drained: Vec<u32> = scores_vec.drain(scores_vec.len() - 3..).collect();
                drained.reverse();
                drained
            }
        }

        // d) Version using a BinaryHeap (which pop()s always the highest)
        // extra vector to fill with the top three.
        /* let mut heap = BinaryHeap::from(self.scores.to_vec());
        let mut top_three: Vec<u32> = vec![];
        for _ in 0..3 {
            match heap.pop() {
                Some(highest) => top_three.push(highest),
                None => {}
            }
        }
        top_three */
    }
}
