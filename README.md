# space-sim
Space particle simulation
-----
### Status & TODOs:
- Rendering
  - Simple shape rendering is done
    - Circles & rectangles
  - Simple pixel rendering is done
    - Setting a specific pixel
  - Can convert between pixel coordinates and buffer indices
- Simulation
  - Not going well
  - Need the ability to test the simulation code, make sure it's working
    - Rendering is mostly there
    - Test case w/ two masses did nothing

TODOs:
- Simulation
  - Decide on & impl permanent simulation controls
    - Step the simulation
    - Add an object
  - Decide how to control the simulation
    - How to trigger redraw when window size changes
- Rendering
  -  objects in top right appear are rendered "stuck" in the corner, unsure if they're actually there