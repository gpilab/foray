import { createDynamicNode } from "../shapes/node/nodeDefinitions";
import { NodeInputs, NodeOutputs } from "../shapes/node/nodeType";
import { isPortDataTypeLabel, PortTypeLabels, singleOutput } from "../shapes/node/portDefinition";


export type SerializedPython = {
  source: {
    Local: { path: string, file_contents: string },
    Remote: Record<string, never>
  },
  config: Record<string, string>,
  input_types: string[],
  output_types: string[]
}
export function parse_nodes(local_python_nodes: SerializedPython[]) {
  console.log("Parsing python nodes:")
  return local_python_nodes.map((node) => {
    const { source, config, input_types, output_types } = node
    console.log(source.Local.path)

    const in_ports = parseInPorts(input_types)
    const out_ports = parseOutPorts(output_types)

    return createDynamicNode(config, in_ports, out_ports)
  });
}

function parseInPorts(port_types: string[]): NodeInputs {
  return port_types.reduce<NodeInputs>(
    (obj: NodeInputs, input: string, i: number): NodeInputs => {
      if (isPortDataTypeLabel(input)) {
        obj[i.toString()] = {
          name: i.toString(),
          ioType: "in",
          dataType: input
        }
      }
      else {
        console.warn(`Encountered unexpected input type: ${input}
Expected input types: ${PortTypeLabels.toString()}`)
      }
      return obj
    }, {})
}

function parseOutPorts(port_types: string[]): NodeOutputs {
  if (port_types.length != 1) {
    console.error("Multiple outputs not supported yet!")
  }
  const port_type = port_types[0]
  if (isPortDataTypeLabel(port_type)) {
    return {
      out: {
        name: "out",
        ioType: "out",
        dataType: port_type
      }
    }
  } else {
    console.warn(`Encountered unexpected output type: ${port_type}
Expected output types: ${PortTypeLabels.toString()}`)
  }
  // TODO: handle error ports, display to user?
  return singleOutput("number")
}
