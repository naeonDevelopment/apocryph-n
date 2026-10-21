use std::collections::BTreeMap;
use crate::{add, Error, Result};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State { Pending, Unknown, Failed, Paid, Refunded }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Intent { pub amount: u128, pub state: State }
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Balances {
    pub accrued: u128, pub invoiced: u128, pub reserved: u128,
    pub confirmed_paid: u128, pub disputed: u128,
}
/// Single-owner, in-memory ledger. External reconciliation is trusted input only.
#[derive(Debug, Clone)]
pub struct Settlement {
    app: String, version: String, balances: Balances, intents: BTreeMap<String, Intent>,
}
impl Settlement {
    pub fn new(app: String, version: String) -> Result<Self> {
        if app.is_empty() || version.is_empty() { return Err(Error::Invalid); }
        Ok(Self { app, version, balances: Balances::default(), intents: BTreeMap::new() })
    }
    pub fn identity(&self) -> (&str, &str) { (&self.app, &self.version) }
    pub fn balances(&self) -> Balances { self.balances }
    pub fn intent(&self, id: &str) -> Option<&Intent> { self.intents.get(id) }
    /// Set monotonically increasing cumulative accrual, avoiding replayed increments.
    pub fn accrue_to(&mut self, total: u128) -> Result<()> {
        if total < self.balances.accrued { return Err(Error::Invalid); }
        self.balances.accrued = total; Ok(())
    }
    pub fn invoice_to(&mut self, total: u128) -> Result<()> {
        if total < self.balances.invoiced || total > self.balances.accrued { return Err(Error::Invalid); }
        self.balances.invoiced = total; Ok(())
    }
    pub fn available(&self) -> Result<u128> {
        let used = add(add(self.balances.confirmed_paid, self.balances.reserved)?, self.balances.disputed)?;
        self.balances.invoiced.checked_sub(used).ok_or(Error::Insufficient)
    }
    pub fn dispute(&mut self, amount: u128) -> Result<()> {
        if amount == 0 || amount > self.available()? { return Err(Error::Insufficient); }
        self.balances.disputed = add(self.balances.disputed, amount)?; Ok(())
    }
    /// Release a dispute back into collectible balance; no adjudication is performed.
    pub fn release_dispute(&mut self, amount: u128) -> Result<()> {
        if amount == 0 { return Err(Error::Invalid); }
        self.balances.disputed = self.balances.disputed.checked_sub(amount).ok_or(Error::Insufficient)?; Ok(())
    }
    /// Existing ID/amount succeeds without changing state, including terminal states.
    /// Success is NOT permission to submit a fresh transfer.
    pub fn reserve(&mut self, id: String, amount: u128) -> Result<()> {
        if id.is_empty() || amount == 0 { return Err(Error::Invalid); }
        if let Some(old) = self.intents.get(&id) {
            return if old.amount == amount { Ok(()) } else { Err(Error::DuplicateMismatch) };
        }
        if amount > self.available()? { return Err(Error::Insufficient); }
        let reserved = add(self.balances.reserved, amount)?;
        self.intents.insert(id, Intent { amount, state: State::Pending });
        self.balances.reserved = reserved; Ok(())
    }
    /// Pending -> Unknown on timeout. Only reconciliation may resolve Unknown.
    /// Paid -> Refunded moves value into disputed, requiring explicit release.
    /// Same-state requests are no-ops even without a new reconciliation assertion.
    /// Refunded is a local balance state, not a legal credit note or invoice credit.
    pub fn transition(&mut self, id: &str, next: State, reconciled: bool) -> Result<()> {
        let intent = self.intents.get(id).ok_or(Error::Missing)?;
        let old = intent.state;
        let amount = intent.amount;
        if old == next { return Ok(()); }
        let mut b = self.balances;
        match (old, next) {
            (State::Pending, State::Unknown) => {},
            (State::Pending, State::Paid | State::Failed) |
            (State::Unknown, State::Paid | State::Failed) => {
                if !reconciled { return Err(Error::Transition); }
                b.reserved = b.reserved.checked_sub(amount).ok_or(Error::Insufficient)?;
                if next == State::Paid { b.confirmed_paid = add(b.confirmed_paid, amount)?; }
            },
            (State::Paid, State::Refunded) if reconciled => {
                b.confirmed_paid = b.confirmed_paid.checked_sub(amount).ok_or(Error::Insufficient)?;
                b.disputed = add(b.disputed, amount)?;
            },
            _ => return Err(Error::Transition),
        }
        self.intents.get_mut(id).ok_or(Error::Missing)?.state = next;
        self.balances = b; Ok(())
    }
}
