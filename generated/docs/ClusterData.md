# ClusterData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authentication_parameters** | Option<**String**> | Authentication parameters when client would like to connect to cluster. | [optional]
**authentication_plugin** | Option<**String**> | Authentication plugin when client would like to connect to cluster. | [optional]
**broker_client_certificate_file_path** | Option<**String**> | TLS certificate file for internal client, used by the internal client to authenticate with Pulsar brokers | [optional]
**broker_client_key_file_path** | Option<**String**> | TLS private key file for internal client, used by the internal client to authenticate with Pulsar brokers | [optional]
**broker_client_tls_enabled** | Option<**bool**> | Enable TLS when talking with other brokers in the same cluster (admin operation) or different clusters (replication) | [optional]
**broker_client_tls_enabled_with_key_store** | Option<**bool**> | Whether internal client use KeyStore type to authenticate with other Pulsar brokers | [optional]
**broker_client_tls_key_store** | Option<**String**> | TLS KeyStore path for internal client,  used by the internal client to authenticate with Pulsar brokers | [optional]
**broker_client_tls_key_store_password** | Option<**String**> | TLS KeyStore password for internal client,  used by the internal client to authenticate with Pulsar brokers | [optional]
**broker_client_tls_key_store_type** | Option<**String**> | TLS KeyStore type configuration for internal client: JKS, PKCS12, used by the internal client to authenticate with Pulsar brokers | [optional]
**broker_client_tls_trust_store** | Option<**String**> | TLS TrustStore path for internal client used by the internal client to authenticate with Pulsar brokers | [optional]
**broker_client_tls_trust_store_password** | Option<**String**> | TLS TrustStore password for internal client used by the internal client to authenticate with Pulsar brokers | [optional]
**broker_client_tls_trust_store_type** | Option<**String**> | TLS TrustStore type configuration for internal client: JKS, PKCS12 used by the internal client to authenticate with Pulsar brokers | [optional]
**broker_client_trust_certs_file_path** | Option<**String**> | Path for the trusted TLS certificate file for outgoing connection to a server (broker) | [optional]
**broker_service_url** | Option<**String**> | The broker service url (for produce and consume operations) | [optional]
**broker_service_url_tls** | Option<**String**> | The secured broker service url (for produce and consume operations) | [optional]
**listener_name** | Option<**String**> | listenerName when client would like to connect to cluster | [optional]
**peer_cluster_names** | Option<**Vec<String>**> | A set of peer cluster names | [optional]
**proxy_protocol** | Option<**String**> | protocol to decide type of proxy routing eg: SNI-routing | [optional]
**proxy_service_url** | Option<**String**> | Proxy-service url when client would like to connect to broker via proxy. | [optional]
**service_url** | Option<**String**> | The HTTP rest service URL (for admin operations) | [optional]
**service_url_tls** | Option<**String**> | The HTTPS rest service URL (for admin operations) | [optional]
**tls_allow_insecure_connection** | Option<**bool**> | Allow TLS connections to servers whose certificate cannot be be verified to have been signed by a trusted certificate authority. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


