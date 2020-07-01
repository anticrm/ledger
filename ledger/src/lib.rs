
use std::collections::HashMap;
use std::ops::AddAssign;
use rust_decimal::prelude::*;
use rust_decimal_macros::*;

type AccountId = u32;
type ResourceId = u32;
type Quantity = u32;

#[derive(Debug)]
struct Amount {
    amount: Decimal,
    qty: Quantity
}

impl Amount {
    fn new(amount: Decimal, qty: Quantity) -> Self {
        Amount { amount: amount, qty: qty }
    }

    fn inc(&mut self, other: Self) {
        self.amount += other.amount
    }

    fn is_zero(&self) -> bool {
        return self.amount.is_zero()  
    }
}

impl PartialEq for Amount {
    fn eq(&self, other: &Self) -> bool {
        return self.qty == other.qty && self.amount == other.amount
    }
}

impl AddAssign for Amount {
    fn add_assign(&mut self, other: Self) {
        self.inc(other)
    }
}

struct Account {
    parent: AccountId,
    balances: HashMap<ResourceId, Amount>,
}

struct Xline {
    account: AccountId,
    resource: ResourceId,
    amount: Amount,
}

impl Xline {
    fn new(account: AccountId, resource: ResourceId, amount: Amount) -> Self {
        Xline { account: account, resource: resource, amount: amount }
    }
}

struct Xact {
    lines: Vec<Xline>,
}

impl Xact {
    fn new(lines: Vec<Xline>) -> Self {
        Xact { lines: lines }
    }

    fn is_balanced(self) -> bool {
        let mut per_resource: HashMap<ResourceId, Amount> = HashMap::new();
        for x in self.lines {
            if per_resource.contains_key(&x.resource) {
                per_resource.get_mut(&x.resource).unwrap().inc(x.amount);
            } else {
                per_resource.insert(x.resource, x.amount);
            }
        }

        for x in per_resource.values() {
            if !x.is_zero() { return false; }
        }

        return true;
    }
}

struct Hierarchy {
    accounts: HashMap<AccountId, Account>,
}

impl Hierarchy {
    fn apply(&mut self, xact: &Xact) {

    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rust_decimal::prelude::*;
    use rust_decimal_macros::*;
    
    #[test]
    fn amount_add_assign() {
        let mut a1 = Amount { amount: dec!(5.5), qty: 55 };
        let a2 = Amount { amount: dec!(6.5), qty: 55 };
        a1 += a2;

        assert_eq!(a1.amount, dec!(12));
    }

    #[test]
    fn xact_balance() {
        let xact = Xact::new(vec![]);
        assert_eq!(xact.is_balanced(), true);

        let xact2 = Xact::new(vec![
            Xline::new(1, 1, Amount::new(dec!(15), 1))
        ]);
        assert_eq!(xact2.is_balanced(), false);

        let xact3 = Xact::new(vec![
            Xline::new(1, 1, Amount::new(dec!(15), 1)),
            Xline::new(1, 1, Amount::new(dec!(-10), 1)),
            Xline::new(1, 1, Amount::new(dec!(-5), 1)),
        ]);
        assert_eq!(xact3.is_balanced(), true);

    }
}
