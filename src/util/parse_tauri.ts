import { createDynamicNode } from "../shapes/node/nodeDefinitions";
import { NodeInputs, NodeOutputs } from "../shapes/node/nodeType";
import { PortDataTypeLabel, PortTypeLabels, singleOutput, isPortDataTypeLabel } from "../shapes/node/portDefinition";

export type serializedPort = Record<string, PortDataTypeLabel>


export type SerializedPythonNode = {
  node_type: string,
  source: {
    Local: { path: string, file_contents: string },
    Remote: Record<string, never>
  },
  config: Record<string, string>,
  input_types: serializedPort,
  output_types: serializedPort
}

export function parse_nodes(local_python_nodes: SerializedPythonNode[]) {
  console.log("Parsing python nodes:")
  return local_python_nodes.map((node) => {
    const { node_type, source, config, input_types, output_types } = node
    console.log(source.Local.path)

    const in_ports = parseInPorts(input_types)
    const out_ports = parseOutPorts(output_types)

    return createDynamicNode(node_type, config, in_ports, out_ports)
  });
}

function parseInPorts(port_types: serializedPort): NodeInputs {
  return Object.entries(port_types).reduce<NodeInputs>(
    (obj: NodeInputs, [name, input]): NodeInputs => {
      if (!isPortDataTypeLabel(input)) {
        input = Object.keys(input)[0] as PortDataTypeLabel
      }
      if (isPortDataTypeLabel(input)) {
        obj[name] = {
          name: name,
          ioType: "in",
          dataType: input
        }
      }
      else {
        console.warn(`Encountered unexpected input type: ${JSON.stringify(input)}
Expected input types: ${PortTypeLabels.toString()}`)
      }
      return obj
    }, {})
}

function parseOutPorts(port_types: serializedPort): NodeOutputs {
  if (Object.entries(port_types).length != 1) {
    console.error("Multiple outputs not supported yet!")
  }
  let port_type = port_types["out"]

  if (!isPortDataTypeLabel(port_type)) {
    port_type = Object.keys(port_type)[0] as PortDataTypeLabel
  }
  if (isPortDataTypeLabel(port_type)) {
    return {
      out: {
        name: "out",
        ioType: "out",
        dataType: port_type
      }
    }
  } else {
    console.warn(`Encountered unexpected output type: ${JSON.stringify(port_type)}
Expected output types: ${PortTypeLabels.toString()}`)
  }
  // TODO: handle error ports, display to user?
  return singleOutput(["Real"])
}
