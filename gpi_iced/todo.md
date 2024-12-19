
# Window
## System Level
- [*] undo/redo

- [-] save to file
  - [-] load from file

- [-] hot reload nodes
  - [-] visually notify node reloads
  - [-] notify of node errors
    - [*] Actionable Error Messages. Not cryptic! Ideally point directly to what needs to change.
## External Input
- [-] Nodes that load data from file
  - [*] Load data from cli arguments (How others will use your network. Shouldn't have to edit the network to point to new files)

# Canvas
 - [x] pan
   - [-] kinetic pan
 - [-] zoom
## Data generation

## Data manipulation
- [x] execution
  - [*] async execution
  - [!] parallel execution
  - [-] pause execution

- [-] load available nodes
- [-] display available nodes
- [-] create nodes

- [x] render nodes

- [x] select single node
  - [-] select multiple nodes

- [-] render node types differently

- [x] wires
  - [x] create wires via click and drag
  - [x] indicate wires that will be deleted when a new wire replaces an old wire  
- [-] multiple inputs/outputs
  - [-] render input/output types differently

- [-] restrict node connections to only valid ports
  - [-] nd arrays of data on wires

- [-] display editable node config
  - [*] Specify config UI from python


## On Canvas Ad-Hoc Visualization 
- [-] efficient image display/manipulation

# Primary Visualization/Output
- [?] compose widgets from multiple nodes together


# Active Item Notes
## Network Execution
1. load network
2. topo sort. 
3. execute top nodes
4. trigger children


## Node status
- `inert`
  - missing required inputs connections/config
- `unprocessed`
  - has connections, but parent output isn't populated yet
  - could be run, but hasn't yet
- `processing(start_time)`
  - Executor is currently running
- `complete(exec_time)`
  - has output

## Node exectution
1. Nodes start at a status of `inert`
2. If Node is `unprocessed`, continue executing
  - if not, visually highlight what parts are missing
  - in cli mode print a warning/error?
3. Get Inputs (assert that inputs exist and are valid)
4. (Async?) Submit inputs and config to Executor (python)
  a. Start timer
  b. Update node state to `processing`
  c. handle errors
5. receive output
  a. stop timer
  b. update node status to `complete`
