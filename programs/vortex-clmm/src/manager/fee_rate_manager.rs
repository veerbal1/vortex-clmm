use crate::{
    math::get_total_fee_rate,
    state::{AdaptiveFeeConstants, AdaptiveFeeVariables},
};

#[derive(Debug, Clone)]
pub enum FeeRateManager {
    /// Static fee - no volatility tracking
    Static { fee_rate: u16 },

    /// Adaptive fee - tracks volatility during swap
    Adaptive {
        a_to_b: bool,
        static_fee_rate: u16,
        tick_group_index: i32,
        constants: AdaptiveFeeConstants,
        variables: AdaptiveFeeVariables,
    },
}

impl FeeRateManager {
    pub fn new_static(fee_rate: u16) -> Self {
        Self::Static { fee_rate }
    }

    pub fn get_fee_rate(&self) -> u32 {
        match self {
            Self::Static { fee_rate } => *fee_rate as u32,
            Self::Adaptive {
                static_fee_rate,
                constants,
                variables,
                ..
            } => get_total_fee_rate(
                *static_fee_rate,
                variables.volatility_accumulator,
                constants.tick_group_size,
                constants.adaptive_fee_control_factor,
            ),
        }
    }

    pub fn new_adaptive(
        a_to_b: bool,
        current_tick_index: i32,
        current_timestamp: u64,
        static_fee_rate: u16,
        constants: AdaptiveFeeConstants,
        mut variables: AdaptiveFeeVariables,
    ) -> Self {
        use crate::math::adaptive_fee::{get_tick_group_index, update_reference};

        let tick_group_index = get_tick_group_index(current_tick_index, constants.tick_group_size);

        let reference_update = update_reference(
            tick_group_index,
            current_timestamp,
            variables.last_reference_update_timestamp,
            variables.tick_group_index_reference,
            variables.volatility_reference,
            variables.volatility_accumulator,
            constants.filter_period,
            constants.decay_period,
            constants.reduction_factor,
        );

        variables.tick_group_index_reference = reference_update.tick_group_index_reference;
        variables.volatility_reference = reference_update.volatility_reference;
        variables.last_reference_update_timestamp =
            reference_update.last_reference_update_timestamp;

        Self::Adaptive {
            a_to_b,
            static_fee_rate,
            tick_group_index,
            constants,
            variables,
        }
    }
}
