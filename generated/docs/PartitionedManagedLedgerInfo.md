# PartitionedManagedLedgerInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**creation_date** | Option<**String**> |  | [optional]
**cursors** | Option<[**std::collections::HashMap<String, models::CursorInfo>**](CursorInfo.md)> |  | [optional]
**ledgers** | Option<[**Vec<models::LedgerInfo>**](LedgerInfo.md)> |  | [optional]
**modification_date** | Option<**String**> |  | [optional]
**partitions** | Option<[**std::collections::HashMap<String, models::ManagedLedgerInfo>**](ManagedLedgerInfo.md)> |  | [optional]
**properties** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**terminated_position** | Option<[**models::PositionInfo**](PositionInfo.md)> |  | [optional]
**version** | Option<**i64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


