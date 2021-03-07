use std::io::Read;

enum Instr {
    HLF(usize),
    TPL(usize),
    INC(usize),
    JMP(isize),
    JIE(usize, isize),
    JIO(usize, isize),
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let instructions: Vec<_> = regex::Regex::new(
        r"(?m)^(\w{3})(?: (a|b))?,?(?: ([\+-]\d+))?$",
    ).unwrap().captures_iter(&input).map(|caps| {
        let reg = caps.get(2).map(|reg| if reg.as_str() == "a" { 0 } else { 1 });
        let offset = caps.get(3).map(|reg| reg.as_str().parse().unwrap());
        match &caps[1] {
            "hlf" => Instr::HLF(reg.unwrap()),
            "tpl" => Instr::TPL(reg.unwrap()),
            "inc" => Instr::INC(reg.unwrap()),
            "jmp" => Instr::JMP(offset.unwrap()),
            "jie" => Instr::JIE(reg.unwrap(), offset.unwrap()),
            "jio" => Instr::JIO(reg.unwrap(), offset.unwrap()),
            _ => panic!(),
        }
    }).collect();

    let mut pc = 0;
    let mut regs = vec![0; 2];

    while let Some(instruction) = instructions.get(pc as usize) {
        pc += 1;

        match *instruction {
            Instr::HLF(reg) => { regs[reg] /= 2; },
            Instr::TPL(reg) => { regs[reg] *= 3; },
            Instr::INC(reg) => { regs[reg] += 1; },
            Instr::JMP(offset) => { pc += offset - 1; },

            Instr::JIE(reg, offset) => {
                if regs[reg] % 2 == 0 {
                    pc += offset - 1;
                }
            },

            Instr::JIO(reg, offset) => {
                if regs[reg] == 1 {
                    pc += offset - 1;
                }
            },
        }
    }

    println!("{}", regs[1]);
}
