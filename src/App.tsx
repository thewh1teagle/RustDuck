import { useEffect, useState } from "react";
import "./App.css";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { DomainsConfig } from "./config";
import Menu from "./components/Menu";
import logo from './assets/logo.png'
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
    await invoke("show_window", { label: "main" });
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
      <div className="w-full h-full flex justify-center items-center p-10">
        <div className="bg-transparent border-primary border-2 p-6 rounded-xl shadow-lg">
          <h1 className="text-3xl font-bold mb-4 text-center opacity-60">Welcome to RustDuck</h1>
          <p className="text-md text-center opacity-60">Domains will be automatically fetched once you sign in.</p>
          <div className="flex justify-center mt-6">
            <button onClick={login} className="btn btn-md btn-primary font-bold">
              <img className="w-7 h-7 mr-2" src={logo} alt="DuckDNS Logo" />
              Sign in with DuckDNS
            </button>
          </div>
        </div>
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
            className="shadow-lg flex flex-row justify-between  p-2 bg-base-300 rounded-lg w-full"
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
