# space-sim
Test bed for rendering with Pixels.rs, a nice little rendering library that gives you a pixel buffer
to get directly drawn on the screen. The end goals here are:
  - Practice using Pixels & get some nice drawing functions
  - Create a particle/physics sim (j with gravity for now, maybe more forces eventually)

I'm mostly happy with the rendering capabilities, not that happy w/ the simulation part, and really need to
work on the controls. 
-----
### Usage & Documentation:
- Control the running program: (Key1 = "1" key in the num row)
  - Key1 => Showcase mode
  - Key2 => Simulation mode
  - Key3 => step the simulation
  - Key4 => play the simulation
  - Key5 => pause the simulation
- Rendering
  - FrameRenderer
-----
### TODOs:
- Rendering
  - Test how robust the current draw functions are
  - Look into text drawing
- Simulation
  - I don't think the gravity correctly applies to all the objects at the same time
  - Muddle with the gravity strength & physical constants
  - Test case with two bodies orbiting each other
  - The simulation steps are relatively uneven & it looks like the objects jitter
- Controls
  - Toggleable overlay w/ text labeling what all the controls do
  - Controls to place an additional object into the simulation while paused
    - controllable size, mass, and (maybe) color