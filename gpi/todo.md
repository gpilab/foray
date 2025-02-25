## System Level
- [x] undo/redo

- [x] save to file
  - [x] load from file

- [x] hot reload nodes
  - [x] notify of node errors
    - [*] Actionable Error Messages. Not cryptic! Ideally point directly to what needs to change.

## External input
- [x] Nodes that load data from file
  - [ ] run from gpi "headless" from cli
  - [*] Load data from cli arguments (How others will use your network. Shouldn't have to edit the network to point to new files)

## Python interop
 
- [ ] create user configuration file

- [ ] load nodes from user configured location
- [ ] Add nodes written from other users via pip install

- [ ] solidfy how the user's python environment will work
 - [ ] make environments reporoducible by default. It should always be simple to share work with others
 - [ ] understand how python package management works, well enough to do this properly


## UI
 - [x] pan
   - [ ] kinetic pan
 - [ ] zoom
 - [x] hotkeys
  - [x] delete node
  - [x] deselect node
  - [x] copy/paste
  - [x] add node
 - [x] duplicate node on command + click
 - [ ] toggle auto reload
 - [ ] visually notify node reloads

 - [ ] Add node UI with user defined hierarchy (assumed from file stucture?)

- [x] render nodes

- [x] select single node
  - [x] select multiple nodes

## Data Model
- [x] execution
  - [x] async execution
  - [x] parallel execution
  - [ ] pause execution
  - [ ] consistent styling for execution state
    - [x] running indicication (vary alpha over time?)
    - [ ] wire fireing indication
          - after node completion, output wire exponential decay of brightness down to base level
    - [ ] unfilled inputs
      - [ ] allow for optional node inputs

- [x] load available nodes
- [x] display available nodes
- [x] create nodes


- [x] render node types differently
- [x] render nodes ontop of one another in the order that they are most recently selected (Current selected node will always be on top)

- [x] wires
  - [x] create wires via click and drag
  - [x] indicate wires that will be deleted when a new wire replaces an old wire  
- [x] multiple inputs/outputs
  - [x] render input/output types differently
  - [x] semantic color for data type 
  - [ ] semantic shape for array shape/dimension

- [ ] restrict node connections to only valid ports
  - [?] and convert arrays of data on wires

- [x] display editable node config
  - [*] Specify config UI from python


## On Canvas Ad-Hoc Visualization 
- [x] efficient image display
- [ ] image display manipulation
  - [ ] floor window level contrast
  - [ ] complex phase vis

## C interface
- [ ] compilation process


## Primary Visualization/Output
- [?] compose widgets from multiple nodes together

# Bugs
- [ ] Node running while it's deleted, results come back, but node is gone. crash on unwrapping node
