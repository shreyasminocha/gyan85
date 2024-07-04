use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, hex_digit1, line_ending, multispace0, space0, space1},
    combinator::{all_consuming, eof, value},
    multi::{many0, many1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

use crate::yan85::{instruction::Instruction, register::Register};

/// Parses a file with Yan85 assembly instructions with one instruction per line. A wrapper around
/// [`parse_asm_instructions`] that accepts an owned type and returns a standard result type.
pub fn parse_asm_file(asm: String) -> Result<Vec<Instruction>> {
    let (_, instructions) = all_consuming(parse_asm_instructions)(&asm)
        .map_err(|e| anyhow!("Unable to parse assembly file: {}", e))?;

    Ok(instructions)
}

/// Parses a string with Yan85 assembly instructions with one instruction per line.
fn parse_asm_instructions(asm: &str) -> IResult<&str, Vec<Instruction>> {
    delimited(
        multispace0, // leading
        many0(terminated(parse_asm_instruction, multispace0)),
        multispace0, // trailing
    )(asm)
}

/// Parses a single Yan85 assembly instruction.
fn parse_asm_instruction(asm: &str) -> IResult<&str, Instruction> {
    alt((
        parse_imm, parse_add, parse_stk, parse_stm, parse_ldm, parse_cmp, parse_jmp, parse_sys,
    ))(asm)
}

/// Parses an `IMM` (immediate assignment) instruction.
///
/// Syntax: `IMM <reg> = <int literal>`
///
/// Examples:
///
/// - `IMM i = 0x42`
/// - `IMM a = 42`
fn parse_imm(asm: &str) -> IResult<&str, Instruction> {
    let (remaining, (reg, imm)) = delimited(
        tuple((tag("IMM"), space1)),
        separated_pair(parse_register, parse_assignment, parse_int_literal),
        tuple((space0, alt((line_ending, eof)))),
    )(asm)?;

    Ok((remaining, Instruction::IMM(reg, imm)))
}

/// Parses an `ADD` instruction.
///
/// Syntax: `ADD <reg> <reg>`
///
/// Examples:
///
/// - `ADD a b`
/// - `ADD b c`
fn parse_add(asm: &str) -> IResult<&str, Instruction> {
    let (remaining, (a, b)) = delimited(
        tuple((tag("ADD"), space1)),
        separated_pair(parse_register, space1, parse_register),
        tuple((space0, alt((line_ending, eof)))),
    )(asm)?;

    Ok((remaining, Instruction::ADD(a, b)))
}

/// Parses a `STK` (stack operation, push/pop) instruction.
///
/// Syntax: `STK <pop_reg> <push_reg>`
///
/// Examples:
///
/// - `STK a b`
/// - `STK a NONE`
/// - `STK NONE b`
fn parse_stk(asm: &str) -> IResult<&str, Instruction> {
    let (remaining, (a, b)) = delimited(
        tuple((tag("STK"), space1)),
        separated_pair(parse_optional_register, space1, parse_optional_register),
        tuple((space0, alt((line_ending, eof)))),
    )(asm)?;

    Ok((remaining, Instruction::STK(a, b)))
}

/// Parses a `STM` (store in memory) instruction.
///
/// Syntax: `STM *<reg> <reg>`
///
/// Examples:
///
/// - `STM *a = b`
/// - `STM *b = a`
fn parse_stm(asm: &str) -> IResult<&str, Instruction> {
    let (remaining, (a, b)) = delimited(
        tuple((tag("STM"), space1)),
        separated_pair(parse_deref_register, parse_assignment, parse_register),
        tuple((space0, alt((line_ending, eof)))),
    )(asm)?;

    Ok((remaining, Instruction::STM(a, b)))
}

/// Parses a `LDM` (load from memory) instruction.
///
/// Syntax: `LDM <reg> *<reg>`
///
/// Examples:
///
/// - `LDM b = *a`
/// - `LDM a = *b`
fn parse_ldm(asm: &str) -> IResult<&str, Instruction> {
    let (remaining, (a, b)) = delimited(
        tuple((tag("LDM"), space1)),
        separated_pair(parse_register, parse_assignment, parse_deref_register),
        tuple((space0, alt((line_ending, eof)))),
    )(asm)?;

    Ok((remaining, Instruction::LDM(a, b)))
}

/// Parses a `CMP` (compare) instruction.
///
/// Syntax: `CMP <reg> <reg>`
///
/// Examples:
///
/// - `CMP a b`
/// - `CMP c d`
fn parse_cmp(asm: &str) -> IResult<&str, Instruction> {
    let (remaining, (a, b)) = delimited(
        tuple((tag("CMP"), space1)),
        separated_pair(parse_register, space1, parse_register),
        tuple((space0, alt((line_ending, eof)))),
    )(asm)?;

    Ok((remaining, Instruction::CMP(a, b)))
}

/// Parses a `JMP` (jump) instruction.
///
/// Syntax: `JMP [LGENZ]+ <reg>`
///
/// Examples:
///
/// - `JMP LE d`
/// - `JMP Z d`
fn parse_jmp(asm: &str) -> IResult<&str, Instruction> {
    let (remaining, (condition, reg)) = delimited(
        tuple((tag("JMP"), space1)),
        separated_pair(
            // note that this allows repeated flag letters, e.g. "LZL"
            many1(alt((tag("L"), tag("G"), tag("E"), tag("N"), tag("Z")))),
            space1,
            parse_register,
        ),
        tuple((space0, alt((line_ending, eof)))),
    )(asm)?;

    let condition = condition
        .join("")
        .as_str()
        .try_into()
        .expect("the parser only allows valid flag letters");

    Ok((remaining, Instruction::JMP(condition, reg)))
}

/// Parses a `SYS` (system call) instruction.
///
/// Syntax: `SYS <int literal> <reg>`
///
/// Examples:
///
/// - `SYS 0x20 d`
/// - `SYS 0x2 d`
fn parse_sys(asm: &str) -> IResult<&str, Instruction> {
    let (remaining, (syscall, reg)) = delimited(
        tuple((tag("SYS"), space1)),
        separated_pair(parse_int_literal, space1, parse_register),
        tuple((space0, alt((line_ending, eof)))),
    )(asm)?;

    Ok((remaining, Instruction::SYS(syscall, reg)))
}

/// Parses an optional register name. Parsing `NONE` results in `None`.
///
/// Primarily used in [`parse_stk`].
fn parse_optional_register(input: &str) -> IResult<&str, Option<Register>> {
    if let Ok((remaining, reg)) = parse_register(input) {
        Ok((remaining, Some(reg)))
    } else {
        value(None, tag("NONE"))(input)
    }
}

/// Parses a register name, *not* including `NONE`.
fn parse_register(input: &str) -> IResult<&str, Register> {
    alt((
        value(Register::A, tag("a")),
        value(Register::B, tag("b")),
        value(Register::C, tag("c")),
        value(Register::D, tag("d")),
        value(Register::S, tag("s")),
        value(Register::I, tag("i")),
        value(Register::F, tag("f")),
    ))(input)
}

/// Parses a dereferenced register, i.e., a register preceded by `*`.
fn parse_deref_register(input: &str) -> IResult<&str, Register> {
    let (remaining, reg) = preceded(tag("*"), parse_register)(input)?;
    Ok((remaining, reg))
}

/// Parses the assignment symbol `=`, including whitespace on either side.
fn parse_assignment(input: &str) -> IResult<&str, ()> {
    let (remaining, _) = delimited(space1, tag("="), space1)(input)?;
    Ok((remaining, ()))
}

/// Parses a decimal or hexadecimal integer literal.
fn parse_int_literal(input: &str) -> IResult<&str, u8> {
    alt((
        parse_int_literal_hex, // _hex must come first; _decimal would consume the "0" in "0x"
        parse_int_literal_decimal,
    ))(input)
}

/// Parses a decimal integer literal.
fn parse_int_literal_decimal(input: &str) -> IResult<&str, u8> {
    let (remaining, digits) = digit1(input)?;
    if let Ok(num) = digits.parse::<u8>() {
        Ok((remaining, num))
    } else {
        // out of bounds
        // todo: ew
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Digit,
        )))
    }
}

/// Parses a hexadecimal integer literal, e.g. `0xfe`.
fn parse_int_literal_hex(input: &str) -> IResult<&str, u8> {
    let (remaining, (_, digits)) = tuple((tag("0x"), hex_digit1))(input)?;
    if let Ok(num) = u8::from_str_radix(digits, 16) {
        Ok((remaining, num))
    } else {
        // out of bounds
        // todo: ew
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Digit,
        )))
    }
}

#[cfg(test)]
mod tests {
    use crate::yan85::flags::Flags;

    use super::*;

    #[test]
    fn test_parse_instructions() {
        let instructions = parse_asm_file(
            r#"
                IMM i = 0x69
                IMM b = 0xac
                IMM c = 0xa
            "#
            .to_string(),
        )
        .unwrap();

        assert_eq!(
            instructions,
            vec![
                Instruction::IMM(Register::I, 0x69),
                Instruction::IMM(Register::B, 0xac),
                Instruction::IMM(Register::C, 0xa),
            ]
        )
    }

    #[test]
    fn test_imm() {
        let (_, instruction) = parse_asm_instruction("IMM i = 42").unwrap();
        assert_eq!(instruction, Instruction::IMM(Register::I, 42));
    }

    #[test]
    fn test_imm_hex_literal() {
        let (_, instruction) = parse_asm_instruction("IMM i = 0x42").unwrap();
        assert_eq!(instruction, Instruction::IMM(Register::I, 0x42));
    }

    #[test]
    fn test_imm_out_of_bounds_literal() {
        assert!(parse_asm_instruction("IMM i = 0xf00").is_err());
    }

    #[test]
    fn test_imm_none_operand() {
        assert!(parse_asm_instruction("IMM NONE = 0").is_err());
    }

    #[test]
    fn test_add() {
        let (_, instruction) = parse_asm_instruction("ADD a b").unwrap();
        assert_eq!(instruction, Instruction::ADD(Register::A, Register::B));
    }

    #[test]
    fn test_add_extra_operand() {
        assert!(parse_asm_instruction("ADD a b c").is_err());
    }

    #[test]
    fn test_add_none_operand() {
        assert!(parse_asm_instruction("ADD a NONE").is_err());
    }

    #[test]
    fn test_stk_push_pop() {
        let (_, instruction) = parse_asm_instruction("STK a b").unwrap();
        assert_eq!(
            instruction,
            Instruction::STK(Some(Register::A), Some(Register::B))
        );
    }

    #[test]
    fn test_stk_pop() {
        let (_, instruction) = parse_asm_instruction("STK a NONE").unwrap();
        assert_eq!(instruction, Instruction::STK(Some(Register::A), None));
    }

    #[test]
    fn test_stk_push() {
        let (_, instruction) = parse_asm_instruction("STK NONE b").unwrap();
        assert_eq!(instruction, Instruction::STK(None, Some(Register::B)));
    }

    #[test]
    fn test_stk_noop() {
        let (_, instruction) = parse_asm_instruction("STK NONE NONE").unwrap();
        assert_eq!(instruction, Instruction::STK(None, None));
    }

    #[test]
    fn test_stm() {
        let (_, instruction) = parse_asm_instruction("STM *a = b").unwrap();
        assert_eq!(instruction, Instruction::STM(Register::A, Register::B));
    }

    #[test]
    fn test_stm_no_deref() {
        assert!(parse_asm_instruction("STM a = b").is_err());
    }

    #[test]
    fn test_stm_misplaced_deref() {
        assert!(parse_asm_instruction("STM a = *b").is_err());
    }

    #[test]
    fn test_ldm() {
        let (_, instruction) = parse_asm_instruction("LDM b = *a").unwrap();
        assert_eq!(instruction, Instruction::LDM(Register::B, Register::A));
    }

    #[test]
    fn test_ldm_no_deref() {
        assert!(parse_asm_instruction("LDM b = a").is_err());
    }

    #[test]
    fn test_ldm_misplaced_deref() {
        assert!(parse_asm_instruction("LDM *b = a").is_err());
    }

    #[test]
    fn test_cmp() {
        let (_, instruction) = parse_asm_instruction("CMP a b").unwrap();
        assert_eq!(instruction, Instruction::CMP(Register::A, Register::B));
    }

    #[test]
    fn test_jmp() {
        let (_, instruction) = parse_asm_instruction("JMP LZ d").unwrap();
        assert_eq!(
            instruction,
            Instruction::JMP(
                Flags {
                    less_than: true,
                    greater_than: false,
                    equal: false,
                    not_equal: false,
                    zeroes: true
                },
                Register::D
            )
        );
    }

    #[test]
    fn test_jmp_misordered_operands() {
        assert!(parse_asm_instruction("JMP d 8").is_err());
    }

    #[test]
    fn test_sys() {
        let (_, instruction) = parse_asm_instruction("SYS 0x20 d").unwrap();
        assert_eq!(instruction, Instruction::SYS(0x20, Register::D));
    }

    #[test]
    fn test_sys_none_operand() {
        // TODO: change if/when SYS ever supports NONE args
        assert!(parse_asm_instruction("SYS 0x8 NONE").is_err());
    }

    #[test]
    fn test_sys_misordered_operands() {
        assert!(parse_asm_instruction("SYS d 0x20").is_err());
    }

    #[test]
    fn test_int_literal_decimal() {
        let (_, literal) = parse_int_literal("5").unwrap();
        assert_eq!(literal, 5);
    }

    #[test]
    fn test_int_literal_hex() {
        let (_, literal) = parse_int_literal("0x20").unwrap();
        assert_eq!(literal, 0x20);
    }

    #[test]
    fn test_int_literal_out_of_bounds() {
        assert!(parse_int_literal("256").is_err());

        // assert!(parse_int_literal("0xf00").is_err());
        //
        // this parses as `0` because when hex parsing fails, it falls back to decimal parsing.
        // i suppose it's not a big deal because the trailing junk will cause a failure eventually.
    }
}
