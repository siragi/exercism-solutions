class LeibnizReihe
  def self.comp(terms)
    k = 0
    ans = 0
    while k < terms
      ans += (-1) ** k / (2 * k + 1)
      k += 1
    end
    puts ans * 4 # ans = pi/4
  end
end

LeibnizReihe.comp 3
LeibnizReihe.comp 6
LeibnizReihe.comp 7
LeibnizReihe.comp 10
LeibnizReihe.comp 20
LeibnizReihe.comp 30
LeibnizReihe.comp 40
LeibnizReihe.comp 50
LeibnizReihe.comp 60
LeibnizReihe.comp 100
LeibnizReihe.comp 1000
LeibnizReihe.comp 10000
