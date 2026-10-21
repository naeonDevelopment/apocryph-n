use apocryph_n::{commerce::*, commerce_store::*, commerce_wire};
use std::{cell::RefCell, rc::Rc};
fn asset() -> Asset { Asset { namespace: "synthetic".into(), network: "offline".into(), identifier: "credit".into(), scale: 6 } }
fn amount(n: u128) -> Amount { Amount { asset: asset(), minor: n } }
fn unit() -> ServiceUnit { ServiceUnit("slot-tick".into()) }
fn cmd(id: &str, now: u64, op: Operation) -> Command { Command { id: id.into(), now, op } }
fn apply(s: &mut Commerce, id: &str, now: u64, op: Operation) { assert_eq!(s.apply(&cmd(id, now, op)), Ok(Applied::New)); }
fn offer() -> Offer { Offer { id: "o".into(), provider: "p".into(), epoch: 1, version: 1, resource: "cpu".into(), capacity: 2, starts: 0, ends: 30, expires: 25, rate: Rate { price: amount(2), unit: unit(), per_units: 1 } } }
fn reservation(id: &str) -> Reservation { Reservation { id: id.into(), account: "a".into(), offer: "o".into(), provider: "p".into(), epoch: 1, version: 1, resource: "cpu".into(), unit: unit(), quantity: 1, start: 0, end: 10, expires: 20, maximum: amount(20) } }
fn bootstrap() -> Vec<Command> { vec![
    cmd("epoch", 0, Operation::Epoch { provider: "p".into(), epoch: 1 }),
    cmd("account", 0, Operation::Account { id: "a".into(), limit: amount(100) }),
    cmd("offer", 0, Operation::Offer(offer())),
    cmd("reserve", 0, Operation::Reserve(reservation("r"))),
    cmd("attempt", 0, Operation::Attempt(Attempt { id: "t".into(), reservation: "r".into(), invoice: "i".into() })),
] }
fn setup() -> Commerce { let mut s = Commerce::default(); for c in bootstrap() { s.apply(&c).unwrap(); } s }
fn acceptance(id: &str, cumulative: u128) -> Acceptance { Acceptance { id: id.into(), attempt: "t".into(), reservation: "r".into(), invoice: "i".into(), evidence: format!("e-{id}"), unit: unit(), cumulative } }
fn billed() -> Commerce { let mut s = setup(); apply(&mut s, "accept", 10, Operation::Accept(acceptance("receipt", 10))); s }
fn collect() -> Operation { Operation::Collect { id: "pay".into(), invoice: "i".into(), amount: amount(20) } }
fn submit() -> Operation { Operation::Submit { intent: "pay".into() } }
fn reconcile(paid: bool) -> Operation { Operation::Reconcile { intent: "pay".into(), evidence: "rail-result".into(), paid } }
fn paid() -> Commerce { let mut s = billed(); apply(&mut s, "collect", 10, collect()); apply(&mut s, "submit", 10, submit()); apply(&mut s, "reconcile", 10, reconcile(true)); s }
fn rejected(s: &mut Commerce, c: Command, expected: Error) { let old = s.clone(); assert_eq!(s.apply(&c), Err(expected)); assert_eq!(*s, old, "failure must roll back every map and clock"); }

#[test] fn typed_assets_units_scale_and_overflow() {
    assert_eq!(amount(1).checked_add(&amount(2)).unwrap().minor, 3);
    assert_eq!(amount(u128::MAX).checked_add(&amount(1)), Err(Error::Overflow));
    assert_eq!(amount(0).checked_sub(&amount(1)), Err(Error::Insufficient));
    for field in 0..4 {
        let mut wrong = amount(1);
        match field { 0 => wrong.asset.namespace = "else".into(), 1 => wrong.asset.network = "else".into(), 2 => wrong.asset.identifier = "else".into(), _ => wrong.asset.scale = 5 }
        assert_eq!(amount(1).checked_add(&wrong), Err(Error::Mismatch));
        let mut s = setup(); let mut r = reservation("other"); r.maximum = wrong;
        rejected(&mut s, cmd("bad", 0, Operation::Reserve(r)), Error::Mismatch);
    }
    let rate = offer().rate;
    assert_eq!(rate.charge(&ServiceUnit("byte-tick".into()), 1), Err(Error::Mismatch));
    assert_eq!(rate.charge(&unit(), u128::MAX), Err(Error::Overflow));
    let mut invalid = asset(); invalid.scale = 39; assert_eq!(invalid.validate(), Err(Error::Invalid));
}
#[test] fn atomic_budget_capacity_and_interval_overlap() {
    let mut s = setup();
    apply(&mut s, "second", 0, Operation::Reserve(reservation("r2")));
    rejected(&mut s, cmd("third", 0, Operation::Reserve(reservation("r3"))), Error::Insufficient);
    assert_eq!(s.exposure("a").unwrap().minor, 40);
    let mut adjacent = reservation("next"); adjacent.start = 10; adjacent.end = 20; adjacent.expires = 25;
    apply(&mut s, "adjacent", 0, Operation::Reserve(adjacent));
    let mut overlap = reservation("overlap"); overlap.start = 9; overlap.end = 11; overlap.expires = 20;
    rejected(&mut s, cmd("overlap", 0, Operation::Reserve(overlap)), Error::Insufficient);
    apply(&mut s, "small-account", 0, Operation::Account { id: "small".into(), limit: amount(19) });
    let mut r = reservation("budget"); r.account = "small".into(); r.start = 20; r.end = 30; r.expires = 30;
    rejected(&mut s, cmd("budget", 0, Operation::Reserve(r)), Error::Insufficient);
    let mut changed = offer(); changed.id = "o2".into(); changed.version = 2; changed.capacity = 1;
    rejected(&mut s, cmd("lower", 0, Operation::Offer(changed)), Error::Insufficient);
}
#[test] fn epoch_version_expiry_and_release_are_fenced() {
    let mut s = setup();
    let mut newer = offer(); newer.id = "new".into(); newer.version = 2; newer.rate.price = amount(3);
    apply(&mut s, "new-offer", 0, Operation::Offer(newer));
    rejected(&mut s, cmd("old-offer", 0, Operation::Reserve(reservation("old"))), Error::Stale);
    // Existing invoice keeps the original rate, despite a newer offer.
    apply(&mut s, "accepted", 1, Operation::Accept(acceptance("x", 1)));
    assert_eq!(s.invoice("i").unwrap().gross, 2);
    apply(&mut s, "fence", 1, Operation::Epoch { provider: "p".into(), epoch: 2 });
    rejected(&mut s, cmd("stale-accept", 2, Operation::Accept(acceptance("y", 2))), Error::Stale);
    rejected(&mut s, cmd("early-expiry", 2, Operation::Expire { reservation: "r".into() }), Error::Transition);
    apply(&mut s, "close", 2, Operation::Close { reservation: "r".into() });
    assert_eq!(s.exposure("a").unwrap().minor, 2);
    let mut expired = setup();
    rejected(&mut expired, cmd("late", 20, Operation::Accept(acceptance("late", 10))), Error::Stale);
    apply(&mut expired, "expire", 20, Operation::Expire { reservation: "r".into() });
    assert_eq!(expired.exposure("a").unwrap().minor, 0);
    rejected(&mut expired, cmd("resurrect", 20, Operation::Attempt(Attempt { id: "new-t".into(), reservation: "r".into(), invoice: "i".into() })), Error::Stale);
}
#[test] fn changed_identity_replay_never_mutates() {
    let mut s = setup(); let command = bootstrap().pop().unwrap();
    assert_eq!(s.apply(&command), Ok(Applied::Replay));
    let mut changed = command; changed.now = 1;
    rejected(&mut s, changed, Error::DuplicateMismatch);
    let mut r = reservation("r"); r.quantity = 2;
    rejected(&mut s, cmd("different-command", 0, Operation::Reserve(r)), Error::DuplicateMismatch);
    let mut o = offer(); o.capacity = 3;
    rejected(&mut s, cmd("different-offer", 0, Operation::Offer(o)), Error::DuplicateMismatch);
}
#[test] fn cumulative_origin_rounds_once_across_attempts() {
    let mut commands = bootstrap();
    if let Operation::Offer(o) = &mut commands[2].op { o.rate.price = amount(1); o.rate.per_units = 3; }
    let mut s = Commerce::default(); for c in commands { s.apply(&c).unwrap(); }
    apply(&mut s, "a1", 1, Operation::Accept(acceptance("one", 1)));
    assert_eq!(s.invoice("i").unwrap().gross, 0);
    apply(&mut s, "retry-attempt", 1, Operation::Attempt(Attempt { id: "t2".into(), reservation: "r".into(), invoice: "i".into() }));
    let mut second = acceptance("two", 2); second.attempt = "t2".into();
    apply(&mut s, "a2", 2, Operation::Accept(second.clone()));
    apply(&mut s, "a2-copy", 2, Operation::Accept(second));
    apply(&mut s, "a3", 3, Operation::Accept(acceptance("three", 3)));
    assert_eq!(s.invoice("i").unwrap().gross, 1);
    let mut duplicate = acceptance("alias", 3); duplicate.evidence = "e-three".into();
    rejected(&mut s, cmd("alias", 3, Operation::Accept(duplicate)), Error::DuplicateMismatch);
    rejected(&mut s, cmd("backwards", 3, Operation::Accept(acceptance("old", 2))), Error::Stale);
    rejected(&mut s, cmd("future", 3, Operation::Accept(acceptance("future", 4))), Error::Insufficient);
    let mut bad = acceptance("wrong", 3); bad.invoice = "other".into();
    rejected(&mut s, cmd("wrong-binding", 3, Operation::Accept(bad)), Error::Mismatch);
    let mut bad = acceptance("wrong-unit", 3); bad.unit = ServiceUnit("bytes".into());
    rejected(&mut s, cmd("wrong-unit", 3, Operation::Accept(bad)), Error::Mismatch);
}
#[test] fn timeout_then_bound_terminal_reconciliation() {
    let mut s = billed();
    apply(&mut s, "collect", 10, collect());
    rejected(&mut s, cmd("no-submit", 10, reconcile(true)), Error::Transition);
    apply(&mut s, "submit", 10, submit());
    assert_eq!(s.apply(&cmd("submit", 10, submit())), Ok(Applied::Replay));
    rejected(&mut s, cmd("second-submit", 10, submit()), Error::Transition);
    rejected(&mut s, cmd("second-collect", 10, Operation::Collect { id: "pay2".into(), invoice: "i".into(), amount: amount(1) }), Error::Insufficient);
    apply(&mut s, "expire", 20, Operation::Expire { reservation: "r".into() });
    assert_eq!(s.invoice("i").unwrap().reserved, 20);
    assert_eq!(s.exposure("a").unwrap().minor, 20);
    rejected(&mut s, cmd("empty-evidence", 20, Operation::Reconcile { intent: "pay".into(), evidence: "".into(), paid: false }), Error::Invalid);
    apply(&mut s, "failed", 20, reconcile(false));
    apply(&mut s, "failed-copy", 20, reconcile(false));
    rejected(&mut s, cmd("conflict", 20, reconcile(true)), Error::Transition);
    apply(&mut s, "terminal-id", 20, collect());
    assert_eq!(s.intent("pay").unwrap().state, CollectionState::Failed);
    assert_eq!(s.invoice("i").unwrap().collectible(), Ok(20));
}
#[test] fn permanent_credit_refund_and_dispute_conservation() {
    let mut s = paid();
    let credit = Credit { id: "c".into(), invoice: "i".into(), amount: amount(7), from_paid: true, evidence: "credit-evidence".into() };
    apply(&mut s, "credit", 10, Operation::Credit(credit.clone()));
    assert_eq!(s.invoice("i").unwrap().refund_due, 7);
    apply(&mut s, "refund", 10, Operation::Refund { credit: "c".into(), evidence: "refund-evidence".into() });
    apply(&mut s, "refund-copy", 10, Operation::Refund { credit: "c".into(), evidence: "refund-evidence".into() });
    apply(&mut s, "credit-copy", 10, Operation::Credit(credit));
    apply(&mut s, "close", 10, Operation::Close { reservation: "r".into() });
    let a = s.invoice("i").unwrap().audit().unwrap();
    assert_eq!(a, Audit { gross: 20, collectible: 0, reserved: 0, paid: 13, credited: 7, disputed: 0, refund_due: 0, refunded: 7, cash_received: 20 });
    assert_eq!(s.exposure("a").unwrap().minor, 13);
    rejected(&mut s, cmd("resurrect", 10, Operation::Dispute { invoice: "i".into(), amount: amount(7), release: true }), Error::Insufficient);
    rejected(&mut s, cmd("recollect", 10, Operation::Collect { id: "new".into(), invoice: "i".into(), amount: amount(7) }), Error::Insufficient);
    let mut u = billed();
    apply(&mut u, "dispute", 10, Operation::Dispute { invoice: "i".into(), amount: amount(5), release: false });
    apply(&mut u, "credit", 10, Operation::Credit(Credit { id: "u".into(), invoice: "i".into(), amount: amount(15), from_paid: false, evidence: "unpaid-credit".into() }));
    apply(&mut u, "release", 10, Operation::Dispute { invoice: "i".into(), amount: amount(5), release: true });
    assert_eq!(u.invoice("i").unwrap().collectible(), Ok(5));
    rejected(&mut u, cmd("refund-unpaid", 10, Operation::Refund { credit: "u".into(), evidence: "not-cash".into() }), Error::Transition);
}
#[test] fn failed_commands_rollback_evidence_and_overflow() {
    let mut s = paid();
    let mut c = Credit { id: "c".into(), invoice: "i".into(), amount: amount(21), from_paid: true, evidence: "reusable-after-rejection".into() };
    rejected(&mut s, cmd("too-much", 11, Operation::Credit(c.clone())), Error::Insufficient);
    c.amount = amount(20);
    apply(&mut s, "valid", 10, Operation::Credit(c)); // failed command did not advance clock or consume evidence
    let mut large = offer(); large.id = "huge".into(); large.version = 2; large.rate.price = amount(u128::MAX);
    apply(&mut s, "huge", 10, Operation::Offer(large));
    let mut r = reservation("overflow"); r.offer = "huge".into(); r.version = 2; r.start = 10; r.end = 12; r.maximum = amount(u128::MAX);
    rejected(&mut s, cmd("overflow", 10, Operation::Reserve(r)), Error::Overflow);
}

#[derive(Clone)] struct Memory { bytes: Rc<RefCell<Vec<u8>>>, failure: Rc<RefCell<Option<usize>>> }
impl Memory { fn new() -> Self { Self { bytes: Rc::default(), failure: Rc::default() } } }
impl AppendSync for Memory {
    fn append_sync(&mut self, b: &[u8]) -> std::io::Result<()> {
        if let Some(n) = *self.failure.borrow() {
            self.bytes.borrow_mut().extend_from_slice(&b[..n.min(b.len())]);
            return Err(std::io::Error::other("injected ambiguous append/sync"));
        }
        self.bytes.borrow_mut().extend_from_slice(b); Ok(())
    }
}
#[test] fn durable_restart_replay_and_changed_duplicate() {
    let sink = Memory::new(); let mut journal = Journal::recover(&[], sink.clone()).unwrap();
    for c in bootstrap() { journal.apply(&c).unwrap(); }
    journal.apply(&cmd("accept", 10, Operation::Accept(acceptance("x", 10)))).unwrap();
    journal.apply(&cmd("collect", 10, collect())).unwrap(); journal.apply(&cmd("submit", 10, submit())).unwrap();
    let bytes = sink.bytes.borrow().clone();
    let mut restarted = Journal::recover(&bytes, sink.clone()).unwrap();
    assert_eq!(journal.state(), restarted.state());
    assert_eq!(restarted.apply(&cmd("submit", 10, submit())), Ok(Applied::Replay));
    assert_eq!(sink.bytes.borrow().len(), bytes.len());
    assert_eq!(restarted.apply(&cmd("submit", 11, submit())), Err(Error::DuplicateMismatch));
    assert_eq!(restarted.apply(&cmd("fresh-submit", 10, submit())), Err(Error::Transition));
    restarted.apply(&cmd("resolve", 10, reconcile(true))).unwrap();
    assert_eq!(replay(&sink.bytes.borrow()).unwrap(), *restarted.state());
}
#[test] fn ambiguous_partial_write_and_lost_sync_poison_without_retry() {
    for written in [0, 8, usize::MAX] {
        let sink = Memory::new(); let mut journal = Journal::recover(&[], sink.clone()).unwrap();
        *sink.failure.borrow_mut() = Some(written);
        let c = bootstrap().remove(0);
        assert_eq!(journal.apply(&c), Err(Error::Io));
        assert_eq!(journal.state().command_count(), 0);
        let len = sink.bytes.borrow().len();
        assert_eq!(journal.apply(&c), Err(Error::Poisoned));
        assert_eq!(sink.bytes.borrow().len(), len);
        if written == usize::MAX { assert_eq!(replay(&sink.bytes.borrow()).unwrap().command_count(), 1); }
        else if written > 0 { assert!(replay(&sink.bytes.borrow()).is_err()); }
    }
}
#[test] fn strict_framing_corruption_truncation_and_bounds() {
    let sink = Memory::new(); let mut j = Journal::recover(&[], sink.clone()).unwrap();
    for c in bootstrap() { j.apply(&c).unwrap(); }
    let bytes = sink.bytes.borrow().clone();
    // Every non-boundary truncation of a single record fails. Complete-prefix loss requires an external manifest.
    let first = { let s = Memory::new(); let mut j = Journal::recover(&[], s.clone()).unwrap(); j.apply(&bootstrap()[0]).unwrap(); let b = s.bytes.borrow().clone(); b };
    for end in 1..first.len() { assert!(replay(&first[..end]).is_err(), "cut {end}"); }
    for index in 0..bytes.len() { let mut bad = bytes.clone(); bad[index] ^= 1; assert!(replay(&bad).is_err(), "byte {index}"); }
    let mut duplicate = bytes.clone(); duplicate.extend(&first); assert!(replay(&duplicate).is_err());
    assert_eq!(replay(&vec![0; MAX_LOG + 1]), Err(Error::Limit));
    for text in ["C2\tx\t0\tepoch\tp\t1", "C1\tx\t00\tepoch\tp\t1", "C1\tx\t0\tunknown", "C1\tx\t0\tepoch\tp\t1\textra", "C1\tx\t0\tepoch\tp\t+1", "C1\tx\t0\tepoch\tp\t1\r", "C1\tx\t0\tepoch\tp\t", "C1\tx\t0\tepoch\tp\t18446744073709551616"] { assert!(commerce_wire::decode(text).is_err(), "{text}"); }
    assert!(commerce_wire::decode(&"x".repeat(commerce_wire::MAX_RECORD + 1)).is_err());
    for c in bootstrap() { assert_eq!(commerce_wire::decode(&commerce_wire::encode(&c).unwrap()), Ok(c)); }
}
#[test] fn file_store_excludes_writers_restarts_and_retains_stale_lock() {
    use std::sync::atomic::{AtomicU64, Ordering};
    static SERIAL: AtomicU64 = AtomicU64::new(0);
    let dir = std::env::temp_dir().join(format!("apocryph-commerce-{}-{}", std::process::id(), SERIAL.fetch_add(1, Ordering::Relaxed)));
    std::fs::create_dir(&dir).unwrap(); let path = dir.join("journal");
    let mut s = FileStore::open(&path).unwrap();
    assert!(matches!(FileStore::open(&path), Err(Error::Locked)));
    for c in bootstrap() { s.apply(&c).unwrap(); }
    let expected = s.state().clone(); s.close().unwrap();
    let s = FileStore::open(&path).unwrap(); assert_eq!(*s.state(), expected); drop(s);
    assert!(matches!(FileStore::open(&path), Err(Error::Locked)));
    // Fixture-only offline reconciliation: all handles above are dropped; no external effects exist.
    std::fs::remove_dir(dir.join("journal.lock")).unwrap();
    let mut bytes = std::fs::read(&path).unwrap(); bytes.pop(); std::fs::write(&path, bytes).unwrap();
    assert!(FileStore::open(&path).is_err()); assert!(dir.join("journal.lock").exists());
    std::fs::remove_dir_all(dir).unwrap();
}

#[test] fn replay_revalidates_semantics_even_with_valid_framing() {
    fn framed(sequence: usize, c: &Command) -> Vec<u8> {
        let text = commerce_wire::encode(c).unwrap();
        let mut checksum = 0xcbf29ce484222325u64;
        for b in text.bytes() { checksum ^= b as u64; checksum = checksum.wrapping_mul(0x100000001b3); }
        format!("J1\t{}\t{}\t{:016x}\n{}\n", sequence, text.len(), checksum, text).into_bytes()
    }
    let sink = Memory::new(); let mut j = Journal::recover(&[], sink.clone()).unwrap();
    for c in bootstrap() { j.apply(&c).unwrap(); }
    let base = sink.bytes.borrow().clone();
    let mut bad = reservation("invalid"); bad.quantity = 3; bad.maximum = amount(60);
    let mut bytes = base.clone(); bytes.extend(framed(6, &cmd("oversubscribe", 0, Operation::Reserve(bad))));
    assert_eq!(replay(&bytes), Err(Error::Insufficient));
    let mut changed = bootstrap()[0].clone(); changed.now = 1;
    let mut bytes = base.clone(); bytes.extend(framed(6, &changed));
    assert_eq!(replay(&bytes), Err(Error::DuplicateMismatch));
    let mut bytes = base; bytes.extend(framed(6, &bootstrap()[0]));
    assert_eq!(replay(&bytes), Err(Error::Corrupt));
}

#[test] fn every_operation_has_one_canonical_wire_spelling() {
    let mut commands = bootstrap();
    let operations = [
        Operation::Close { reservation: "r".into() }, Operation::Expire { reservation: "r".into() },
        Operation::Accept(acceptance("x", 3)), collect(), submit(), reconcile(true), reconcile(false),
        Operation::Credit(Credit { id: "c".into(), invoice: "i".into(), amount: amount(1), from_paid: true, evidence: "e".into() }),
        Operation::Credit(Credit { id: "u".into(), invoice: "i".into(), amount: amount(1), from_paid: false, evidence: "u".into() }),
        Operation::Refund { credit: "c".into(), evidence: "refund".into() },
        Operation::Dispute { invoice: "i".into(), amount: amount(1), release: false },
        Operation::Dispute { invoice: "i".into(), amount: amount(1), release: true },
    ];
    for op in operations { commands.push(cmd("wire", 10, op)); }
    for c in commands {
        let text = commerce_wire::encode(&c).unwrap();
        assert_eq!(commerce_wire::decode(&text), Ok(c));
        assert!(commerce_wire::decode(&(text.clone() + "\textra")).is_err());
        assert!(commerce_wire::decode(&(text + "\r")).is_err());
    }
}
