fn main() {}

#[cfg(test)]
mod tests {
    use akri_shared::akri::configuration::KubeAkriConfig;
    use chrono::prelude::*;
    use k8s_openapi::apimachinery::pkg::apis::meta::v1::Time;

    const JSON: &str = r#"{
        "apiVersion": "akri.sh/v0",
        "kind": "Configuration",
        "metadata": {
            "annotations": {
                "kubectl.kubernetes.io/last-applied-configuration": ""
            },
            "creationTimestamp": "2021-01-01T00:00:00Z",
            "deletionTimestamp": "2021-01-01T00:00:00Z",
            "generation": 1,
            "managedFields": [],
            "name": "name",
            "namespace": "default",
            "uid": "00000000-0000-0000-0000-000000000000"
        },
        "spec": {
            "brokerPodSpec": {
                "containers": []
            },
            "capacity": 1,
            "protocol": {
                "debugEcho": {
                    "descriptions": ["foo","bar"],
                    "shared": true
                }
            }
        }
    }"#;

    #[test]
    fn test_creation() {
        let c: KubeAkriConfig = serde_json::from_str(JSON).expect("Valid Configuration");
        let lhs = &c.metadata.creation_timestamp.expect("Valid Time");

        let rhs = &Some(Time(
            Utc.from_utc_datetime(
                &DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z")
                    .unwrap()
                    .naive_utc(),
            ),
        ))
        .expect("Valid Time");

        assert_eq!(lhs, rhs);
    }
    #[test]
    fn test_deletion() {
        let c: KubeAkriConfig = serde_json::from_str(JSON).expect("Valid Configuration");
        let lhs = &c.metadata.deletion_timestamp.expect("Valid Time");

        let rhs = &Some(Time(
            Utc.from_utc_datetime(
                &DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z")
                    .unwrap()
                    .naive_utc(),
            ),
        ))
        .expect("Valid Time");

        assert_eq!(lhs, rhs);
    }
}
