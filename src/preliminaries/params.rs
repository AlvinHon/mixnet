use crate::preliminaries::{MixnetError, MixnetResult};

#[derive(Debug, Clone)]
pub struct SecurityParams {
    pub statistical_security_bits: u32,
    pub computational_security_bits: u32,
    pub n_mixers: usize,
    pub n_messages: usize,
    pub polynomial_degree: usize,
    pub modulus_q: u64,
}

impl SecurityParams {
    pub fn baseline() -> Self {
        Self {
            statistical_security_bits: 40,
            computational_security_bits: 128,
            n_mixers: 3,
            n_messages: 64,
            polynomial_degree: 256,
            modulus_q: 12289,
        }
    }

    pub fn validate(&self) -> MixnetResult<()> {
        if self.n_mixers == 0 {
            return Err(MixnetError::InvalidParameter("n_mixers must be > 0"));
        }
        if self.n_messages == 0 {
            return Err(MixnetError::InvalidParameter("n_messages must be > 0"));
        }
        if self.polynomial_degree == 0 {
            return Err(MixnetError::InvalidParameter(
                "polynomial_degree must be > 0",
            ));
        }
        if self.modulus_q < 2 {
            return Err(MixnetError::InvalidParameter("modulus_q must be >= 2"));
        }

        Ok(())
    }
}
