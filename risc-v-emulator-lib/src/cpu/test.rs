#[test]
fn test_cmp_casting() {
    assert_eq!(1, u64::from(69.lt(&420)));
    assert_eq!(0, u64::from(69.lt(&42)));
}

#[test]
fn test_unsigned_signed_add() {
    let a: u64 = 100;
    let b: i64 = -5;
    let (c, _) = a.overflowing_add(b as u64);
    assert_eq!(95, c)
}

mod instructions {
    use std::fs;
    use std::num::ParseIntError;
    use std::str::FromStr;
    use std::time::SystemTime;

    use crate::bus::Bus;
    use crate::cpu::{CpuRV64I, RegisterDump};
    use crate::dram::Dram;

    // TODO parse at compile time
    fn parse_testcase(testcase: &str) -> RegisterDump {
        fn parse_u64(s: &str) -> Result<u64, ParseIntError> {
            if let Some(s) = s.strip_prefix("0x") {
                u64::from_str_radix(s, 16)
            } else if let Some(s) = s.strip_prefix("0o") {
                u64::from_str_radix(s, 8)
            } else if let Some(s) = s.strip_prefix("0b") {
                u64::from_str_radix(s, 2)
            } else if let Some(s) = s.strip_prefix('-') {
                u64::from_str(s).map(|v| -i128::from(v) as u64)
            } else {
                u64::from_str(s)
            }
        }

        let comments = testcase
            .lines()
            .enumerate()
            .filter_map(|(line_n, l)| l.split_once('#').map(|(_, comment)| (line_n, comment)));

        let definitions = comments
            .clone()
            .filter_map(|(line_n, c)| c.split_once('=').map(|(k, v)| (line_n, k.trim(), v.trim())));

        let mut expected_regs = RegisterDump::uninitialized();

        for (line, k, v) in definitions {
            match k.strip_prefix('x') {
                Some(reg) => {
                    expected_regs.registers[usize::from_str(reg)
                        .unwrap_or_else(|_| panic!("Could not parse key {k} in line {line}"))] =
                        Some(parse_u64(v).unwrap_or_else(|e| {
                            panic!("Could not parse value {v} in line {line}: {e}")
                        }));
                }
                None => match k {
                    "pc" => {
                        expected_regs.pc = Some(parse_u64(v).unwrap_or_else(|e| {
                            panic!("Could not parse value {v} in line {line}: {e}")
                        }));
                    }
                    _ => {
                        panic!("Could not parse key {k} in line {line}")
                    }
                },
            }
        }

        expected_regs
    }

    /// test runner for instruction tests
    fn execute_insn_test(name: &str, testcase: &str, binary: &[u8]) {
        let mut cpu = CpuRV64I::new(Bus::new(Dram::with_code(binary)));

        loop {
            // were currently just waiting for the cpu to run into empty memory
            if cpu.cycle().is_err() {
                break;
            }
        }

        let mut actual_regs = cpu.dump_registers();

        let expected_regs = parse_testcase(testcase);

        actual_regs.apply_mask(&expected_regs);

        if actual_regs != expected_regs {
            let data = cpu.bus.get_mem().get_data();

            fs::write(
                format!(
                    "memdump_dram_test_{name}_{}.dump",
                    SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .expect("Could not calculate current timestamp!")
                        .as_secs()
                ),
                data,
            )
            .expect("Could not write memory dump!");
            assert_eq!(
                actual_regs, expected_regs,
                "Actual Register values are not matching expected values defined in {name}.S!",
            );
        }
    }
    // include tests generated via build.rs
    include!(concat!(env!("OUT_DIR"), "/tests_insn.rs"));
}
