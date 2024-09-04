import { v4 } from "uuid";
import { TAlert, TATIS, vATIS } from "./types";
import { error, warn } from "@tauri-apps/plugin-log";

const find_number_of_occurances = (
  str: string,
  char: string,
  n: number
): number => {
  return (
    str.split("").reduce((acc: number[], cur, index) => {
      if (cur === char) {
        acc.push(index);
      }
      return acc;
    }, [])[n - 1] || -1
  );
};

const create_template = (
  facility: string,
  combined: boolean,
  type?: "ARR" | "DEP"
): string => {
  if (combined) {
    return "[FACILITY] ATIS INFO [ATIS_CODE] [OBS_TIME]. [FULL_WX_STRING]. [ARPT_COND] [NOTAMS]";
  } else {
    return `${facility} ${type} INFO [ATIS_CODE] [OBS_TIME]. [FULL_WX_STRING]. [ARPT_COND] [NOTAMS]`;
  }
};

const notam_varients = [
  "NOTAMS...",
  "NOTICE TO AIR MISSIONS.",
  "NOTAM.",
  "NOTICE TO AIR MISSION.",
  "NOTICES TO AIRMEN.",
];

const parse_atis = (atis: TATIS, split: boolean, facility: string): vATIS => {
  // need to add ability to parse atis without NOTAM keyword

  const vATIS = {
    atis_type: atis.type as "arr" | "dep" | "combined",
    atis_code: atis.code,
    atis: {
      id: v4(),
      name: "REAL WORLD",
      airportConditions: "",
      notams: "",
      template: create_template(
        facility.slice(1),
        split,
        split ? (atis.type.toUpperCase() as "ARR" | "DEP") : undefined
      ),
      externalGenerator: {
        enabled: false,
      },
    },
  };

  if (!notam_varients.some((varient) => atis.datis.includes(varient))) {
    let message = `No NOTAM keyword found in the ${atis.airport} ATIS, unable to parse.`;
    vATIS.atis.airportConditions = atis.datis
      .slice(find_number_of_occurances(atis.datis, ".", 2) + 1)
      .split(" ...ADVS")[0]
      .trim();
    warn(message);
    throw {
      alert_type: "warn",
      message,
      payload: vATIS,
    };
  } else {
    const notam_varient =
      notam_varients.find((varient) => atis.datis.includes(varient)) ||
      "NOTAMS...";

    vATIS.atis.notams = atis.datis.split(notam_varient)[1].split(" ...ADVS")[0];
    vATIS.atis.airportConditions = atis.datis
      .slice(find_number_of_occurances(atis.datis, ".", 2) + 1)
      .split(notam_varient)[0]
      .trim();
  }
  return vATIS;
};

export const fetch_atis = async (facility: string) => {
  const res = await fetch(`https://datis.clowd.io/api/${facility}`);
  if (!res.ok) {
    error(
      `Error fetching ATIS data. Status Code: ${res.status}, Status: ${res.statusText}`
    );
    throw {
      alert_type: "error",
      message: `An error occurred while fetching the ATIS data. Status Code: ${res.status}`,
    };
  }

  const contentType = res.headers.get("Content-Type");
  if (contentType && contentType.includes("application/json")) {
    const response = await res.json();

    let split = false;
    if (response.length > 1) {
      split = true;
    }

    const atisArray: vATIS[] = [];
    response.forEach((atis: TATIS) => {
      let parsed: vATIS;
      try {
        parsed = parse_atis(atis, split, facility);
      } catch (e) {
        const alert = e as TAlert;
        if (alert.payload) {
          parsed = alert.payload;
          atisArray.push(parsed);
          throw alert;
        } else {
          throw e;
        }
      }

      atisArray.push(parsed);
    });

    return atisArray;
  } else {
    warn("Response was not JSON.");
    throw {
      alert_type: "error",
      message: "An error occurred while fetching the ATIS data.",
    };
  }
};
