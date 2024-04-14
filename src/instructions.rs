/// All instructions in an enum that are easy to use. Essentially decode.
#[derive(Debug)]
pub enum Instruction {
    // Loads
    Lb { rd: u32, rs1: u32, imm: u32 },
    Lh { rd: u32, rs1: u32, imm: u32 },
    Lw { rd: u32, rs1: u32, imm: u32 },

    Lbu { rd: u32, rs1: u32, imm: u32 },
    Lhu { rd: u32, rs1: u32, imm: u32 },

    // Stores
    Sb { rs1: u32, rs2: u32, imm: u32 },
    Sh { rs1: u32, rs2: u32, imm: u32 },
    Sw { rs1: u32, rs2: u32, imm: u32 },

    // Shifts
    Sll { rd: u32, rs1: u32, rs2: u32 },
    Slli { rd: u32, rs1: u32, shamt: u32 },
    Srl { rd: u32, rs1: u32, rs2: u32 },
    Srli { rd: u32, rs1: u32, shamt: u32 },
    Sra { rd: u32, rs1: u32, rs2: u32 },
    Srai { rd: u32, rs1: u32, shamt: u32 },

    // Arithmetic
    Add { rd: u32, rs1: u32, rs2: u32 },
    Addi { rd: u32, rs1: u32, imm: u32 },
    Sub { rd: u32, rs1: u32, rs2: u32 },
    Subi { rd: u32, rs1: u32, imm: u32 },
    Lui { rd: u32, imm: u32 },
    Auipc { rd: u32, imm: u32 },

    // Logical
    Xor { rd: u32, rs1: u32, rs2: u32 },
    Xori { rd: u32, rs1: u32, imm: u32 },
    Or { rd: u32, rs1: u32, rs2: u32 },
    Ori { rd: u32, rs1: u32, imm: u32 },
    And { rd: u32, rs1: u32, rs2: u32 },
    Andi { rd: u32, rs1: u32, imm: u32 },

    // Compare
    Slt { rd: u32, rs1: u32, rs2: u32 },
    Slti { rd: u32, rs1: u32, imm: u32 },
    Sltu { rd: u32, rs1: u32, rs2: u32 },
    Sltiu { rd: u32, rs1: u32, imm: u32 },

    // Branches
    Beq { rs1: u32, rs2: u32, imm: u32 },
    Bne { rs1: u32, rs2: u32, imm: u32 },
    Blt { rs1: u32, rs2: u32, imm: u32 },
    Bge { rs1: u32, rs2: u32, imm: u32 },
    Bltu { rs1: u32, rs2: u32, imm: u32 },
    Bgeu { rs1: u32, rs2: u32, imm: u32 },

    // Jumps
    Jal { rd: u32, imm: u32 },
    Jalr { rd: u32, rs1: u32, imm: u32 },

    // // Sync
    // Fence,
    // FenceI,

    // Illegal instruction
    Ill,
}
fn parse_r_type(inst: u32) -> (u32, u32, u32, u32, u32, u32) {
    let funct7 = (inst >> 25) & 0x7F;
    let rs2 = (inst >> 20) & 0x1F;
    let rs1 = (inst >> 15) & 0x1F;
    let funct3 = (inst >> 12) & 0x7;
    let rd = (inst >> 7) & 0x1F;
    let opcode = inst & 0x7F;
    (funct7, rs2, rs1, funct3, rd, opcode)
}
fn parse_i_type(inst: u32) -> (u32, u32, u32, u32, u32) {
    let imm = (inst >> 20) & 0xFFF;
    let rs1 = (inst >> 15) & 0x1F;
    let funct3 = (inst >> 12) & 0x7;
    let rd: u32 = (inst >> 7) & 0x1F;
    let opcode = inst & 0x7F;
    (imm, rs1, funct3, rd, opcode)
}
fn parse_s_type(inst: u32) -> (u32, u32, u32, u32, u32) {
    let imm = ((inst >> 20) & 0x7F) | ((inst >> 7) & 0x1F);
    let rs2 = (inst >> 20) & 0x1F;
    let rs1 = (inst >> 15) & 0x1F;
    let funct3 = (inst >> 12) & 0x7;
    let opcode = inst & 0x7F;
    (imm, rs2, rs1, funct3, opcode)
}
// TODO: FIX IMMEDIATE INDEXING
fn parse_b_type(inst: u32) -> (u32, u32, u32, u32, u32) {
    let imm = ((inst >> 20) & 0x7F) | ((inst >> 7) & 0x1F);
    let rs2 = (inst >> 20) & 0x1F;
    let rs1 = (inst >> 15) & 0x1F;
    let funct3 = (inst >> 12) & 0x7;
    let opcode = inst & 0x7F;
    (imm, rs2, rs1, funct3, opcode)
}
fn parse_u_type(inst: u32) -> (u32, u32, u32) {
    let imm = inst >> 12;
    let rd: u32 = (inst >> 7) & 0x1F;
    let opcode = inst & 0x7F;
    (imm, rd, opcode)
}
// TODO: FIX IMMEDIATE INDEXING
fn parse_j_type(inst: u32) -> (u32, u32, u32) {
    let imm = inst >> 12;
    let rd: u32 = (inst >> 7) & 0x1F;
    let opcode = inst & 0x7F;
    (imm, rd, opcode)
}

pub fn decode_inst(inst: u32) -> Instruction {
    let mut opcode: u32 = inst & 0x7F;

    match opcode {
        0b0110111 => {
            let (imm, rd, _) = parse_u_type(inst);
            Instruction::Lui { imm, rd }
        }
        0b0010111 => {
            let (imm, rd, _) = parse_u_type(inst);
            Instruction::Auipc { imm, rd }
        }
        0b1101111 => {
            let (imm, rd, _) = parse_j_type(inst);
            Instruction::Auipc { imm, rd }
        }
        0b1100111 => {
            let (imm, rs1, _, rd, _) = parse_i_type(inst);
            Instruction::Jalr { rd, rs1, imm }
        }
        // Branches
        0b1100011 => {
            let (imm, rs2, rs1, funct3, _) = parse_b_type(inst);
            match funct3 {
                0b000 => Instruction::Beq { rs1, rs2, imm },
                0b001 => Instruction::Bne { rs1, rs2, imm },
                0b100 => Instruction::Blt { rs1, rs2, imm },
                0b101 => Instruction::Bge { rs1, rs2, imm },
                0b110 => Instruction::Bltu { rs1, rs2, imm },
                0b111 => Instruction::Bgeu { rs1, rs2, imm },
                _ => Instruction::Ill,
            }
        }
        // Loads
        0b0000011 => {
            let (imm, rs1, funct3, rd, _) = parse_i_type(inst);
            match funct3 {
                0b000 => Instruction::Lb { rd, rs1, imm },
                0b001 => Instruction::Lh { rd, rs1, imm },
                0b010 => Instruction::Lw { rd, rs1, imm },
                0b100 => Instruction::Lbu { rd, rs1, imm },
                0b101 => Instruction::Lhu { rd, rs1, imm },
                _ => Instruction::Ill,
            }
        }
        // Stores
        0b0100011 => {
            let (imm, rs2, rs1, funct3, _) = parse_s_type(inst);
            match funct3 {
                0b000 => Instruction::Sb { rs1, rs2, imm },
                0b001 => Instruction::Sh { rs1, rs2, imm },
                0b010 => Instruction::Sw { rs1, rs2, imm },
                _ => Instruction::Ill,
            }
        }
        // Immediates
        0b0010011 => {
            let (imm, rs1, funct3, rd, _) = parse_i_type(inst);
            let shamt = imm & 0x1F;
            let funct7 = (imm & 0xFE) >> 5;
            match funct3 {
                0b000 => Instruction::Addi { rd, rs1, imm },
                0b010 => Instruction::Slti { rd, rs1, imm },
                0b011 => Instruction::Sltiu { rd, rs1, imm },
                0b100 => Instruction::Xori { rd, rs1, imm },
                0b110 => Instruction::Ori { rd, rs1, imm },
                0b111 => Instruction::Andi { rd, rs1, imm },
                0b001 => Instruction::Slli { rd, rs1, shamt },
                0b101 => match funct7 {
                    0b0000000 => Instruction::Srli { rd, rs1, shamt },
                    0b0100000 => Instruction::Srai { rd, rs1, shamt },
                    _ => Instruction::Ill,
                },
                _ => Instruction::Ill,
            }
        }
        // R-type (standard)
        0b0110011 => {
            let (funct7, rs2, rs1, funct3, rd, _) = parse_r_type(inst);
            match funct3 {
                0b000 => match funct7 {
                    0b0000000 => Instruction::Add { rd, rs1, rs2 },
                    0b0100000 => Instruction::Sub { rd, rs1, rs2 },
                    _ => Instruction::Ill,
                },

                0b001 => Instruction::Sll { rd, rs1, rs2 },
                0b010 => Instruction::Slt { rd, rs1, rs2 },
                0b011 => Instruction::Sltu { rd, rs1, rs2 },
                0b100 => Instruction::Xor { rd, rs1, rs2 },
                0x101 => match funct7 {
                    0b0000000 => Instruction::Srl { rd, rs1, rs2 },
                    0b0100000 => Instruction::Sra { rd, rs1, rs2 },
                    _ => Instruction::Ill,
                },
                0x110 => Instruction::Or { rd, rs1, rs2 },
                0x111 => Instruction::And { rd, rs1, rs2 },
                _ => Instruction::Ill,
            }
        }
        _ => Instruction::Ill,
    }
}
