# connect-four

## Refactors

- Implement graphics as a trait. Move Raylib into an implementation of that trait.  
- Optimize board representation. Bitarrays could be used to represent pieces.
- Most of the View object, tile positions, and tile sizes can be precomputed values.
