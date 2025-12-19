fn main() {
    let mut _owner1: i32 = 82;

    let _owner2: &mut i32 = &mut _owner1;

    *_owner2 += 2;

    println!("value of owner1 is {}", _owner1); // 84
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
