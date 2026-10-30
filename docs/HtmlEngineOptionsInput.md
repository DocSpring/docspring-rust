# HtmlEngineOptionsInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**format** | Option<[**models::PageFormat**](PageFormat.md)> | The paper size. `Custom` uses `width` and `height`. | [optional]
**width** | Option<**String**> | The page width for the `Custom` format, with a unit (`8.5in`, `210mm`). | [optional]
**height** | Option<**String**> | The page height for the `Custom` format, with a unit. | [optional]
**scale** | Option<**f64**> | How much to scale the page, such as `0.8`. `1` prints it at its normal size. | [optional]
**landscape** | Option<**bool**> | Print pages in landscape orientation. | [optional]
**print_background** | Option<**bool**> | Print background colors and images. | [optional]
**margin_top** | Option<**String**> | The top margin, with a unit, such as `1in` or `20mm`. | [optional]
**margin_bottom** | Option<**String**> | The bottom margin, with a unit. | [optional]
**margin_left** | Option<**String**> | The left margin, with a unit. | [optional]
**margin_right** | Option<**String**> | The right margin, with a unit. | [optional]

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


