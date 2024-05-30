type Labels<T> = keyof T;
interface PortDataTypes {
  number: number;
  string: string;
  boolean: boolean;
}

type NodeDef<T extends [string, Labels<PortDataTypes>][], Output extends keyof PortDataTypes> = {
  compute: (...args: { [k in keyof T]: PortDataTypes[T[k][1]] }) => PortDataTypes[Output];
  inputs: { [k in keyof T]: T[k] };
  outputType: Output;
}

class NodeBuilder<T extends [string, Labels<PortDataTypes>][] = any[], Output extends keyof PortDataTypes = any> {
  private computeFunction?: (...args: any[]) => PortDataTypes[Output];
  private inputsArray?: T;
  private outputType?: Output;

  public setComputeFunction<F extends (...args: { [k in keyof T]: PortDataTypes[T[k][1]] }) => PortDataTypes[Output]>(fn: F) {
    this.computeFunction = fn;
    return this as unknown as NodeBuilder<T, Output>;
  }

  public setInputs<I extends [string, Labels<PortDataTypes>][]>(inputs: I) {
    this.inputsArray = inputs as unknown as T;
    return this as unknown as NodeBuilder<I, Output>;
  }

  public setOutputType<O extends keyof PortDataTypes>(output: O) {
    this.outputType = output as unknown as O;
    return this as unknown as NodeBuilder<T, O>;
  }

  public build(): NodeDef<T, Output> {
    if (!this.computeFunction || !this.inputsArray || !this.outputType) {
      throw new Error("Builder is not fully initialized.");
    }
    return {
      compute: this.computeFunction,
      inputs: this.inputsArray,
      outputType: this.outputType
    };
  }
}

// Example usage:
const builder = new NodeBuilder()
  .setInputs([["a", "number"], ["b", "number"], ["c", "string"]])
  .setOutputType("number")
  .setComputeFunction((a: number, b: number, c: string) => {
    console.log(a, b, c);
    return a + b; // Just an example computation
  });

const node = builder.build();
console.log(node);
