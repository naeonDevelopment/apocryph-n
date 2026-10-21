//! C1 canonical TSV: bounded ASCII tokens; no quoting, escaping, floats or JSON.
use crate::commerce::*;
pub const MAX_RECORD: usize = 8192;
pub const MAX_TOKEN: usize = 64;
pub fn identifier(s: &str) -> Result<()> {
    if s.is_empty() || s.len() > MAX_TOKEN || !s.bytes().all(|b| b.is_ascii_alphanumeric() || b"._:-".contains(&b)) { return Err(Error::Invalid); }
    Ok(())
}
fn asset(v: &mut Vec<String>, a: &Asset) { v.extend([a.namespace.clone(), a.network.clone(), a.identifier.clone(), a.scale.to_string()]); }
fn amount(v: &mut Vec<String>, a: &Amount) { asset(v, &a.asset); v.push(a.minor.to_string()); }
fn offer(v: &mut Vec<String>, o: &Offer) {
    v.extend([o.id.clone(), o.provider.clone(), o.epoch.to_string(), o.version.to_string(), o.resource.clone(), o.capacity.to_string(), o.starts.to_string(), o.ends.to_string(), o.expires.to_string()]);
    amount(v, &o.rate.price); v.extend([o.rate.unit.0.clone(), o.rate.per_units.to_string()]);
}
pub fn encode(c: &Command) -> Result<String> {
    let mut v = vec!["C1".into(), c.id.clone(), c.now.to_string()];
    match &c.op {
        Operation::Epoch { provider, epoch } => v.extend(["epoch".into(), provider.clone(), epoch.to_string()]),
        Operation::Account { id, limit } => { v.extend(["account".into(), id.clone()]); amount(&mut v, limit); }
        Operation::Offer(o) => { v.push("offer".into()); offer(&mut v, o); }
        Operation::Reserve(r) => {
            v.extend(["reserve".into(), r.id.clone(), r.account.clone(), r.offer.clone(), r.provider.clone(), r.epoch.to_string(), r.version.to_string(), r.resource.clone(), r.unit.0.clone(), r.quantity.to_string(), r.start.to_string(), r.end.to_string(), r.expires.to_string()]);
            amount(&mut v, &r.maximum);
        }
        Operation::Close { reservation } => v.extend(["close".into(), reservation.clone()]),
        Operation::Expire { reservation } => v.extend(["expire".into(), reservation.clone()]),
        Operation::Attempt(a) => v.extend(["attempt".into(), a.id.clone(), a.reservation.clone(), a.invoice.clone()]),
        Operation::Accept(a) => v.extend(["accept".into(), a.id.clone(), a.attempt.clone(), a.reservation.clone(), a.invoice.clone(), a.evidence.clone(), a.unit.0.clone(), a.cumulative.to_string()]),
        Operation::Collect { id, invoice, amount: a } => { v.extend(["collect".into(), id.clone(), invoice.clone()]); amount(&mut v, a); }
        Operation::Submit { intent } => v.extend(["submit".into(), intent.clone()]),
        Operation::Reconcile { intent, evidence, paid } => v.extend(["reconcile".into(), intent.clone(), evidence.clone(), if *paid { "paid" } else { "failed" }.into()]),
        Operation::Credit(c) => { v.extend(["credit".into(), c.id.clone(), c.invoice.clone()]); amount(&mut v, &c.amount); v.extend([if c.from_paid { "paid" } else { "unpaid" }.into(), c.evidence.clone()]); }
        Operation::Refund { credit, evidence } => v.extend(["refund".into(), credit.clone(), evidence.clone()]),
        Operation::Dispute { invoice, amount: a, release } => { v.extend([if *release { "release-dispute" } else { "dispute" }.into(), invoice.clone()]); amount(&mut v, a); }
    }
    for token in &v { identifier(token)?; }
    let text = v.join("\t");
    if text.len() > MAX_RECORD { return Err(Error::Limit); }
    Ok(text)
}
struct Fields<'a> { values: std::str::Split<'a, char> }
impl<'a> Fields<'a> {
    fn raw(&mut self) -> Result<&'a str> { self.values.next().ok_or(Error::Invalid) }
    fn text(&mut self) -> Result<String> { Ok(self.raw()?.to_owned()) }
    fn number<T: std::str::FromStr>(&mut self) -> Result<T> {
        let s = self.raw()?;
        if !s.bytes().all(|b| b.is_ascii_digit()) || (s.len() > 1 && s.starts_with('0')) { return Err(Error::Invalid); }
        s.parse().map_err(|_| Error::Overflow)
    }
    fn asset(&mut self) -> Result<Asset> {
        let a = Asset { namespace: self.text()?, network: self.text()?, identifier: self.text()?, scale: self.number()? };
        a.validate()?; Ok(a)
    }
    fn amount(&mut self) -> Result<Amount> { Ok(Amount { asset: self.asset()?, minor: self.number()? }) }
}
pub fn decode(text: &str) -> Result<Command> {
    if text.len() > MAX_RECORD { return Err(Error::Limit); }
    for token in text.split('\t') { identifier(token)?; }
    let mut f = Fields { values: text.split('\t') };
    if f.raw()? != "C1" { return Err(Error::Invalid); }
    let id = f.text()?; let now = f.number()?;
    let op = match f.raw()? {
        "epoch" => Operation::Epoch { provider: f.text()?, epoch: f.number()? },
        "account" => Operation::Account { id: f.text()?, limit: f.amount()? },
        "offer" => Operation::Offer(Offer { id: f.text()?, provider: f.text()?, epoch: f.number()?, version: f.number()?, resource: f.text()?, capacity: f.number()?, starts: f.number()?, ends: f.number()?, expires: f.number()?, rate: Rate { price: f.amount()?, unit: ServiceUnit(f.text()?), per_units: f.number()? } }),
        "reserve" => Operation::Reserve(Reservation { id: f.text()?, account: f.text()?, offer: f.text()?, provider: f.text()?, epoch: f.number()?, version: f.number()?, resource: f.text()?, unit: ServiceUnit(f.text()?), quantity: f.number()?, start: f.number()?, end: f.number()?, expires: f.number()?, maximum: f.amount()? }),
        "close" => Operation::Close { reservation: f.text()? },
        "expire" => Operation::Expire { reservation: f.text()? },
        "attempt" => Operation::Attempt(Attempt { id: f.text()?, reservation: f.text()?, invoice: f.text()? }),
        "accept" => Operation::Accept(Acceptance { id: f.text()?, attempt: f.text()?, reservation: f.text()?, invoice: f.text()?, evidence: f.text()?, unit: ServiceUnit(f.text()?), cumulative: f.number()? }),
        "collect" => Operation::Collect { id: f.text()?, invoice: f.text()?, amount: f.amount()? },
        "submit" => Operation::Submit { intent: f.text()? },
        "reconcile" => Operation::Reconcile { intent: f.text()?, evidence: f.text()?, paid: match f.raw()? { "paid" => true, "failed" => false, _ => return Err(Error::Invalid) } },
        "credit" => Operation::Credit(Credit { id: f.text()?, invoice: f.text()?, amount: f.amount()?, from_paid: match f.raw()? { "paid" => true, "unpaid" => false, _ => return Err(Error::Invalid) }, evidence: f.text()? }),
        "refund" => Operation::Refund { credit: f.text()?, evidence: f.text()? },
        "dispute" => Operation::Dispute { invoice: f.text()?, amount: f.amount()?, release: false },
        "release-dispute" => Operation::Dispute { invoice: f.text()?, amount: f.amount()?, release: true },
        _ => return Err(Error::Invalid),
    };
    if f.values.next().is_some() { return Err(Error::Invalid); }
    let c = Command { id, now, op };
    if encode(&c)? != text { return Err(Error::Invalid); }
    Ok(c)
}
