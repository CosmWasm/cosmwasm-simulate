use cosmwasm_std::{HumanAddr, StdResult, CanonicalAddr, Binary, generic_err};

pub struct Utils;

impl Utils {
    pub fn canonical_address(human: &HumanAddr) -> StdResult<CanonicalAddr> {
        const CANONICAL_LEN: usize = 20;
        if human.len() < 3 {
            return Err(generic_err("Invalid input: human address too short"));
        }
        if human.len() > CANONICAL_LEN {
            return Err(generic_err("Invalid input: human address too long"));
        }

        let mut out = Vec::from(human.as_str());
        let append = CANONICAL_LEN - out.len();
        if append > 0 {
            out.extend(vec![0u8; append]);
        }
        Ok(CanonicalAddr(Binary(out)))
    }
}