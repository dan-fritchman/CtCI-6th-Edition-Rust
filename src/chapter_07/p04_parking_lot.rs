//!
//! # Parking Lot:
//!
//! Design a parking lot using object-oriented principles.
//!
//! Hints: #258
//!
//! (Borrowing from CtCI's Solutions section for more context.)
//!
//! The wording of this question is vague, just as it would be in an actual interview.
//! This requires you to have a conversation with your interviewer about what types of vehicles it can support,
//! whether the parking lot has multiple levels, and so on.
//! For our purposes right now, we'll make the following assumptions.
//! We made these specific assumptions to add a bit of complexity to the problem without adding too much.
//! If you made different assumptions, that's totally fine.
//!
//! * The parking lot has multiple levels. Each level has multiple rows of spots.
//! * The parking lot can park motorcycles, cars, and buses.
//! * The parking lot has motorcycle spots, compact spots, and large spots.
//! * A motorcycle can park in any spot.
//! * A car can park in either a single compact spot or a single large spot.
//! * A bus can park in five large spots that are consecutive and within the same row. It cannot park in small spots.
//!

// In addition to being a wide-open question, we'll also be designing this [ParkingLot] in Rust,
// a language which wouldn't call itself object-oriented.
// This is actually a case where (the author thinks) Rust's algebraic enums work better than sub-classes.

/// Enumerated Vehicle Types
#[derive(Debug, PartialEq, Eq)]
pub enum Vehicle {
    Motorcycle,
    Car,
    Bus,
}

/// Enumerated [ParkingSpace] Types
#[derive(Debug, PartialEq, Eq)]
pub enum SpaceType {
    Motorcycle,
    Compact,
    Large,
}

/// Space Occupation Status: Filled vs Empty
#[derive(Debug, PartialEq, Eq)]
pub enum SpaceStatus {
    Empty,
    Filled,
}

/// Parking Space 
#[derive(Debug, PartialEq, Eq)]
pub struct ParkingSpace {
    /// Space Type
    tp: SpaceType,
    /// Occupation status: filled vs empty
    status: SpaceStatus,
}

#[derive(Debug)]
pub struct ParkingRow {
    spaces: Vec<ParkingSpace>,
}
impl ParkingRow {
    /// Boolean indication of whether the level has space for `vehicle`
    pub fn fits(&self, vehicle: &Vehicle) -> bool {
        match vehicle {
            Vehicle::Motorcycle => self
                .spaces
                .iter()
                .any(|space| space.status == SpaceStatus::Empty),
            Vehicle::Car => self.spaces.iter().any(|space| {
                space.status == SpaceStatus::Empty
                    && (space.tp == SpaceType::Compact || space.tp == SpaceType::Large)
            }),
            Vehicle::Bus => {
                // Try to find 5 consecutive large spots
                let mut consec = 0;
                for space in &self.spaces {
                    if space.status == SpaceStatus::Empty && space.tp == SpaceType::Large {
                        consec += 1;
                        if consec >= 5 {
                            return true;
                        }
                    } else {
                        consec = 0;
                    }
                }
                // Didn't find any.
                false
            }
        }
    }
}
#[derive(Debug)]
pub struct ParkingLevel {
    rows: Vec<ParkingRow>,
}
impl ParkingLevel {
    /// Boolean indication of whether the lot has space for `vehicle`
    pub fn fits(&self, vehicle: &Vehicle) -> bool {
        self.rows.iter().any(|row| row.fits(vehicle))
    }
    /// Park `vehicle` in the lot. Returns an [Err] if space is not available.
    pub fn park(&mut self, vehicle: Vehicle) -> Result<(), ()> {
        todo!()
    }
}

#[derive(Debug)]
pub struct ParkingLot {
    levels: Vec<ParkingLevel>,
}
impl ParkingLot {
    /// Boolean indication of whether the lot has space for `vehicle`
    pub fn fits(&self, vehicle: &Vehicle) -> bool {
        self.levels.iter().any(|level| level.fits(vehicle))
    }
    /// Park `vehicle` in the lot. Returns an [Err] if space is not available.
    pub fn park(&mut self, vehicle: Vehicle) -> Result<(), ()> {
        todo!()
    }
}
