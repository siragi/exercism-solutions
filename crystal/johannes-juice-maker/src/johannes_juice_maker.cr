# To Do: Define the class JuiceMaker
class JuiceMaker
  PER_MINUTE_USE = 5

  def self.debug_light_on?
    true
  end

  def initialize(@fluid : UInt32)
    @running = false
  end

  def start
    @running = true
  end

  def running? @running end

  def add_fluid(fluid : UInt32)
    @fluid += fluid
  end

  def stop(runtime : UInt32)
    @running = false

    used_fluid = runtime * PER_MINUTE_USE
    @fluid -= @fluid < used_fluid ? @fluid : used_fluid 
  end
end
