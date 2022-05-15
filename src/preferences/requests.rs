use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::common_types::{BackUrls, CheckoutProPayer, Item, PaymentMethods};

/// Buyers will be redirected back to your site immediately after completing
/// the purchase.
///
/// By default, `AutoReturn` is not specified.
///
/// Possible values are:
/// * approved: The redirection takes place only for approved payments.
///
/// * all: The redirection takes place only for approved payments, forward compatibility only if we
///   change the default behavior
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Debug, strum::AsRefStr)]
pub enum AutoReturn {
    /// The redirection takes place only for approved payments.
    #[strum(serialize = "approved")]
    Approved,

    /// The redirection takes place only for approved payments, forward compatibility only if we
    /// change the default behavior
    #[strum(serialize = "all")]
    All,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckoutProPreferences {
    pub(crate) items: Vec<Item>,
    pub(crate) payer: CheckoutProPayer,

    /// Also known as Cust ID, or simply User ID. It its the ID of the seller's MercadoPago
    /// account.
    pub(crate) collector_id: i64,

    /// If specified, your buyers will be redirected back to your site immediately after completing
    /// the purchase.
    ///
    /// Possible values are:
    /// * approved: The redirection takes place only for approved payments.
    ///
    /// * all: The redirection takes place only for approved payments, forward compatibility only
    ///   if we change the default behavior
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) auto_return: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) back_urls: Option<BackUrls>,

    /// Describes Checkout Pro's payment methods and attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<PaymentMethods>,

    /// Sets up an IPN(instant payment notification) URL to notify when payment updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_url: Option<String>,

    /// Depending on the card brand, the description (attribute value) will appear on the buyer's
    /// card invoice.
    pub statement_descriptor: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reference: Option<String>,

    /// Useful for limited-time promotions, or to setup any kind of limited-time logic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) expires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) expiration_date_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) expiration_date_to: Option<String>,
}

impl CheckoutProPreferences {}

impl Default for CheckoutProPreferences {
    fn default() -> Self {
        Self {
            items: vec![],
            payer: Default::default(),
            back_urls: Default::default(),
            statement_descriptor: "".to_string(),
            auto_return: None,
            payment_methods: None,
            notification_url: None,
            external_reference: None,
            expires: None,
            expiration_date_from: None,
            expiration_date_to: None,
            collector_id: 182423046,
        }
    }
}

impl CheckoutProPreferences {
    /// Creates a blank [`CheckoutProPreferences`].
    /// You need to set at least a `Payer`, and one `Item`, otherwise it will fail to validate.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets [`AutoReturn`].
    pub fn set_auto_return(mut self, auto_return: AutoReturn) -> Self {
        self.auto_return = Some(auto_return.as_ref().to_string());
        self
    }

    /// Sets the preference to begin on `from_date` and expire at `end_date`.
    pub fn set_limited_offer(
        mut self,
        from_date: OffsetDateTime,
        end_date: OffsetDateTime,
    ) -> Self {
        self.expires = Some(true);
        self.expiration_date_from = Some(from_date.to_string());
        self.expiration_date_to = Some(end_date.to_string());

        self
    }

    pub fn set_payer(mut self, payer: CheckoutProPayer) -> Self {
        self.payer = payer;
        self
    }

    /// Sets the [`BackUrls`].
    pub fn set_backurls(
        mut self,
        success: Option<String>,
        pending: Option<String>,
        failure: Option<String>,
    ) -> Self {
        self.back_urls = Some(BackUrls {
            success,
            failure,
            pending,
        });

        self
    }

    pub fn set_items(mut self, items: Vec<Item>) -> Self {
        self.items = items;
        self
    }

    pub(crate) fn validate(&self) -> bool {
        if !self.payer.validate() {
            return false;
        }
        true
    }
}
