// The actual length of one complete orbit of the Earth around the sun is closer to 365.256 days (1 sidereal year).
// The Gregorian calendar has, on average, 365.2425 days.
// While not entirely accurate, 365.25 is the value used in this exercise.
pub const Planet = enum {
    // Planets
    mercury,
    venus,
    earth,
    mars,
    jupiter,
    saturn,
    uranus,
    neptune,
    //  Orbital period in Earth Years per planet
    const orbital_period_in_earth_years = [8]f64{ 0.2408467, 0.61519726, 1.0, 1.8808158, 11.862615, 29.447498, 84.016846, 164.79132 };

    const earth_year_in_seconds: f64 = 365.25 * 24 * 60 * 60;

    pub fn age(self: Planet, seconds: usize) f64 {
        const age_in_earth_years: f64 = @as(f64, @floatFromInt(seconds)) / earth_year_in_seconds;
        return age_in_earth_years / orbital_period_in_earth_years[@intFromEnum(self)];
    }
};
