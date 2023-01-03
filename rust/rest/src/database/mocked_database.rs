use crate::models::payment::Payment;

#[derive(Debug, Clone)]
pub struct Database {
    payments: Vec<Payment>,
}

impl Database {
    pub fn new() -> Self {
        let payments = vec![
            Payment {
                id: 1,
                amount: 100,
                currency: "USD".to_string(),
                status: "pending".to_string(),
            },
            Payment {
                id: 2,
                amount: 200,
                currency: "USD".to_string(),
                status: "pending".to_string(),
            },
            Payment {
                id: 3,
                amount: 300,
                currency: "USD".to_string(),
                status: "pending".to_string(),
            },
        ];
        Self { payments: payments }
    }

    pub fn get_payments(&self) -> Vec<Payment> {
        self.payments.clone().to_vec()
    }

    pub fn get_payment(&self, id: u32) -> Option<Payment> {
        self.payments.iter().find(|p| p.id == id).cloned()
    }

    pub fn create_payment(&mut self, payment: Payment) -> Payment {
        let id = self.payments.len() as u32 + 1;
        let payment = Payment { id: id, ..payment };
        self.payments.push(payment.clone());
        payment
    }

    pub fn update_payment(&mut self, payment: Payment) -> Option<Payment> {
        let index = self.payments.iter().position(|p| p.id == payment.id)?;
        self.payments[index] = payment.clone();
        Some(payment)
    }
}
