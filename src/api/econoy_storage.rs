use uuid::Uuid;

use crate::utils::Account;

pub trait EconomyStorage {
    fn load(&self, uuid: Uuid) -> Option<Account>;

    fn save(&self, account: &Account);

    fn delete(&self, uuid: Uuid);
}
