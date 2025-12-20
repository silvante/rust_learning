fn main() {
    let mut account: BankAccount = BankAccount{
        owner: "Valentine".to_string(),
        balance: 12500.05,
    };
    // immutable borrow
    account.chack_balance();

    // mutable borrow
    account.withdraw(2500.05);

    account.chack_balance();
}

struct BankAccount {
    owner: String,
    balance: f32,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f32) {
        println!("withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn chack_balance(&self) {
        println!("account owned by {}, has {} in balance", self.owner, self.balance);
    }
}
