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
    for lxd_bridge in lxd_bridges {
        let output = Command::new("iptables")
            .args(&["-I", "DOCKER-USER", "-i", &lxd_bridge.name, "-j", "ACCEPT"])
            .output()?;

        if !output.status.success() {
            anyhow::bail!(
                "Failed to register LXD bridge {} to iptables DOCKER-USER chain: {}",
                lxd_bridge.name,
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    Ok(())
}
