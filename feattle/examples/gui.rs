use feattle::*;
use std::collections::{BTreeMap, BTreeSet};
use strum::VariantNames;
use uuid::Uuid;

feattle_enum! {
    CalculateMoneySupply {
        High,
        Low,
    }
}

feattle_enum! {
    CalibratePersonalityMatrix {
        Rows,
        Columns,
        Diagonals,
        AntiDiagonals,
    }
}

feattle_enum! {
    MapInfluenceAttributes {
        Bias,
        Linear,
        Square,
    }
}

feattles! {
    Features {
        extrude_mesh_terrain: bool,
        /// A short description
        balance_domestic_coefficients: i32,
        /// A longer, complete description, bringing attention to contentious issues surrounding
        /// this configuration and what could go wrong if misused.
        invert_career_ladder: f64,
        calculate_money_supply: CalculateMoneySupply,
        reticulate_splines: Uuid,
        normalize_social_network: String,
        adjust_emotional_weights: BTreeSet<i32>,
        calibrate_personality_matrix: BTreeSet<CalibratePersonalityMatrix>,
        concatenate_vertex_nodes: BTreeSet<Uuid>,
        insert_chaos_generator: BTreeSet<String>,
        map_influence_attributes: BTreeMap<MapInfluenceAttributes, i32>,
        iterate_chaos_array: BTreeMap<Uuid, i32>,
        assign_mimic_propagation: BTreeMap<String, i32>,
    }
}

fn main() {
    dbg!(Features::definitions());
}
