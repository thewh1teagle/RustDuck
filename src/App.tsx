import { useEffect, useState } from "react";
import "./App.css";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { DomainsConfig } from "./config";
import Menu from "./components/Menu";

export function login() {
  invoke("login");
}
export async function logOut() {
  await invoke("logout");
  location.reload();
}

export async function update(config: DomainsConfig) {
  await invoke("set_config", { config });
}

function App() {
  const [config, setConfig] = useState<DomainsConfig>();
  const [updating, setUpdating] = useState(false);
  useEffect(() => {
    if (config) {
      update(config);
    }
  }, [config]);

  async function loadData() {
    // await invoke('clear')
    const config = await invoke("get_config");
    console.log("config => ", config);
    if (config) {
      setConfig(config as DomainsConfig);
    }
  }

  useEffect(() => {
    loadData();
    listen("success_auth", () => {
      console.log("success auth");
      loadData();
    });
  }, []);

  if (!config) {
    return (
      <div className="w-[100vw] h-[100vh] flex flex-col items-center">
        <h1 className="text-3xl mt-10">RustDuck</h1>
        <button onClick={login} className="btn btn-primary btn-lg mt-36">
          Log in to DuckDNS
        </button>
      </div>
    );
  }

  async function updateDomains() {
    setUpdating(true);
    await invoke("update_domains");
    setUpdating(false);
  }

  return (
    <div className="flex flex-col items-center">
      <h1 className="text-3xl mt-5">RustDuck</h1>

      <div className="flex flex-col gap-3 w-[300px]">
        <div className="w-full self-start justify-self-start">
          <Menu config={config} setConfig={setConfig} />
        </div>

        <button onClick={updateDomains} className="btn btn-primary">
          {updating ? (
            <span className="loading loading-spinner loading-sm"></span>
          ) : (
            <>Update Now ({config.domains.filter((d) => d.enable).length})</>
          )}
        </button>
      </div>
      <div className="flex flex-col mt-3 gap-2 w-[300px]">
        {config.domains.map((domain) => (
          <div
            key={domain.name}
            className="shadow-lg flex flex-row justify-between  p-2 bg-neutral rounded-lg w-full"
          >
            <div className="flex flex-col">
              <div className="text-lg">{domain.name}</div>
              <div className="text-xs">{domain.name}.duckdns.org</div>
            </div>
            <input
              type="checkbox"
              className="toggle toggle-primary"
              defaultChecked={domain.enable}
              onChange={(e) => {
                console.log("change");
                const newConfig = { ...config };
                const index = config.domains.findIndex(
                  (d) => d.name == domain.name
                );
                newConfig.domains[index].enable = e.target.checked;
                setConfig(newConfig);
              }}
            />
          </div>
        ))}
      </div>
    </div>
  );
}

export default App;
