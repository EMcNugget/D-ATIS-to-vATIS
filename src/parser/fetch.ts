import { error } from "@tauri-apps/plugin-log";
import { TAirportData, TATIS, vATIS } from "../lib/types";
import { invoke } from "@tauri-apps/api/core";
import { parse_atis } from "./parser";
import { AIRPORT_DB_KEY, AIRPORT_DB_URL } from "../lib/consts";

const fetch_helper = async <T>(
  url: string,
  func: (data: Response) => Promise<T>
) => {
  return await fetch(url).then(async (res) => {
    if (!res.ok) {
      const e = `Failed to fetch data: ${res.statusText} (${res.status})`;
      error(e);
      throw {
        alert_type: "error",
        message: e,
      };
    } else if (!res.headers.get("Content-Type")?.includes("application/json")) {
      const e = `Failed to fetch data: Response was not JSON. (${url})`;
      error(e);
      throw {
        alert_type: "error",
        message: e,
      };
    } else {
      return await func(res);
    }
  });
};

export const fetch_airport = async (facility: string) => {
  const url = `${AIRPORT_DB_URL}${facility}?apiToken=${AIRPORT_DB_KEY}`;
  return await fetch_helper<TAirportData>(url, async (res) => {
    const response = await res.json();
    return response;
  });
};

export const fetch_atis = async (facility: string) => {
  const url = `https://datis.clowd.io/api/${facility}`;

  return await fetch_helper<vATIS[]>(url, async (res) => {
    const response = await res.json();

    let split = false;
    if (response.length > 1) {
      split = true;
    }

    const atis_arr: vATIS[] = [];
    response.forEach(async (v: TATIS) => {
      const custom_template = await invoke<string | undefined>(
        "get_facility_config",
        { facility: facility }
      );
      atis_arr.push(parse_atis(v, split, facility, custom_template));
    });
    return atis_arr;
  });
};
