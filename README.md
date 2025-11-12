# README

- https://www.desmos.com/calculator/mb0l9dzmws
- discrete and computational geometry Satyan Devadoss

## visualization ideas

- left click to drag points
- left click point to add it to selection
- space to clear selection
- right click(?) to add point
- hovered point should ~glow
- left click to pan
- save and load point set
- draw invariant region for vertices (where it can move without changing the triangulation)
- click on a tool then right click to use the tool

## alg ideas

- consider a complete quadtree (all leaves are filled)
- intersection of bounding boxes of tested triangles to refine search
- if we go to the up-right quadtree, any triangle must be in the down-left most eventual child
    - not really
- THIS IS FALSE: if we have a node whose children are all leaves, then its leaves must have one of two cases for the triangles between them. in particular, the quad they form must be in the triangulation.
- maybe at least one edge between them must be in it?
- consider tris/quads that don't contain points?
- consider bounding boxes of tris / bounding boxes of circumcircles
- look in a direction until we hit a node at the same level as us?
- look out 2*node_width
- maybe: if a path from a cell to another cell crosses >= 3 quadtree lines in the same direction, they can't be an edge
- maybe: if a triangle is big enough, it must be acute
