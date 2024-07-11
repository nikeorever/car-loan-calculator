use std::fmt::Display;

struct CarLoan {
    money: f64,
    year: u32,
    interest_rate_per_year: f64,
}

impl Display for CarLoan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "money(贷款总金额) = {}, year(贷款年数) = {}, interest_rate_per_year(年利率`) = {}",
            self.money, self.year, self.interest_rate_per_year
        )
    }
}

impl CarLoan {
    fn new(money: f64, year: u32, interest_rate_per_year: f64) -> CarLoan {
        Self {
            money,
            year,
            interest_rate_per_year,
        }
    }

    /// 贷款总期数，也就是贷多少个月
    fn month(&self) -> u32 {
        self.year * 12
    }

    /// 本金， 也就是每月除了利息外需要还的钱数
    fn principal_per_month(&self) -> f64 {
        self.money / (self.month() as f64)
    }

    /// 每月的利息
    fn interest_per_month(&self) -> f64 {
        self.interest_per_year() / 12f64
    }

    /// 每年的利息
    fn interest_per_year(&self) -> f64 {
        self.money * self.interest_rate_per_year
    }

    /// 总利息
    fn total_interest(&self) -> f64 {
        self.interest_per_year() * (self.year as f64)
    }

    /// 月供
    fn total_money_per_month(&self) -> f64 {
        self.principal_per_month() + self.interest_per_month()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let car_loan = CarLoan::new(100000f64, 3, 0.023);
        println!("贷款总期数，也就是贷多少个月={}", car_loan.month());
        println!("本金， 也就是每月除了利息外需要还的钱数={}", car_loan.principal_per_month());
        println!("每月的利息={}", car_loan.interest_per_month());
        println!("每年的利息={}", car_loan.interest_per_year());
        println!("总利息={}", car_loan.total_interest());
        println!("月供={}", car_loan.total_money_per_month());
    }
}
