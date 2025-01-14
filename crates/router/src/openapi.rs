#[derive(utoipa::OpenApi)]
#[openapi(
    info(
        title = "Juspay Router - API Documentation",
        contact(
            name = "Juspay Support",
            url = "https://juspay.io",
            email = "support@juspay.in"
        ),
        // terms_of_service = "https://www.juspay.io/terms",
        description = r#"
## Get started

Juspay Router provides a collection of APIs that enable you to process and manage payments.
Our APIs accept and return JSON in the HTTP body, and return standard HTTP response codes.

You can consume the APIs directly using your favorite HTTP/REST library.

We have a testing environment referred to "sandbox", which you can setup to test API calls without
affecting production data.

### Base URLs

Use the following base URLs when making requests to the APIs:

| Environment   |  Base URL                                            |
|---------------|------------------------------------------------------|
| Sandbox       | <https://sandbox-router.juspay.io>                   |
| Production    | <https://router.juspay.io>                           |

## Authentication

When you sign up on our [dashboard](https://dashboard-hyperswitch.netlify.app) and create a merchant
account, you are given a secret key (also referred as api-key).
You may authenticate all API requests with Juspay server by providing the appropriate key in the
request Authorization header.

Never share your secret api keys. Keep them guarded and secure.
"#,
    ),
    servers(
        (url = "https://sandbox-router.juspay.io", description = "Sandbox Environment"),
        (url = "https://router.juspay.io", description = "Production Environment")
    ),
    paths(
        crate::routes::refunds::refunds_create,
        crate::routes::admin::merchant_account_create,
        crate::routes::payments::payments_create
    ),
    components(schemas(
        crate::types::api::refunds::RefundRequest,
        crate::types::api::refunds::RefundType,
        crate::types::api::refunds::RefundResponse,
        crate::types::api::refunds::RefundStatus,
        crate::types::api::admin::CreateMerchantAccount,
        api_models::enums::RoutingAlgorithm,
        api_models::enums::PaymentMethodType,
        api_models::enums::PaymentMethodSubType,
        api_models::enums::ConnectorType,
        api_models::enums::Currency,
        api_models::enums::IntentStatus,
        api_models::enums::CaptureMethod,
        api_models::enums::FutureUsage,
        api_models::enums::AuthenticationType,
        api_models::enums::WalletIssuer,
        api_models::enums::Connector,
        api_models::enums::PaymentMethodType,
        api_models::admin::PaymentConnectorCreate,
        api_models::admin::PaymentMethods,
        api_models::payments::AddressDetails,
        api_models::payments::Address,
        api_models::payments::OrderDetails,
        api_models::payments::NextActionType,
        api_models::payments::Metadata,
        api_models::payments::WalletData,
        api_models::payments::KlarnaRedirectIssuer,
        api_models::payments::KlarnaSdkIssuer,
        api_models::payments::NextAction,
        api_models::payments::PayLaterData,
        api_models::payments::MandateData,
        api_models::payments::PhoneDetails,
        api_models::payments::PaymentMethod,
        api_models::payments::MandateType,
        api_models::payments::AcceptanceType,
        api_models::payments::MandateAmountData,
        api_models::payments::OnlineMandate,
        api_models::payments::CCard,
        api_models::payments::CustomerAcceptance,
        api_models::payments::PaymentsRequest,
        api_models::payments::PaymentsResponse,
        api_models::payment_methods::PaymentExperience,
        crate::types::api::admin::MerchantAccountResponse,
        crate::types::api::admin::MerchantConnectorId,
        crate::types::api::admin::MerchantDetails,
        crate::types::api::admin::WebhookDetails,
    ))
)]
pub struct ApiDoc;
