# HtmlEngineOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**format** | [**models::PageFormat**](PageFormat.md) | The paper size. `Custom` uses `width` and `height`. | 
**width** | Option<**String**> | The page width for the `Custom` format, with a unit (`8.5in`, `210mm`). | [optional]
**height** | Option<**String**> | The page height for the `Custom` format, with a unit. | [optional]
**scale** | **f64** | How much to scale the page, from 0.1 to 2, such as `0.8`. `1` prints it at its normal size. | 
**landscape** | **bool** | Print pages in landscape orientation. | 
**print_background** | **bool** | Print background colors and images. | 
**margin_top** | **String** | The top margin, with a unit, such as `1in` or `20mm`. | 
**margin_bottom** | **String** | The bottom margin, with a unit. | 
**margin_left** | **String** | The left margin, with a unit. | 
**margin_right** | **String** | The right margin, with a unit. | 

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


