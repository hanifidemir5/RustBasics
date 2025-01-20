pub fn traits_and_generics_homework(){
    let mut bank_account1:BankAccount= BankAccount{
        account_number : 186267,
        holder_name : "James Bond".to_string(),
        balance : 7000000.0,
    };

    let mut bank_account2:BankAccount= BankAccount{
        account_number : 187667,
        holder_name : "Bruce Wayne".to_string(),
        balance : 2147483647.0,
    };

    bank_account1.deposit(777777.0);
    bank_account2.withdraw(147483647.0);
    let bank_account1_balance = bank_account1.balance();
    let bank_account2_balance = bank_account2.balance();
    println!("Balance for {} is {:?}", bank_account1.holder_name, bank_account1_balance);
    println!("Balance for {} is {:?}", bank_account2.holder_name, bank_account2_balance);
}

trait Account {
    fn deposit(&mut self,deposit_amount:f32);
    fn withdraw(&mut self,withdraw_amount:f32);
    fn balance(&mut self) -> f32;
}

struct BankAccount{
    account_number:i32,
    holder_name:String,
    balance:f32,
}

impl Account for  BankAccount{
    fn deposit(&mut self, deposit_amount:f32) {
        self.balance = self.balance + deposit_amount;
    }
    fn withdraw(&mut self,withdraw_amount:f32) {
        self.balance = self.balance - withdraw_amount;
    }
    fn balance(&mut self) -> f32 {
        self.balance
    }
}