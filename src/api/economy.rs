use crate::err::EconomyError;

pub trait Economy {
    fn balance(&self, player: Uuid) -> i64;

    fn set_balance(&mut self, player: Uuid, amount: i64);

    fn deposit(&mut self, player: Uuid, amount: i64);

    fn withdraw(&mut self, player: Uuid, amount: i64) -> Result<(), EconomyError>;

    fn has(&self, player: Uuid, amount: i64) -> bool;
}
