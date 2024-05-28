import { Observable, ReplaySubject, combineLatest } from 'rxjs';
import { map } from 'rxjs/operators';



// Define a generic Node class with typed inputs based on the transformation function
class Node<TFunction extends (...args: any[]) => any> {
  public inputSubjects: ReplaySubject<any>[];
  public output$: Observable<ReturnType<TFunction>>;
  private transformationFunction: TFunction;

  constructor(transformationFunction: TFunction) {
    this.transformationFunction = transformationFunction;
    const numInputs = transformationFunction.length;
    this.inputSubjects = Array.from({ length: numInputs }, () => new ReplaySubject<any>(1));

    this.output$ = combineLatest(this.inputSubjects).pipe(
      map(inputs => {
        const value = this.transformationFunction(...inputs as Parameters<TFunction>);
        console.log(`Processing (${inputs}) through ${transformationFunction}....${value}`);
        return value;
      }));
  }
}

class Graph {
  private nodeAdjacencies: Map<Node<any>, Node<any>[]> = new Map();

  addNode(node: Node<any>, connections: { targetNode: Node<any>, targetInputIndex: number }[] = []) {
    this.nodeAdjacencies.set(node, []);
    connections.forEach(({ targetNode, targetInputIndex }) => {
      this.connectNodes(node, targetNode, targetInputIndex);
    });
  }

  // Connect output of one node to input of another node
  connectNodes(sourceNode: Node<any>, targetNode: Node<any>, targetInputIndex: number = 0) {
    const sourceNodeAdjacencies = this.nodeAdjacencies.get(sourceNode)
    if (sourceNodeAdjacencies == undefined) {
      console.error("Source node not present in graph");
      return;
    }
    if (!this.nodeAdjacencies.has(targetNode)) {
      console.error("Target node not present in graph");
      return;
    }
    // Add target node to the adjacency list of the source node
    sourceNodeAdjacencies.push(targetNode);

    // Subscribe the output of the source node to the input of the target node
    sourceNode.output$.subscribe(output => {
      targetNode.inputSubjects[targetInputIndex].next(output);
    });
  }
}

const graph = new Graph();

const constantNode = new Node((x) => x);
const doubleNode = new Node((x: number) => x * 2);
const addNode = new Node((x, y) => x + y);
const subtractNode = new Node((x, y) => x - y);
// Subscribe to outputs to see the typed results
subtractNode.output$.subscribe(result => console.log("Result from subtractNode:", result));

graph.addNode(constantNode);
graph.addNode(doubleNode);
graph.addNode(addNode);
graph.addNode(subtractNode);

// Explicitly connect nodes
graph.connectNodes(constantNode, doubleNode, 0);
graph.connectNodes(constantNode, addNode, 1);
graph.connectNodes(doubleNode, addNode, 0);
graph.connectNodes(doubleNode, subtractNode, 0);
graph.connectNodes(addNode, subtractNode, 1);



constantNode.inputSubjects[0].next(5)

// const sin: Observable<number> = timer(0, 10) // Emit values every 100 ms
//   .pipe(
//     map((t: number) => Math.sin(t / 1000 * Math.PI * 2) * 0.5 + 0.5), // t / 1000 to slow down the frequency for better visualization
//     throttleTime(1000)
//   );
// sin.subscribe(constantNode.inputSubjects[0])
//
