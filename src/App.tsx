import { useEffect, useState } from "react";
import "./App.css";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { DomainsConfig } from "./config";

function App() {
  const [config, setConfig] = useState<DomainsConfig>()

  async function loadData() {
    // await invoke('clear')
    const config = await invoke('get_config')
    console.log('config => ', config)
    if (config) {
      setConfig(config as DomainsConfig)
    }
  }

  useEffect(() => {
    loadData()
    listen('success_auth', () => {
      console.log('success auth')
      loadData()
    })
  }, [])

  function login() {
    invoke('login')
  }
  async function logOut() {
    await invoke('logout')
    location.reload()
  }

  if (!config) {
    return (
      <div>
        <button onClick={login} className="btn btn-primary">Login</button>
      </div>
    )
  }

  return (
    <div className="app">
      <button onClick={logOut} className="btn btn-primary">Logout</button>
      <h1>Hello {config.email}</h1>
      <h4>Token: {config.token}</h4>
      <input type="number" value={config.interval_minutes} />
      {config.domains.map(domain => (
        <div key={domain.name}>
          <input type="checkbox" checked={domain.enable} />
          <h1>{domain.name}</h1>
        </div>
      ))}
    </div>
  );
}

export default App;
