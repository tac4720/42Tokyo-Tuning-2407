use sqlx::FromRow;

#[derive(FromRow, Clone, Debug)]
pub struct TowTruck {
    pub id: i32,
    pub driver_id: i32,
    pub driver_username: Option<String>,
    pub status: bool,
    pub area_id: i32,
    pub node_id: i32,
}
