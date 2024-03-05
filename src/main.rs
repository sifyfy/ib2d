use anyhow::anyhow;
use std::process::Command;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let incus_bridges = get_incus_network_bridges()?;

    log::info!(
        "These incus networks will be added to iptables DOCKER-USER chain: {:?}",
        incus_bridges
            .iter()
            .map(|x| x.name.as_str())
            .collect::<Vec<_>>()
    );
    register_to_iptables_docker_user_chain(&incus_bridges)?;

    log::info!("Done.");
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
struct IncusNetwork {
    name: String,
    #[serde(rename = "type")]
    network_type: String,
    managed: bool,
}

type IncusNetworks = Vec<IncusNetwork>;

fn get_incus_network_bridges() -> anyhow::Result<IncusNetworks> {
    let output = Command::new("incus")
        .args(&["network", "list", "--format", "json"])
        .output()?;

    let networks: IncusNetworks = serde_json::from_slice::<IncusNetworks>(&output.stdout)?;
    let bridges: IncusNetworks = networks
        .into_iter()
        .filter(|n| n.managed && n.network_type == "bridge")
        .collect();

    Ok(bridges)
}

fn register_to_iptables_docker_user_chain(incus_bridges: &IncusNetworks) -> anyhow::Result<()> {
    let ipt = iptables::new(false).map_err(|e| anyhow!("Failed to prepare iptables: {e}"))?;
    for incus_bridge in incus_bridges {
        let incus_bridge_name = &incus_bridge.name;
        let rules = [
            ("egress", format!("-i {} -j ACCEPT", incus_bridge_name)),
            (
                "ingress",
                format!(
                    "-o {} -m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT",
                    incus_bridge_name
                ),
            ),
        ];

        for (kind, rule) in &rules {
            if !ipt.exists("filter", "DOCKER-USER", &rule).map_err(|e| {
                anyhow!(
                    "Failed to check iptables whether {incus_bridge_name} {kind} rule exists: {e}"
                )
            })? {
                ipt.insert("filter", "DOCKER-USER", &rule, 1)
                    .map_err(|e| anyhow!("Failed to add {kind} rule to iptables: {e}"))?;
            }
        }
    }

    Ok(())
}
