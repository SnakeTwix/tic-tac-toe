use crate::grid::MarkerType;

pub fn marker_to_num(marker: &MarkerType) -> i32 {
    match marker {
        MarkerType::X => 1,
        MarkerType::O => 2,
    }
}
