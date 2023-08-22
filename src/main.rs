use anyhow::anyhow;
use std::process::Command;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let lxd_bridges = get_lxd_network_bridges()?;

    log::info!(
        "These LXD networks will be added to iptables DOCKER-USER chain: {:?}",
        lxd_bridges
            .iter()
            .map(|x| x.name.as_str())
            .collect::<Vec<_>>()
    );
    register_to_iptables_docker_user_chain(&lxd_bridges)?;

    log::info!("Done.");
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
struct LxdNetwork {
    name: String,
    #[serde(rename = "type")]
    network_type: String,
    managed: bool,
}

type LxdNetworks = Vec<LxdNetwork>;

fn get_lxd_network_bridges() -> anyhow::Result<LxdNetworks> {
    let output = Command::new("lxc")
        .args(&["network", "list", "--format", "json"])
        .output()?;

    let networks: LxdNetworks = serde_json::from_slice::<LxdNetworks>(&output.stdout)?;
    let bridges: LxdNetworks = networks
        .into_iter()
        .filter(|n| n.managed && n.network_type == "bridge")
        .collect();

    Ok(bridges)
}

fn register_to_iptables_docker_user_chain(lxd_bridges: &LxdNetworks) -> anyhow::Result<()> {
    let ipt = iptables::new(false).map_err(|e| anyhow!("Failed to prepare iptables: {e}"))?;
    for lxd_bridge in lxd_bridges {
        let lxd_bridge_name = &lxd_bridge.name;
        let rules = [
            ("egress", format!("-i {} -j ACCEPT", lxd_bridge_name)),
            (
                "ingress",
                format!(
                    "-o {} -m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT",
                    lxd_bridge_name
                ),
            ),
        ];

        for (kind, rule) in &rules {
            if !ipt.exists("filter", "DOCKER-USER", &rule).map_err(|e| {
                anyhow!(
                    "Failed to check iptables whether {lxd_bridge_name} {kind} rule exists: {e}"
                )
            })? {
                ipt.insert("filter", "DOCKER-USER", &rule, 1)
                    .map_err(|e| anyhow!("Failed to add {kind} rule to iptables: {e}"))?;
            }
        }
    }

    Ok(())
}
