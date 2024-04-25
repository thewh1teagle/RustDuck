import { useEffect, useState } from "react";
import { logOut, login } from "../App";
import { DomainsConfig } from "../config";
import * as autostart from "@tauri-apps/plugin-autostart";
import { useLocalStorage } from "usehooks-ts";
import { invoke } from "@tauri-apps/api/core";
import * as shell from "@tauri-apps/plugin-shell";

interface MenuProps {
  config: DomainsConfig;
  setConfig: React.Dispatch<React.SetStateAction<DomainsConfig | undefined>>;
}
export default function Menu({ config, setConfig }: MenuProps) {
  const [appInfo, setAppInfo] = useState<{ version: string; commit: string }>();
  const [autoStartEnabled, setAutoStartEnabled] = useState(false);
  const [autoStartOneShot, setAutoStartOneShot] = useLocalStorage(
    "autoStartOneShot",
    false
  );

  async function init() {
    // setup autostart
    if (!autoStartOneShot) {
      // active once
      await autostart.enable();
      setAutoStartOneShot(true);
    }
    const enabled = await autostart.isEnabled();
    console.log("enabled => ", enabled);
    setAutoStartEnabled(enabled);

    // load app info
    const info = await invoke("app_info");
    if (info) {
      setAppInfo(info as any);
    }
  }
  useEffect(() => {
    init();
  }, []);

  async function onAutoStartChange(status: boolean) {
    if (status) {
      await autostart.enable();
    } else {
      await autostart.disable();
    }
    setAutoStartEnabled(await autostart.isEnabled());
  }

  function report() {
    const title = encodeURIComponent("[Bug]: ");
    const body = encodeURIComponent(
      `\n\n\n\n\`\`\`console\nVersion: ${appInfo?.version}\nCommit: ${appInfo?.commit}\n\`\`\``
    );
    const url = `https://github.com/thewh1teagle/RustDuck/issues/new?title=${title}&body=${body}`;
    shell.open(url);
  }

  return (
    <div className="dropdown dropdown-bottom">
      <div tabIndex={0} role="button" className="">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          strokeWidth={1.5}
          stroke="currentColor"
          className="w-6 h-6"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M12 6.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5ZM12 12.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5ZM12 18.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5Z"
          />
        </svg>
      </div>
      <ul
        tabIndex={0}
        className="dropdown-content z-[1] menu p-2 gap-2 shadow bg-base-100 rounded-box w-52"
      >
        <label className="form-control w-full max-w-xs">
          <span className="label-text text-sm mb-2">Autostart</span>
          <input
            type="checkbox"
            className="toggle toggle-primary"
            checked={autoStartEnabled}
            onChange={(e) => onAutoStartChange(e.target.checked)}
          />
        </label>

        <div>
          <label className="form-control w-full max-w-xs">
            <span className="label-text text-sm">Interval (minutes)</span>
            <input
              type="number"
              className="input input-sm input-bordered w-full mt-2"
              value={config.interval_minutes}
              onChange={(e) => {
                const newValue = parseInt(e.target.value);
                if (newValue) {
                  setConfig({ ...config, interval_minutes: newValue });
                }
              }}
            />
          </label>
        </div>

        <li className="mt-9" onClick={login}>
          <a className="btn btn-primary btn-sm">
            Add Domain
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              strokeWidth={1.5}
              stroke="currentColor"
              className="w-4 h-4"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                d="M13.19 8.688a4.5 4.5 0 0 1 1.242 7.244l-4.5 4.5a4.5 4.5 0 0 1-6.364-6.364l1.757-1.757m13.35-.622 1.757-1.757a4.5 4.5 0 0 0-6.364-6.364l-4.5 4.5a4.5 4.5 0 0 0 1.242 7.244"
              />
            </svg>
          </a>
        </li>
        <li>
          <a className="btn btn-error btn-sm" onClick={logOut}>
            Logout
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              strokeWidth={1.5}
              stroke="currentColor"
              className="w-4 h-4"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                d="m8.25 4.5 7.5 7.5-7.5 7.5"
              />
            </svg>
          </a>
        </li>
        <li className="">
          <a className="btn btn-neutral btn-sm" onClick={report}>
            Report issue
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              width="1em"
              height="1em"
              fill="currentColor"
              className="w-4 h-4 text-white opacity-100 duration-300 hover:opacity-50"
            >
              <path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"></path>
            </svg>
          </a>
        </li>
        <span className="text-xs text-center mb-1 text-neutral-content">
          RustDuck {appInfo?.version}
        </span>
      </ul>
    </div>
  );
}
