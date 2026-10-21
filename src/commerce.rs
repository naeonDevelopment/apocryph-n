//! Bounded deterministic commerce. All external evidence is an unauthenticated assertion.
use std::collections::BTreeMap;

pub const MAX_COMMANDS: usize = 100_000;
pub const MAX_SPAN: u64 = 31_536_000;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error { Invalid, Overflow, Mismatch, DuplicateMismatch, Missing, Stale,
    Insufficient, Transition, Limit, Corrupt, Locked, Io, Poisoned }
pub type Result<T> = std::result::Result<T, Error>;
fn add(a: u128, b: u128) -> Result<u128> { a.checked_add(b).ok_or(Error::Overflow) }
fn sub(a: u128, b: u128) -> Result<u128> { a.checked_sub(b).ok_or(Error::Insufficient) }

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Asset { pub namespace: String, pub network: String, pub identifier: String, pub scale: u8 }
impl Asset {
    pub fn validate(&self) -> Result<()> {
        for s in [&self.namespace, &self.network, &self.identifier] { crate::commerce_wire::identifier(s)?; }
        if self.scale > 38 { return Err(Error::Invalid); }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ServiceUnit(pub String);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Amount { pub asset: Asset, pub minor: u128 }
impl Amount {
    pub fn checked_add(&self, other: &Self) -> Result<Self> {
        self.asset.validate()?; other.asset.validate()?;
        if self.asset != other.asset { return Err(Error::Mismatch); }
        Ok(Self { asset: self.asset.clone(), minor: add(self.minor, other.minor)? })
    }
    pub fn checked_sub(&self, other: &Self) -> Result<Self> {
        self.asset.validate()?; other.asset.validate()?;
        if self.asset != other.asset { return Err(Error::Mismatch); }
        Ok(Self { asset: self.asset.clone(), minor: sub(self.minor, other.minor)? })
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rate { pub price: Amount, pub unit: ServiceUnit, pub per_units: u128 }
impl Rate {
    pub fn charge(&self, unit: &ServiceUnit, cumulative: u128) -> Result<Amount> {
        self.price.asset.validate()?;
        crate::commerce_wire::identifier(&self.unit.0)?;
        if unit != &self.unit { return Err(Error::Mismatch); }
        if self.per_units == 0 { return Err(Error::Invalid); }
        let numerator = cumulative.checked_mul(self.price.minor).ok_or(Error::Overflow)?;
        Ok(Amount { asset: self.price.asset.clone(), minor: numerator / self.per_units })
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Offer {
    pub id: String, pub provider: String, pub epoch: u64, pub version: u64,
    pub resource: String, pub capacity: u64, pub starts: u64, pub ends: u64,
    pub expires: u64, pub rate: Rate,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reservation {
    pub id: String, pub account: String, pub offer: String, pub provider: String,
    pub epoch: u64, pub version: u64, pub resource: String, pub unit: ServiceUnit,
    pub quantity: u64, pub start: u64, pub end: u64, pub expires: u64, pub maximum: Amount,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attempt { pub id: String, pub reservation: String, pub invoice: String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Acceptance {
    pub id: String, pub attempt: String, pub reservation: String, pub invoice: String,
    pub evidence: String, pub unit: ServiceUnit, pub cumulative: u128,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionState { Ready, Unknown, Paid, Failed }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Intent { pub id: String, pub invoice: String, pub amount: Amount, pub state: CollectionState, pub evidence: Option<String> }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Credit { pub id: String, pub invoice: String, pub amount: Amount, pub from_paid: bool, pub evidence: String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invoice {
    pub id: String, pub reservation: String, pub account: String, pub offer: Offer,
    pub cumulative: u128, pub gross: u128, pub credited: u128, pub paid: u128,
    pub reserved: u128, pub disputed: u128, pub refund_due: u128, pub refunded: u128,
}
impl Invoice {
    /// gross = collectible + reserved + disputed + paid + credited.
    /// paid is retained cash allocation, excluding credited paid value.
    pub fn collectible(&self) -> Result<u128> {
        sub(self.gross, add(add(self.credited, self.paid)?, add(self.reserved, self.disputed)?)?)
    }
    pub fn audit(&self) -> Result<Audit> {
        let collectible = self.collectible()?;
        if add(self.refund_due, self.refunded)? > self.credited { return Err(Error::Invalid); }
        Ok(Audit { gross: self.gross, collectible, reserved: self.reserved, paid: self.paid,
            credited: self.credited, disputed: self.disputed, refund_due: self.refund_due,
            refunded: self.refunded, cash_received: add(add(self.paid, self.refund_due)?, self.refunded)? })
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Audit { pub gross: u128, pub collectible: u128, pub reserved: u128, pub paid: u128,
    pub credited: u128, pub disputed: u128, pub refund_due: u128, pub refunded: u128, pub cash_received: u128 }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    Epoch { provider: String, epoch: u64 },
    Account { id: String, limit: Amount },
    Offer(Offer), Reserve(Reservation),
    Close { reservation: String },
    Expire { reservation: String },
    Attempt(Attempt), Accept(Acceptance),
    Collect { id: String, invoice: String, amount: Amount },
    Submit { intent: String },
    Reconcile { intent: String, evidence: String, paid: bool },
    Credit(Credit),
    Refund { credit: String, evidence: String },
    Dispute { invoice: String, amount: Amount, release: bool },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command { pub id: String, pub now: u64, pub op: Operation }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Applied { New, Replay }
#[derive(Debug, Clone, PartialEq, Eq)]
struct Held { request: Reservation, closed: bool, invoice: Option<String> }
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Commerce {
    now: u64, commands: BTreeMap<String, String>, epochs: BTreeMap<String, u64>,
    accounts: BTreeMap<String, Amount>, offers: BTreeMap<String, Offer>,
    reservations: BTreeMap<String, Held>, attempts: BTreeMap<String, Attempt>,
    acceptances: BTreeMap<String, Acceptance>, invoices: BTreeMap<String, Invoice>,
    intents: BTreeMap<String, Intent>, credits: BTreeMap<String, Credit>,
    refunds: BTreeMap<String, String>, evidence: BTreeMap<String, String>,
}
impl Commerce {
    pub fn invoices(&self) -> impl Iterator<Item = &Invoice> { self.invoices.values() }
    pub fn intents(&self) -> impl Iterator<Item = &Intent> { self.intents.values() }
    pub fn credits(&self) -> impl Iterator<Item = &Credit> { self.credits.values() }
    pub fn invoice(&self, id: &str) -> Option<&Invoice> { self.invoices.get(id) }
    pub fn intent(&self, id: &str) -> Option<&Intent> { self.intents.get(id) }
    pub fn command_count(&self) -> usize { self.commands.len() }
    pub fn apply(&mut self, command: &Command) -> Result<Applied> {
        let bytes = crate::commerce_wire::encode(command)?;
        if let Some(old) = self.commands.get(&command.id) {
            return if old == &bytes { Ok(Applied::Replay) } else { Err(Error::DuplicateMismatch) };
        }
        if self.commands.len() >= MAX_COMMANDS { return Err(Error::Limit); }
        if command.now < self.now { return Err(Error::Stale); }
        let mut next = self.clone();
        next.now = command.now;
        next.execute(&command.op)?;
        next.audit()?;
        next.commands.insert(command.id.clone(), bytes);
        *self = next;
        Ok(Applied::New)
    }
    fn epoch(&self, provider: &str, epoch: u64) -> Result<()> {
        if self.epochs.get(provider) != Some(&epoch) { return Err(Error::Stale); }
        Ok(())
    }
    fn live(&self, id: &str) -> Result<&Held> {
        let h = self.reservations.get(id).ok_or(Error::Missing)?;
        self.epoch(&h.request.provider, h.request.epoch)?;
        if h.closed || self.now >= h.request.expires { return Err(Error::Stale); }
        Ok(h)
    }
    fn claim_evidence(&mut self, evidence: &str, binding: String) -> Result<()> {
        if let Some(old) = self.evidence.get(evidence) {
            return if old == &binding { Ok(()) } else { Err(Error::DuplicateMismatch) };
        }
        self.evidence.insert(evidence.into(), binding); Ok(())
    }
    /// Active holds use the full maximum; closed obligations use gross minus permanent credits.
    pub fn exposure(&self, account: &str) -> Result<Amount> {
        let limit = self.accounts.get(account).ok_or(Error::Missing)?;
        let mut total = 0;
        for h in self.reservations.values().filter(|h| h.request.account == account) {
            let used = if !h.closed { h.request.maximum.minor } else {
                match &h.invoice { Some(id) => { let i = &self.invoices[id]; sub(i.gross, i.credited)? }, None => 0 }
            };
            total = add(total, used)?;
        }
        Ok(Amount { asset: limit.asset.clone(), minor: total })
    }
    fn audit(&self) -> Result<()> {
        for i in self.invoices.values() { i.audit()?; }
        for (id, limit) in &self.accounts {
            if self.exposure(id)?.minor > limit.minor { return Err(Error::Insufficient); }
        }
        Ok(())
    }
    fn capacity(&self, provider: &str, resource: &str, capacity: u64) -> Result<()> {
        let rows: Vec<_> = self.reservations.values().filter(|h| !h.closed && h.request.provider == provider && h.request.resource == resource).collect();
        // Occupancy changes only at starts/ends; half-open endpoints permit adjacency.
        for h in &rows {
            let at = h.request.start;
            let mut used = 0u128;
            for r in &rows {
                if r.request.start <= at && at < r.request.end { used = add(used, r.request.quantity as u128)?; }
            }
            if used > capacity as u128 { return Err(Error::Insufficient); }
        }
        Ok(())
    }
    fn execute(&mut self, op: &Operation) -> Result<()> {
        match op {
            Operation::Epoch { provider, epoch } => {
                if *epoch == 0 { return Err(Error::Invalid); }
                if let Some(old) = self.epochs.get(provider) {
                    if epoch < old { return Err(Error::Stale); }
                }
                self.epochs.insert(provider.clone(), *epoch);
            }
            Operation::Account { id, limit } => {
                limit.asset.validate()?;
                if let Some(old) = self.accounts.get(id) { return identical(old, limit); }
                self.accounts.insert(id.clone(), limit.clone());
            }
            Operation::Offer(o) => {
                if let Some(old) = self.offers.get(&o.id) { return identical(old, o); }
                self.epoch(&o.provider, o.epoch)?;
                span(o.starts, o.ends)?;
                o.rate.charge(&o.rate.unit, 0)?;
                if o.version == 0 || o.capacity == 0 || o.expires <= self.now || o.expires > o.ends { return Err(Error::Invalid); }
                for old in self.offers.values().filter(|v| v.provider == o.provider && v.resource == o.resource) {
                    if o.rate.unit != old.rate.unit { return Err(Error::Mismatch); }
                    if o.epoch < old.epoch || (o.epoch == old.epoch && o.version <= old.version) { return Err(Error::Stale); }
                }
                self.capacity(&o.provider, &o.resource, o.capacity)?;
                self.offers.insert(o.id.clone(), o.clone());
            }
            Operation::Reserve(r) => {
                if let Some(old) = self.reservations.get(&r.id) { return identical(&old.request, r); }
                let o = self.offers.get(&r.offer).ok_or(Error::Missing)?.clone();
                self.epoch(&r.provider, r.epoch)?;
                if self.offers.values().any(|v| v.provider == o.provider && v.resource == o.resource && (v.epoch, v.version) > (o.epoch, o.version)) { return Err(Error::Stale); }
                if r.provider != o.provider || r.epoch != o.epoch || r.version != o.version || r.resource != o.resource || r.unit != o.rate.unit || r.maximum.asset != o.rate.price.asset { return Err(Error::Mismatch); }
                let limit = self.accounts.get(&r.account).ok_or(Error::Missing)?;
                if limit.asset != r.maximum.asset { return Err(Error::Mismatch); }
                let duration = span(r.start, r.end)?;
                if r.quantity == 0 || r.start < self.now || r.start < o.starts || r.end > o.ends || self.now >= o.expires || r.expires <= self.now || r.expires < r.end || r.expires > o.ends { return Err(Error::Invalid); }
                let units = (r.quantity as u128).checked_mul(duration as u128).ok_or(Error::Overflow)?;
                if o.rate.charge(&r.unit, units)?.minor > r.maximum.minor { return Err(Error::Insufficient); }
                self.reservations.insert(r.id.clone(), Held { request: r.clone(), closed: false, invoice: None });
                self.capacity(&r.provider, &r.resource, o.capacity)?;
            }
            Operation::Close { reservation } | Operation::Expire { reservation } => {
                let h = self.reservations.get_mut(reservation).ok_or(Error::Missing)?;
                if matches!(op, Operation::Expire { .. }) && self.now < h.request.expires { return Err(Error::Transition); }
                h.closed = true;
            }
            Operation::Attempt(a) => {
                if let Some(old) = self.attempts.get(&a.id) { return identical(old, a); }
                let h = self.live(&a.reservation)?.clone();
                if self.now < h.request.start || self.now >= h.request.end { return Err(Error::Stale); }
                if let Some(invoice) = &h.invoice {
                    if invoice != &a.invoice { return Err(Error::Mismatch); }
                } else {
                    if self.invoices.contains_key(&a.invoice) { return Err(Error::DuplicateMismatch); }
                    let o = self.offers[&h.request.offer].clone();
                    self.invoices.insert(a.invoice.clone(), Invoice { id: a.invoice.clone(), reservation: a.reservation.clone(), account: h.request.account.clone(), offer: o,
                        cumulative: 0, gross: 0, credited: 0, paid: 0, reserved: 0, disputed: 0, refund_due: 0, refunded: 0 });
                    self.reservations.get_mut(&a.reservation).ok_or(Error::Missing)?.invoice = Some(a.invoice.clone());
                }
                self.attempts.insert(a.id.clone(), a.clone());
            }
            Operation::Accept(a) => {
                if let Some(old) = self.acceptances.get(&a.id) { return identical(old, a); }
                let h = self.live(&a.reservation)?.clone();
                let attempt = self.attempts.get(&a.attempt).ok_or(Error::Missing)?;
                if attempt.reservation != a.reservation || attempt.invoice != a.invoice { return Err(Error::Mismatch); }
                if self.now < h.request.start { return Err(Error::Stale); }
                let elapsed = self.now.min(h.request.end) - h.request.start;
                let bound = (h.request.quantity as u128).checked_mul(elapsed as u128).ok_or(Error::Overflow)?;
                if a.cumulative > bound { return Err(Error::Insufficient); }
                self.claim_evidence(&a.evidence, format!("accept:{}", a.id))?;
                let i = self.invoices.get_mut(&a.invoice).ok_or(Error::Missing)?;
                if a.cumulative < i.cumulative { return Err(Error::Stale); }
                let gross = i.offer.rate.charge(&a.unit, a.cumulative)?.minor;
                if gross > h.request.maximum.minor { return Err(Error::Insufficient); }
                i.cumulative = a.cumulative; i.gross = gross;
                self.acceptances.insert(a.id.clone(), a.clone());
            }
            Operation::Collect { id, invoice, amount } => {
                if let Some(old) = self.intents.get(id) {
                    return if old.invoice == *invoice && old.amount == *amount { Ok(()) } else { Err(Error::DuplicateMismatch) };
                }
                let i = self.invoices.get_mut(invoice).ok_or(Error::Missing)?;
                money(i, amount)?;
                if amount.minor > i.collectible()? { return Err(Error::Insufficient); }
                i.reserved = add(i.reserved, amount.minor)?;
                self.intents.insert(id.clone(), Intent { id: id.clone(), invoice: invoice.clone(), amount: amount.clone(), state: CollectionState::Ready, evidence: None });
            }
            Operation::Submit { intent } => {
                let t = self.intents.get_mut(intent).ok_or(Error::Missing)?;
                // Durable claim consumes eligibility BEFORE any hypothetical adapter call.
                if t.state != CollectionState::Ready { return Err(Error::Transition); }
                t.state = CollectionState::Unknown;
            }
            Operation::Reconcile { intent, evidence, paid } => {
                let t = self.intents.get(intent).ok_or(Error::Missing)?.clone();
                let terminal = if *paid { CollectionState::Paid } else { CollectionState::Failed };
                if t.state == terminal && t.evidence.as_ref() == Some(evidence) { return Ok(()); }
                if t.state != CollectionState::Unknown { return Err(Error::Transition); }
                self.claim_evidence(evidence, format!("collect:{}:{}", intent, paid))?;
                let i = self.invoices.get_mut(&t.invoice).ok_or(Error::Missing)?;
                i.reserved = sub(i.reserved, t.amount.minor)?;
                if *paid { i.paid = add(i.paid, t.amount.minor)?; }
                let t = self.intents.get_mut(intent).ok_or(Error::Missing)?;
                t.state = terminal; t.evidence = Some(evidence.clone());
            }
            Operation::Credit(c) => {
                if let Some(old) = self.credits.get(&c.id) { return identical(old, c); }
                self.claim_evidence(&c.evidence, format!("credit:{}", c.id))?;
                let i = self.invoices.get_mut(&c.invoice).ok_or(Error::Missing)?;
                money(i, &c.amount)?;
                if c.from_paid {
                    i.paid = sub(i.paid, c.amount.minor)?;
                    i.refund_due = add(i.refund_due, c.amount.minor)?;
                } else {
                    if c.amount.minor > i.collectible()? { return Err(Error::Insufficient); }
                }
                i.credited = add(i.credited, c.amount.minor)?;
                self.credits.insert(c.id.clone(), c.clone());
            }
            Operation::Refund { credit, evidence } => {
                if let Some(old) = self.refunds.get(credit) { return identical(old, evidence); }
                let c = self.credits.get(credit).ok_or(Error::Missing)?.clone();
                if !c.from_paid { return Err(Error::Transition); }
                self.claim_evidence(evidence, format!("refund:{}", credit))?;
                let i = self.invoices.get_mut(&c.invoice).ok_or(Error::Missing)?;
                i.refund_due = sub(i.refund_due, c.amount.minor)?;
                i.refunded = add(i.refunded, c.amount.minor)?;
                self.refunds.insert(credit.clone(), evidence.clone());
            }
            Operation::Dispute { invoice, amount, release } => {
                let i = self.invoices.get_mut(invoice).ok_or(Error::Missing)?;
                money(i, amount)?;
                if *release { i.disputed = sub(i.disputed, amount.minor)?; }
                else {
                    if amount.minor > i.collectible()? { return Err(Error::Insufficient); }
                    i.disputed = add(i.disputed, amount.minor)?;
                }
            }
        }
        Ok(())
    }
}
fn identical<T: PartialEq>(old: &T, new: &T) -> Result<()> {
    if old == new { Ok(()) } else { Err(Error::DuplicateMismatch) }
}
fn span(start: u64, end: u64) -> Result<u64> {
    let d = end.checked_sub(start).ok_or(Error::Invalid)?;
    if d == 0 || d > MAX_SPAN { return Err(Error::Invalid); }
    Ok(d)
}
fn money(i: &Invoice, amount: &Amount) -> Result<()> {
    if i.offer.rate.price.asset != amount.asset { return Err(Error::Mismatch); }
    if amount.minor == 0 { return Err(Error::Invalid); }
    Ok(())
}
