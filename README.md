# space-sim
Space particle simulation

### Steps:
1. Create window
   1. Open window
   2. Allow resizing
   3. Make window controls work
2. Draw pixels to window
   1. Specify colors
   2. Specify positions
3. Plan / Make structure for 2d physics


----
### Problems RN:
- The rendering is weird
  - sizing is not what i'd expect
  - i can't get it to draw anything
  - I think i'm lost in the ~~abstraction~~ sauce
-----
### Status & TODOs:
- Rendering
  - Write function to set pixel color from x & y
  - Write function to draw a shape w/ properties at an x, y
    - Probably involve iterating through the whole frame for each shape
    - Will mean draw order maters 
  - Setup correct pixel canvas vs surface texture size (+ see difference)
  - Get a small rendering text case working