import { useEffect, useState } from "react";
import { logOut, login } from "../App";
import { DomainsConfig } from "../config";
import * as autostart from "@tauri-apps/plugin-autostart";
import { useLocalStorage } from "usehooks-ts";

interface MenuProps {
  config: DomainsConfig;
  setConfig: React.Dispatch<React.SetStateAction<DomainsConfig | undefined>>;
}
export default function Menu({ config, setConfig }: MenuProps) {
  const [autoStartEnabled, setAutoStartEnabled] = useState(false);
  const [autoStartOneShot, setAutoStartOneShot] = useLocalStorage(
    "autoStartOneShot",
    false
  );

  async function initAutoStart() {
    if (!autoStartOneShot) {
      // active once
      await autostart.enable();
      setAutoStartOneShot(true);
    }
    const enabled = await autostart.isEnabled();
    console.log("enabled => ", enabled);
    setAutoStartEnabled(enabled);
  }
  useEffect(() => {
    initAutoStart();
  }, []);

  async function onAutoStartChange(status: boolean) {
    if (status) {
      await autostart.enable();
    } else {
      await autostart.disable();
    }
    setAutoStartEnabled(await autostart.isEnabled());
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
        <li>
          <div>
            <label className="form-control w-full max-w-xs">
              <span className="label-text text-sm">Autostart</span>
              <input
                type="checkbox"
                className="toggle toggle-primary"
                checked={autoStartEnabled}
                onChange={(e) => onAutoStartChange(e.target.checked)}
              />
            </label>
          </div>
        </li>
        <li>
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
        </li>

        <li onClick={login}>
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
      </ul>
    </div>
  );
}
