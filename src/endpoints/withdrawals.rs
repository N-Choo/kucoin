use crate::{
    client::rest::KuCoinClient,
    types::{
        KuCoinResponse,
        withdraw::{WithdrawRequest, WithdrawResponse, WithdrawType},
    },
    utils::errors::KucoinResults,
};

pub struct WithdrawHandler<'a> {
    pub client: &'a KuCoinClient,
}

impl WithdrawRequest {
    /// Creates a new withdrawal request.
    pub fn new(currency: &str, to_address: &str, amount: f64, withdraw_type: WithdrawType) -> Self {
        WithdrawRequest {
            amount: amount.to_string(),
            chain: None,
            currency: currency.to_string(),
            fee_deduct_type: None,
            is_inner: None,
            memo: None,
            remark: None,
            to_address: to_address.to_string(),
            withdraw_type,
        }
    }

    /// Sets the chain name (e.g., "ERC20").
    pub fn set_chain(mut self, chain: &str) -> Self {
        self.chain = Some(chain.to_string());
        self
    }

    /// Sets the address memo.
    pub fn set_memo(mut self, memo: &str) -> Self {
        self.memo = Some(memo.to_string());
        self
    }

    /// Marks the withdrawal as internal.
    pub fn set_isinner(mut self) -> Self {
        self.is_inner = Some(true);
        self
    }

    /// Sets a remark for the transaction.
    pub fn set_remark(mut self, rm: &str) -> Self {
        self.remark = Some(rm.to_string());
        self
    }

    /// Withdrawal fee deduction type: INTERNAL, EXTERNAL, or not specified
    /// INTERNAL: Deduct the transaction fees from your withdrawal amount
    /// EXTERNAL: Deduct the transaction fees from your main account
    ///
    /// If you don't specify the feeDeductType parameter, when the balance in
    /// your main account is sufficient to support the withdrawal,
    /// the system will initially deduct the transaction fees from your main account.
    /// But if the balance in your main account is not sufficient to support the withdrawal,
    /// the system will deduct the fees from your withdrawal amount.
    pub fn set_fee_deduct_type(mut self, type_: &str) -> Self {
        self.fee_deduct_type = Some(type_.to_string());
        self
    }
}

impl<'a> WithdrawHandler<'a> {
    /// Executes the withdrawal request.
    pub async fn execute(
        &self,
        req: WithdrawRequest,
    ) -> KucoinResults<KuCoinResponse<WithdrawResponse>> {
        let payload = serde_json::to_string(&req)?;
        let endpoint = "/api/v3/withdrawals";

        let res = self
            .client
            .send::<KuCoinResponse<WithdrawResponse>>("POST", &payload, endpoint)
            .await?;

        Ok(res)
    }
}
