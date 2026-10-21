//! Offline generic TSV fixture driver. No external submission adapter exists.
use std::io::{self, BufRead, Write};
use apocryph_n::{commerce::*, commerce_store::FileStore, commerce_wire::{self, MAX_RECORD}};
fn summaries(state: &Commerce, out: &mut impl Write) -> io::Result<()> {
    for i in state.invoices() {
        let a = i.audit().map_err(|_| io::Error::other("invalid audit"))?;
        writeln!(out, "S1\tinvoice\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            i.id, i.reservation, i.offer.rate.price.asset.namespace, i.offer.rate.price.asset.network,
            i.offer.rate.price.asset.identifier, i.offer.rate.price.asset.scale, i.offer.rate.unit.0,
            a.gross, a.collectible, a.reserved, a.paid, a.credited, a.disputed, a.refund_due, a.refunded, a.cash_received,
            i.cumulative, i.offer.id)?;
    }
    for c in state.credits() { writeln!(out, "S1\tcredit\t{}\t{}\t{}\t{}\t{}", c.id, c.invoice, c.amount.minor, if c.from_paid { "paid" } else { "unpaid" }, c.evidence)?; }
    for t in state.intents() { writeln!(out, "S1\toutbox\t{}\t{}\t{}\t{}\t{}", t.id, t.invoice, t.amount.minor,
        match t.state { CollectionState::Ready => "ready", CollectionState::Unknown => "unknown", CollectionState::Paid => "paid", CollectionState::Failed => "failed" }, t.evidence.as_deref().unwrap_or("none"))?; }
    Ok(())
}
fn run() -> std::result::Result<(), String> {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 2 { return Err("usage: commerce JOURNAL < commands.tsv".into()); }
    let mut store = FileStore::open(&args[1]).map_err(|e| format!("open: {e:?}; retain lock and reconcile offline"))?;
    let input = io::stdin(); let mut input = input.lock(); let mut line = Vec::new();
    let mut output = io::BufWriter::new(io::stdout().lock());
    // Read one byte at a time through a bounded buffer; read_line could allocate unbounded input.
    let result = (|| -> std::result::Result<(), String> {
        loop {
            let buf = input.fill_buf().map_err(|e| e.to_string())?;
            if buf.is_empty() {
                if !line.is_empty() { return Err("unterminated input line".into()); }
                break;
            }
            let b = buf[0]; input.consume(1);
            if b != b'\n' {
                if line.len() == MAX_RECORD { return Err("oversize input line".into()); }
                line.push(b); continue;
            }
            let text = std::str::from_utf8(&line).map_err(|_| "non-UTF8 input")?;
            let c = commerce_wire::decode(text).map_err(|e| format!("parse: {e:?}"))?;
            let applied = store.apply(&c).map_err(|e| format!("command {}: {e:?}", c.id))?;
            writeln!(output, "S1\tcommand\t{}\t{}", c.id, if applied == Applied::New { "new" } else { "replay" }).map_err(|e| e.to_string())?;
            line.clear();
        }
        summaries(store.state(), &mut output).map_err(|e| e.to_string())?;
        output.flush().map_err(|e| e.to_string())
    })();
    if store.is_poisoned() { return Err(format!("{}; writer poisoned: reconcile journal and retained lock offline", result.err().unwrap_or_default())); }
    // Parsing/output failures do not undo already synchronized commands. Reuse their exact IDs.
    store.close().map_err(|e| format!("close: {e:?}; reconcile offline"))?;
    result
}
fn main() {
    if let Err(e) = run() { eprintln!("commerce: {e}"); std::process::exit(1); }
}
