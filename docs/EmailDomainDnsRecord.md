# EmailDomainDnsRecord

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**purpose** | [**models::EmailDomainDnsRecordPurpose**](EmailDomainDnsRecordPurpose.md) | What the record is for. `dkim` signs the emails (required), `mail_from` lets SPF pass for your domain (recommended), and `dmarc` tells inboxes what to do with emails that fail these checks (required). | 
**r#type** | [**models::DnsRecordType**](DnsRecordType.md) | The record's type. | 
**name** | **String** | The record's name. Some DNS hosts add the domain to the end of a name by themselves. | 
**value** | **String** | The record's value. | 
**additional_properties** | **serde_json::Map<String, serde_json::Value>** | Properties the schema doesn't list (it allows additional properties), in the order they came (with serde_json's `preserve_order` feature) or sorted by name. | [optional]

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


