# CardCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**card_id** | **String** |  | 
**char_id** | **String** | Foreign key to the character table | 
**rarity** | **String** |  | 
**rarity_modifier** | Option<**String**> | 6-1, 6-2 (6S) | [optional]
**name** | **String** |  | 
**name_normalized** | **String** | NFKD normalized with special characters removed | 
**jp_name** | Option<**String**> |  | [optional]
**jp_name_normalized** | Option<**String**> |  | [optional]
**link_name** | **String** |  | 
**link_name_normalized** | **String** |  | 
**card_type** | [**models::CardType**](CardType.md) |  | 
**main_color** | **String** |  | 
**side_color** | Option<**String**> |  | [optional]
**wiki_template** | Option<[**models::CardTemplateData**](CardTemplateData.md)> |  | [optional]
**updated_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


