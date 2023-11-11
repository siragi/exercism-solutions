class Temperature
  ABSOLUTE_NULL              = -273.15 # in celsius (0 kelvin)
  FAHRENHEIT_PER_CELSIUS     =     1.8 # in Fahrenheit
  CELSIUS_NULL_IN_FAHRENHEIT =      32 # in Fahrenheit
  ROUNDING_PRECISION         =       1 # positions after decimal point
  TEMPERATURE_SENSOR_UNITS   =       4 # sensors needed to get a functional Temperatur Sensor

  def to_kelvin(celsius)
    celsius - ABSOLUTE_NULL
  end

  def round(celsius)
    celsius.round(ROUNDING_PRECISION)
  end

  def to_fahrenheit(celsius)
    CELSIUS_NULL_IN_FAHRENHEIT + (celsius * FAHRENHEIT_PER_CELSIUS).to_i # converts float to int (flooring)
  end

  def number_missing_sensors(number_of_sensors)
    incomplete_unit = (number_of_sensors % TEMPERATURE_SENSOR_UNITS)
    incomplete_unit == 0 ? 0 : TEMPERATURE_SENSOR_UNITS - incomplete_unit
    # or remainder of negative numbers!
    # (TEMPERATURE_SENSOR_UNITS - number_of_sensors) % TEMPERATURE_SENSOR_UNITS
  end
end
