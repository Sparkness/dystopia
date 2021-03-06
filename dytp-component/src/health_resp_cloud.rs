use crate::node::Node;
use semver::Version;
use serde_derive::Serialize;

// TODO:
// May be it's too heavy to return every nodes every times.
// We need pagination or other logics to limit number of return nodes.
#[derive(Debug, Serialize)]
pub struct HealthRespCloud {
    version: Version,
    nodes: Vec<Node>,
}

impl HealthRespCloud {
    pub fn new(version: &str, nodes: &[Node]) -> HealthRespCloud {
        HealthRespCloud {
            version: Version::parse(version).unwrap(),
            nodes: nodes.to_owned(),
        }
    }
}

impl Into<Vec<u8>> for HealthRespCloud {
    fn into(self) -> Vec<u8> {
        if self.nodes.len() > 0 {
            format!(
                "{} {}",
                self.version,
                self.nodes
                    .iter()
                    .map(|n| format!("{}", n))
                    .collect::<Vec<String>>()
                    .join(" ")
            )
            .into_bytes()
        } else {
            format!("{}", self.version).into_bytes()
        }
    }
}

impl From<&[u8]> for HealthRespCloud {
    fn from(n: &[u8]) -> HealthRespCloud {
        let version_nodes = std::str::from_utf8(n)
            .unwrap()
            .split(" ")
            .collect::<Vec<&str>>();

        if version_nodes.len() % 3 != 1 {
            log::error!("invalid response={:?}", version_nodes);

            panic!();
        }

        let version = version_nodes[0].parse().unwrap();
        let nodes_len = (version_nodes.len() - 1) / 3;
        let mut nodes = Vec::new();

        for idx in 0..nodes_len {
            let addr = version_nodes[idx * 3 + 1].parse().unwrap();
            let state = version_nodes[idx * 3 + 2].parse().unwrap();
            let version = version_nodes[idx * 3 + 3].parse().unwrap();

            nodes.push(Node {
                addr,
                state,
                version,
            });
        }

        HealthRespCloud { version, nodes }
    }
}
