# GPI V2

Prototype of a graphical programming enviroment for scientific computation.


## Features

### UI
Nodes can be placed on an infinite canvas.
You can freely move in any direction and place nodes, figures, or labels anywhere.

Nodes can be connected via their in-ports (top) and out-port (bottom)

Multi-select, copy/paste, and layout tools are all available

TODO:
[ ] - Node UI interactions that make editing paramters quick and easy
[ ] - Detailed node configuration editing
[ ] - make any python node defined automatically appear in the UI
[ ] - debugging/logging information surface to the user
[ ] - indication of when nodes fire, and how long they run
[ ] - export network as a file

### CLI
The cli can be used either to run networks without the gui,
as well as a development tool to verify user authored nodes
are working correctly.


TODO:
[ ] - run a single node with supplied paramaters
[ ] - run a network with a supplied file
[ ] - run tests on user's nodes
[ ] - check for any python node type errors

### Core

TODO:
[ ] - process network in rust, sending updates to the UI each time a node finishes it's compute function
[ ] - reject networks that don't meet requirements (mis-matched port types etc.)


### Python API

TODO:
[ ] - init function
[ ] - compute function
[ ] - view function



## Status
Early prototype


## gotchas
1. Use "onPointerDown", rather than "onClick" or "onMouseDown", this works better with the UI framework.
  - Also make sure to use "stopEventPropogation" when applicable

