# EmailDomain

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The email domain's ID. | 
**domain** | **String** | The domain name, such as `example.com` or `mail.example.com`. Email actions can send from any address at it. | 
**state** | [**models::EmailDomainState**](EmailDomainState.md) | `pending` until its DNS records are found, then `verified`. `failed`: its records weren't found within 3 days, or were removed. Verify it again after fixing them. | 
**dkim_state** | [**models::EmailDomainRecordState**](EmailDomainRecordState.md) | Whether the DKIM records were found. | 
**mail_from_state** | [**models::EmailDomainRecordState**](EmailDomainRecordState.md) | Whether the MAIL FROM records were found. They're optional: they let SPF pass for your domain too. | 
**dns_records** | [**Vec<models::EmailDomainDnsRecord>**](EmailDomainDnsRecord.md) | The DNS records to add to the domain. Empty while DocSpring is still setting the domain up: verify it again in a few minutes to get them. | 
**dmarc_policy** | Option<**String**> | The policy (`p=`) of the domain's DMARC record, once it has been found. | 
**last_error** | Option<**String**> | Why the domain isn't verified yet, or what to fix. | 
**verified_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | When the domain was verified. | 
**checked_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | When DocSpring last checked the domain's DNS records. | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | When the domain was added. | 
**additional_properties** | **serde_json::Map<String, serde_json::Value>** | Properties the schema doesn't list (it allows additional properties), in the order they came (with serde_json's `preserve_order` feature) or sorted by name. | [optional]

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


