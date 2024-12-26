use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
enum LogicGateOperation {
    Xor,
    Or,
    And,
}

#[derive(Debug, Clone)]
struct LogicGate {
    in_wire1: String,
    in_wire2: String,
    op: LogicGateOperation,
    out_wire: String,
}

impl LogicGate {
    fn eval(&self, states: &mut HashMap<String, u8>) {
        let in_wire1 = *states.get(&self.in_wire1).unwrap();
        let in_wire2 = *states.get(&self.in_wire2).unwrap();

        let result = match self.op {
            LogicGateOperation::And => in_wire1 & in_wire2,
            LogicGateOperation::Or => in_wire1 | in_wire2,
            LogicGateOperation::Xor => in_wire1 ^ in_wire2,
        };

        states.insert(self.out_wire.clone(), result);
    }
}

fn generate_dot(in_wires: &HashMap<String, u8>, logic_gates: &Vec<LogicGate>) -> String {
    let mut dot = String::from("digraph circuit {\n");
    
    // Add input nodes
    dot.push_str("  // Input nodes\n");
    for (wire, value) in in_wires {
        dot.push_str(&format!("  {} [shape=diamond,label=\"{} ({})\"];\n", 
            wire, wire, value));
    }

    // Add gate nodes and connections
    dot.push_str("\n  // Logic gates and connections\n");
    for (idx, gate) in logic_gates.iter().enumerate() {
        let gate_name = format!("gate_{}", idx);
        let op_symbol = match gate.op {
            LogicGateOperation::And => "AND",
            LogicGateOperation::Or => "OR",
            LogicGateOperation::Xor => "XOR",
        };
        
        // Add gate node
        dot.push_str(&format!("  {} [shape=box,label=\"{}\"];\n", 
            gate_name, op_symbol));
        
        // Add connections
        dot.push_str(&format!("  {} -> {};\n", gate.in_wire1, gate_name));
        dot.push_str(&format!("  {} -> {};\n", gate.in_wire2, gate_name));
        dot.push_str(&format!("  {} -> {};\n", gate_name, gate.out_wire));
    }
    
    dot.push_str("}\n");
    dot
}

fn simulate(in_wires: &HashMap<String, u8>, logic_gates: &Vec<LogicGate>) -> HashMap<String, u8> {
    let mut states = in_wires.clone();
    let mut execution_queue = VecDeque::new();

    let mut index: HashMap<String, Vec<usize>> = HashMap::new();
    logic_gates
        .iter()
        .enumerate()
        .for_each(|(idx, logic_gate)| {
            index
                .entry(logic_gate.in_wire1.clone())
                .and_modify(|idxs| {
                    idxs.push(idx);
                })
                .or_insert_with(|| vec![idx]);
            index
                .entry(logic_gate.in_wire2.clone())
                .and_modify(|idxs| {
                    idxs.push(idx);
                })
                .or_insert_with(|| vec![idx]);

            if states.contains_key(&logic_gate.in_wire1)
                && states.contains_key(&logic_gate.in_wire2)
            {
                execution_queue.push_back(idx);
            }
        });

    while let Some(idx) = execution_queue.pop_front() {
        logic_gates[idx].eval(&mut states);

        if let Some(idxs) = index.get(&logic_gates[idx].out_wire) {
            for &idx2 in idxs {
                let logic_gate_candidate = &logic_gates[idx2];
    
                if states.contains_key(&logic_gate_candidate.in_wire1)
                    && states.contains_key(&logic_gate_candidate.in_wire2)
                {
                    execution_queue.push_back(idx2);
                }
            }
        }
    }

    states
}

fn construct_z_result(states: &HashMap<String, u8>) -> u64 {
    let mut z_results = states
        .iter()
        .filter(|(key, _)| key.starts_with("z"))
        .map(|(key, bit)| (key.clone(), *bit))
        .collect::<Vec<_>>();
    z_results.sort_unstable();

    let mut result = 0;
    for idx in 0..z_results.len() {
        result |= (z_results[idx].1 as u64) << idx;
    }

    result
}

fn read_data(example: &str) -> std::io::Result<(HashMap<String, u8>, Vec<LogicGate>)> {
    let txt = read_to_string(example)?;

    let mut iter = txt.split("\n");

    let mut in_wires = HashMap::new();
    while let Some(line) = iter.next() {
        if line.is_empty() {
            break;
        }

        let split_line = line.split(": ").collect::<Vec<_>>();
        let wire = split_line[0].to_string();
        let bit = if split_line[1] == "1" { 1u8 } else { 0u8 };
        in_wires.insert(wire, bit);
    }

    let mut logic_gates = Vec::new();
    while let Some(line) = iter.next() {
        let line_split = line.split(" ").collect::<Vec<_>>();

        let in_wire1 = line_split[0].to_string();
        let in_wire2 = line_split[2].to_string();
        let out_wire = line_split[4].to_string();
        let op = match line_split[1] {
            "AND" => LogicGateOperation::And,
            "XOR" => LogicGateOperation::Xor,
            "OR" => LogicGateOperation::Or,
            _ => panic!("Unknown logic gate operation"),
        };

        logic_gates.push(LogicGate {
            in_wire1,
            in_wire2,
            op,
            out_wire,
        });
    }

    Ok((in_wires, logic_gates))
}

fn main() -> std::io::Result<()> {
    // let (in_wires, logic_gates) = read_data("example.txt")?;
    // let (in_wires, logic_gates) = read_data("example2.txt")?;
    let (in_wires, logic_gates) = read_data("input.txt")?;

    let states = simulate(&in_wires, &logic_gates);
    let task1 = construct_z_result(&states);
    println!("Task 1: {task1}");

    // dot -Tpng circuit.dot -o circuit.png
    let dot_content = generate_dot(&in_wires, &logic_gates);
    std::fs::write("circuit.dot", dot_content)?;
    println!("Generated circuit.dot");
    
    Ok(())
}

/*

Errors:

hks XOR mbv --> z10
gvm OR smt --> ggn
x39 AND y39 --> twr
bnv XOR pqv --> z39
x17 XOR y17 --> ndw
x17 AND y17 --> jcb
whq AND rmn --> grm
whq XOR rmn --> z32


Answer:
ggn,grm,jcb,ndw,twr,z10,z32,z39


*/