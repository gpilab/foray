import { Observable, Subject, combineLatest } from 'rxjs';
import { map } from 'rxjs/operators';

class Node {
  public inputSubjects: Subject<any>[] = [];
  public output$: Observable<any>;
  public transformationFunction: Function;

  constructor(transformationFunction: Function, numberOfInputs: number) {
    this.transformationFunction = transformationFunction;
    for (let i = 0; i < numberOfInputs; i++) {
      this.inputSubjects.push(new Subject<any>());
    }

    // Combine all inputs into a single observable if more than one input is expected
    if (numberOfInputs > 1) {
      this.output$ = combineLatest(this.inputSubjects).pipe(
        map(inputs => this.transformationFunction(...inputs))
      );
    } else {
      this.output$ = this.inputSubjects[0].pipe(
        map(input => this.transformationFunction(input))
      );
    }
  }

  emit(inputIndex: number, value: any) {
    if (inputIndex < this.inputSubjects.length) {
      this.inputSubjects[inputIndex].next(value);
    }
  }
}

class Graph {
  private nodes: Node[] = [];

  addNode(node: Node) {
    this.nodes.push(node);

    // If there's a previous node, connect its output to the input of the new node
    if (this.nodes.length > 1) {
      const previousNode = this.nodes[this.nodes.length - 2];
      previousNode.output$.subscribe({
        next: output => {
          // If the new node has multiple inputs, this needs adjustment
          node.inputSubjects[0].next(output);  // Assumes single-input scenario for simplicity
        }
      });
    }
  }
}

function double(x: number): number {
  return x * 2;
}

function add(x: number, y: number): number {
  console.log("adding nums", x, y)
  return x + y;
}

const graph = new Graph();
const doubleNode = new Node(double, 1);
graph.addNode(doubleNode);

// Assumes addNode expects two inputs; second input needs to be handled separately
const addNode = new Node(add, 2);
graph.addNode(addNode);
console.log(addNode.inputSubjects)

doubleNode.emit(0, 5);  // Output should be 10, expected to be input to addNode
addNode.emit(1, 3);     // Combined with 10, should output 13
addNode.emit(0, 3);     // Combined with 10, should output 13

addNode.output$.subscribe(result => console.log("Result from addNode:", result));
