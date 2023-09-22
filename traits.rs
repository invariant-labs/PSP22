use ink::{prelude::vec::Vec, primitives::AccountId};

use crate::types::{Balance, PSP22Error};

#[ink::trait_definition]
pub trait PSP22 {
    /// Returns the total token supply.
    #[ink(message)]
    fn total_supply(&self) -> Balance;

    /// Returns the account balance for the specified `owner`.
    ///
    /// Returns `0` if the account is non-existent.
    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> Balance;

    /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
    ///
    /// Returns `0` if no allowance has been set.
    #[ink(message)]
    fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance;

    /// Transfers `value` amount of tokens from the caller's account to account `to`
    /// with additional `data` in unspecified format.
    ///
    /// On success a `Transfer` event is emitted.
    ///
    /// # Errors
    ///
    /// Reverts with error `InsufficientBalance` if there are not enough tokens on
    /// the caller's account Balance.
    ///
    /// Reverts with error `ZeroSenderAddress` if sender's address is zero.
    ///
    /// Reverts with error `ZeroRecipientAddress` if recipient's address is zero.
    ///
    /// Reverts with error `SafeTransferCheckFailed` if the recipient is a contract and rejected the transfer.
    #[ink(message)]
    fn transfer(&mut self, to: AccountId, value: Balance, data: Vec<u8>) -> Result<(), PSP22Error>;

    /// Transfers `value` tokens on the behalf of `from` to the account `to`
    /// with additional `data` in unspecified format.
    ///
    /// This can be used to allow a contract to transfer tokens on ones behalf and/or
    /// to charge fees in sub-currencies, for example.
    ///
    /// On success a `Transfer` and `Approval` events are emitted.
    ///
    /// # Errors
    ///
    /// Reverts with error `InsufficientAllowance` if there are not enough tokens allowed
    /// for the caller to withdraw from `from`.
    ///
    /// Reverts with error `InsufficientBalance` if there are not enough tokens on
    /// the the account Balance of `from`.
    ///
    /// Reverts with error `ZeroSenderAddress` if sender's address is zero.
    ///
    /// Reverts with error `ZeroRecipientAddress` if recipient's address is zero.
    #[ink(message)]
    fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        value: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP22Error>;

    /// Allows `spender` to withdraw from the caller's account multiple times, up to
    /// the `value` amount.
    ///
    /// If this function is called again it overwrites the current allowance with `value`.
    ///
    /// An `Approval` event is emitted.
    ///
    /// # Errors
    ///
    /// Reverts with error `ZeroSenderAddress` if sender's address is zero.
    ///
    /// Reverts with error `ZeroRecipientAddress` if recipient's address is zero.
    #[ink(message)]
    fn approve(&mut self, spender: AccountId, amount: Balance) -> Result<(), PSP22Error>;

    /// Atomically increases the allowance granted to `spender` by the caller.
    ///
    /// An `Approval` event is emitted.
    ///
    /// # Errors
    ///
    /// Reverts with error `ZeroSenderAddress` if sender's address is zero.
    ///
    /// Reverts with error `ZeroRecipientAddress` if recipient's address is zero.
    #[ink(message)]
    fn increase_allowance(&mut self, spender: AccountId, by: Balance) -> Result<(), PSP22Error>;

    /// Atomically decreases the allowance granted to `spender` by the caller.
    ///
    /// An `Approval` event is emitted.
    ///
    /// # Errors
    ///
    /// Reverts with error `InsufficientAllowance` if there are not enough tokens allowed
    /// by owner for `spender`.
    ///
    /// Reverts with error `ZeroSenderAddress` if sender's address is zero.
    ///
    /// Reverts with error `ZeroRecipientAddress` if recipient's address is zero.
    #[ink(message)]
    fn decrease_allowance(&mut self, spender: AccountId, by: Balance) -> Result<(), PSP22Error>;
}
