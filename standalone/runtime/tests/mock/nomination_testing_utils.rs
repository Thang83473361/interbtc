use currency::Amount;

use crate::{assert_eq, *};

pub const USER: [u8; 32] = ALICE;
pub const VAULT: [u8; 32] = BOB;

pub const DEFAULT_GRIEFING_COLLATERAL: Amount<Runtime> = griefing(5_000);
pub const DEFAULT_BACKING_COLLATERAL: u128 = 1_000_000;
pub const DEFAULT_NOMINATION: u128 = 20_000;

pub const DEFAULT_VAULT_UNBONDING_PERIOD: u32 = 100;
pub const DEFAULT_NOMINATOR_UNBONDING_PERIOD: u32 = 50;

pub fn default_backing_collateral(currency_id: CurrencyId) -> Amount<Runtime> {
    Amount::new(DEFAULT_BACKING_COLLATERAL, currency_id)
}

pub fn enable_nomination() {
    assert_ok!(Call::Nomination(NominationCall::set_nomination_enabled(true))
        .dispatch(<Runtime as frame_system::Config>::Origin::root()));
}

pub fn disable_nomination() {
    assert_ok!(Call::Nomination(NominationCall::set_nomination_enabled(false))
        .dispatch(<Runtime as frame_system::Config>::Origin::root()));
}

pub fn register_vault(currency_id: CurrencyId, vault: [u8; 32]) -> DispatchResultWithPostInfo {
    Call::VaultRegistry(VaultRegistryCall::register_vault(
        DEFAULT_BACKING_COLLATERAL,
        dummy_public_key(),
        currency_id,
    ))
    .dispatch(origin_of(account_of(vault)))
}

pub fn assert_register_vault(currency_id: CurrencyId, vault: [u8; 32]) {
    assert_ok!(register_vault(currency_id, vault));
}

pub fn nomination_opt_in(vault: [u8; 32]) -> DispatchResultWithPostInfo {
    Call::Nomination(NominationCall::opt_in_to_nomination()).dispatch(origin_of(account_of(vault)))
}

pub fn assert_nomination_opt_in(vault: [u8; 32]) {
    assert_ok!(nomination_opt_in(vault));
}

pub fn nomination_opt_out(vault: [u8; 32]) -> DispatchResultWithPostInfo {
    Call::Nomination(NominationCall::opt_out_of_nomination()).dispatch(origin_of(account_of(vault)))
}

pub fn nominate_collateral(
    vault: [u8; 32],
    nominator: [u8; 32],
    amount_collateral: Amount<Runtime>,
) -> DispatchResultWithPostInfo {
    Call::Nomination(NominationCall::deposit_collateral(
        account_of(vault),
        amount_collateral.amount(),
    ))
    .dispatch(origin_of(account_of(nominator)))
}

pub fn assert_nominate_collateral(vault: [u8; 32], nominator: [u8; 32], amount_collateral: Amount<Runtime>) {
    assert_ok!(nominate_collateral(vault, nominator, amount_collateral));
}

pub fn withdraw_vault_collateral(vault: [u8; 32], amount_collateral: Amount<Runtime>) -> DispatchResultWithPostInfo {
    Call::VaultRegistry(VaultRegistryCall::withdraw_collateral(amount_collateral.amount()))
        .dispatch(origin_of(account_of(vault)))
}

pub fn assert_withdraw_vault_collateral(vault: [u8; 32], amount_dot: Amount<Runtime>) {
    assert_ok!(withdraw_vault_collateral(vault, amount_dot));
}

pub fn withdraw_nominator_collateral(
    nominator: [u8; 32],
    vault: [u8; 32],
    amount_collateral: Amount<Runtime>,
) -> DispatchResultWithPostInfo {
    Call::Nomination(NominationCall::withdraw_collateral(
        account_of(vault),
        amount_collateral.amount(),
        None,
    ))
    .dispatch(origin_of(account_of(nominator)))
}

pub fn assert_withdraw_nominator_collateral(nominator: [u8; 32], vault: [u8; 32], amount_dot: Amount<Runtime>) {
    assert_ok!(withdraw_nominator_collateral(nominator, vault, amount_dot));
}

pub fn assert_total_nominated_collateral_is(vault: [u8; 32], amount_collateral: Amount<Runtime>) {
    let nominated_collateral = NominationPallet::get_total_nominated_collateral(&account_of(vault)).unwrap();
    assert_eq!(nominated_collateral, amount_collateral);
}

pub fn get_nominator_collateral(vault: [u8; 32], nominator: [u8; 32]) -> Amount<Runtime> {
    NominationPallet::get_nominator_collateral(&account_of(vault), &account_of(nominator)).unwrap()
}
