pub struct QuantumCircuit {
    size : i32 // number of qubits
    data : Qubit // will be an array of qubits with length size
}

// these are all generic gates that should 
// inherit from the same class
impl QuantumCircuit {
    fn H(Self, qubit : i32 ) -> Self {
    }

    fn CNOT(Self, qubit1 : i32, qubit2 : i32 ) -> Self {
    }

    fn X(Self, qubit1 : i32) -> Self {
    }
}
