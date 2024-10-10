mod primitive;
pub use primitive::{Bool, Complex, Integer, Real, Str};
pub use primitive::{Primitive, PrimitiveType};

mod io_port;
pub(crate) use io_port::{InputPort, OutputPort, PortName};
pub(crate) type NodeIndex = petgraph::graph::NodeIndex;

mod port_def;
pub use port_def::{ArrayValue, Port, PortType, Shape};
